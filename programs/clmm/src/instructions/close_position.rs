use anchor_lang::prelude::*;
use anchor_spl::{ token::{self,  Transfer}, token_interface::{Mint, TokenAccount, TokenInterface}};
use crate::{states::{Pool, Position, TickArray}, utils::{get_amount_for_liquidity, get_sqrt_price_from_tick}};
use crate::utils::ErrorCode;

#[derive(Accounts)]
#[instruction(lower_tick: i32, upper_tick: i32, tick_array_lower_start_index: i32, tick_array_upper_start_index: i32)]
pub struct  ClosePosition<'info>{
    #[account(mut)]
    pub owner : Signer<'info>,
    #[account(mut)]
    pub pool : Account<'info, Pool>,
    #[account(
        mut,
        seeds = [
            b"tick_array",
            pool.key().as_ref(),
            &tick_array_lower_start_index.to_le_bytes()
        ],
        bump
    )]
    pub lower_tick_array : Account<'info,TickArray>,
    #[account(
        mut,
        seeds = [
            b"tick_array",
            pool.key().as_ref(),
            &tick_array_upper_start_index.to_le_bytes()
        ],
        bump
    )]
    pub upper_tick_array : Account<'info,TickArray>,
    #[account(
        mut,
        close = owner,
        seeds = [
            b"position",
            owner.key().as_ref(),
            pool.key().as_ref(),
            &lower_tick.to_le_bytes(),      
            &upper_tick.to_le_bytes()
        ],
        bump = position.bump
    )]
    pub position : Account<'info,Position>,
        #[account(
        mut,
        token::mint = token_mint_0
    )]
    pub user_token_0 : InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        token::mint = token_mint_1
    )]
    pub user_token_1 : InterfaceAccount<'info,TokenAccount>,
    #[account(
        mut,
        token::mint = token_mint_0
    )]
    pub pool_token_0 : InterfaceAccount<'info, TokenAccount>,
     #[account(
        mut,
        token::mint = token_mint_1
    )]
    pub pool_token_1 : InterfaceAccount<'info, TokenAccount>,

    pub token_mint_0 : InterfaceAccount<'info,Mint>,
    pub token_mint_1 : InterfaceAccount<'info,Mint>,
    pub system_program : Program<'info,System>,
    pub token_program : Interface<'info , TokenInterface>
  
}
impl <'info> ClosePosition <'info>{

    pub fn new(
        &mut self,
        lower_tick:i32,
        uppar_tick : i32, 
        _tick_arrry_lower_start_index : i32,
         _tick_array_uppar_start_index:i32
        )->Result<(u64,u64)>{
            let pool = &mut self.pool;
            let position = &mut self.position;

            let liquidity_to_remove = position.liquidity;
            require!(liquidity_to_remove > 0, ErrorCode::NoLiquidityToRemove);

             let (amount_0, amount_1) = get_amount_for_liquidity(
                pool.sqrt_price_x96,
                get_sqrt_price_from_tick(lower_tick)?,
                get_sqrt_price_from_tick(uppar_tick)?,
                liquidity_to_remove,
            )?;
            let lower_tick_array = &mut self.lower_tick_array;
            let uppar_tick_array = &mut self.upper_tick_array;

            let lower_tick_info = 
                lower_tick_array.get_tick_info_mutable(lower_tick,pool.tick_spacing)?;
            lower_tick_info.update_liquidity(-(liquidity_to_remove as i128), true)?;

            let uppar_tick_info = 
                uppar_tick_array.get_tick_info_mutable(uppar_tick,pool.tick_spacing)?;
            uppar_tick_info.update_liquidity(-(liquidity_to_remove as i128), false)?;

            pool.global_liquidity = pool
                .global_liquidity
                .checked_sub(liquidity_to_remove)
                .ok_or(ErrorCode::ArithmeticOverflow)?;

            let pool_seeds = &[
                b"pool".as_ref(),
                pool.token_mint_0.as_ref(),
                pool.token_mint_1.as_ref(),
                &[pool.bump],
            ];
            let signer_seeds = &[&pool_seeds[..]];

            if amount_0>0 {
                token::transfer(
                    CpiContext::new_with_signer(
                        self.token_program.to_account_info(),
                        Transfer { 
                            from : self.pool_token_0.to_account_info(),
                            to : self.user_token_0.to_account_info(),
                            authority : pool.to_account_info()
                         },
                         signer_seeds
                    ),
                    amount_0
                )?;
            };
            if amount_1 > 1 {
                token::transfer(
                    CpiContext::new_with_signer(
                        self.token_program.to_account_info(),
                        Transfer{
                            from : self.pool_token_1.to_account_info(),
                            to : self.user_token_1.to_account_info(),
                            authority : pool.to_account_info()
                        },
                        signer_seeds
                    ),
                    amount_1
                )?;
            };
            Ok((amount_0,amount_1))

        }
}