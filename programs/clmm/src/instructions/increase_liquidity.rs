use anchor_lang::prelude::*;
use anchor_spl::{ token::{self,  Transfer}, token_interface::{Mint, TokenAccount, TokenInterface}};

use crate::{states::{Pool, Position, TickArray}, utils::{get_amount_for_liquidity, get_sqrt_price_from_tick}};
use crate::utils::ErrorCode;

#[derive(Accounts)]
pub struct IncreaseLiquidity<'info>{
    #[account(mut)]
    pub payer : Signer<'info>,
    #[account(
        mut,
        has_one = token_mint_0,
        has_one = token_mint_1
    )]
    pub pool : Account<'info,Pool>,

    #[account(mut)]
    pub lower_tick_array : Account<'info,TickArray>,
    #[account(mut)]
    pub uppar_tick_array : Account<'info,TickArray>,

    #[account(
        constraint = position.pool == pool.key() @ErrorCode::InvalidRange,
        constraint = position.owner == payer.key() @ErrorCode::Unauthorized
    )]
    pub position : Account<'info,Position>,

    #[account(
        mut,
        token::mint = token_mint_0
    )]
    pub user_token_0 : InterfaceAccount<'info,TokenAccount>,

    #[account(
        mut,
        token::mint = token_mint_1
    )]
    pub user_token_1 : InterfaceAccount<'info,TokenAccount>,

    #[account(
        mut,
        token::mint = token_mint_0
    )]
    pub pool_token_0 : InterfaceAccount<'info,TokenAccount>,

    #[account(
        mut,
        token::mint = token_mint_1
    )]
    pub pool_token_1 : InterfaceAccount<'info,TokenAccount>,

    pub token_mint_0 : InterfaceAccount<'info,Mint>,
    pub token_mint_1 : InterfaceAccount<'info,Mint>,
    pub system_program : Program<'info,System>,
    pub token_program : Interface<'info,TokenInterface>
}

impl <'info> IncreaseLiquidity<'info>{
    pub fn new(
        &mut self,
        liquidity_amount : u128,
        lower_tick : i32,
        uppar_tick : i32
    )->Result<(u64,u64)>{
        let pool = &mut self.pool;
        let position = &mut self.position;

        require!(
            lower_tick < uppar_tick
                 && lower_tick % pool.tick_spacing == 0 
                 && uppar_tick % pool.tick_spacing == 0,
            ErrorCode::InvalidTickRange
        );
        require!(liquidity_amount > 0 , ErrorCode::InsufficentAmount);

        require!(
            pool.current_tick >= lower_tick && pool.current_tick < uppar_tick,
            ErrorCode::MintRangeMustCoverCurrentPrice
        );
        let lower_tick_array = &mut self.lower_tick_array;
        let uppar_tick_array = &mut self.uppar_tick_array;
        
        let lower_tick_info = lower_tick_array
             .get_tick_info_mutable(lower_tick, pool.tick_spacing)?;
        
        let uppar_tick_info = uppar_tick_array
            .get_tick_info_mutable(uppar_tick, pool.tick_spacing)?;

        lower_tick_info.update_liquidity(liquidity_amount as i128,true)?;

        lower_tick_info.update_liquidity(liquidity_amount as i128,false)?;
        let (amount_0, amount_1) = get_amount_for_liquidity(
            pool.sqrt_price_x96,
            get_sqrt_price_from_tick(lower_tick)?,
            get_sqrt_price_from_tick(uppar_tick)?,
            liquidity_amount,
        )?;   
        
        pool.global_liquidity = pool
            .global_liquidity
            .checked_add(liquidity_amount)
            .ok_or(ErrorCode::ArithmeticOverflow)?;

        if amount_0 > 0 {
            token::transfer(
                CpiContext::new(
                    self.token_program.to_account_info(),
                    Transfer{
                        from : self.user_token_0.to_account_info(),
                        to : self.pool_token_0.to_account_info(),
                        authority : self.payer.to_account_info()
                    }
                ),
                amount_0
            )?;
        }
        if amount_1 > 0{
            token::transfer(
                CpiContext::new(
                    self.token_program.to_account_info(),
                    Transfer{
                        from:self.user_token_1.to_account_info(),
                        to : self.pool_token_1.to_account_info(),
                        authority : self.payer.to_account_info()
                    }
                ),
                amount_1
            )?;
        } 
        Ok((amount_0,amount_1))
    }
}