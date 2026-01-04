use anchor_lang::prelude::*;
use anchor_spl::{ token::{self, Transfer}, token_interface::{Mint, TokenAccount, TokenInterface}};
use crate::{states::{Pool, Position, TickArray}, utils::{get_amount_for_liquidity, get_sqrt_price_from_tick}};
use crate::utils::ErrorCode;


#[derive(Accounts)]
#[instruction(owner:Pubkey,lower_tick:i32,uppar_tick:i32,liquidity_amount:u128,tick_array_lower_start_index:i32,tick_array_uppar_start_index:i32)]
pub struct OpenPosition<'info>{
    #[account(mut)]
    pub payer : Signer<'info>,
    #[account(
        mut,
       // You are only verifying the PDA, not creating it,this allowed
        seeds = [
            b"pool",
            pool.token_mint_0.as_ref(),
            pool.token_mint_1.as_ref(),
            pool.tick_spacing.to_le_bytes().as_ref(),
        ],
        bump = pool.bump
    )]
    pub pool: Account<'info, Pool>,  

    #[account(
        init_if_needed,
        payer = payer,
        space = TickArray::SPACE,
        seeds = [
            b"tick_array",
            pool.key().as_ref(),
            &tick_array_lower_start_index.to_le_bytes()
        ],
        bump
    )]
    pub lower_tick_array : Account<'info,TickArray>,

    #[account(
        init_if_needed,
        payer = payer ,
        space = TickArray::SPACE,
        seeds = [
            b"tick_array",
            pool.key().as_ref(),
            &tick_array_uppar_start_index.to_le_bytes()
        ],
        bump
    )]
    pub uppar_tick_array : Account<'info,TickArray>,

    #[account(
        init_if_needed,
        payer = payer,
        space = Position::INIT_SPACE,
        seeds = [
            b"position",
            owner.as_ref(),
            pool.key().as_ref(),
            &lower_tick.to_le_bytes(),
            &uppar_tick.to_le_bytes()
        ],
        bump
    )]
    pub position : Account<'info, Position>,
    
    pub token_mint_0 : InterfaceAccount<'info,Mint>,
    pub token_mint_1 : InterfaceAccount<'info,Mint>,

    #[account(mut)]
    pub user_token_0: InterfaceAccount<'info, TokenAccount>,
    #[account(mut)]
    pub user_token_1 : InterfaceAccount<'info,TokenAccount>,
    #[account(mut)]
    pub pool_token_0 : InterfaceAccount<'info,TokenAccount>,
    #[account(mut)]
    pub pool_token_1 : InterfaceAccount<'info,TokenAccount>,
    pub system_program : Program<'info,System>,
    pub token_program : Interface<'info,TokenInterface>,

}

impl <'info>OpenPosition<'info>{
    pub fn new(
        &mut self,
        owner: Pubkey,
        lower_tick : i32,
        uppar_tick : i32,
        liquidity_amount : u128,
        _tick_array_lower_start_index : i32,
        _tick_array_uppar_start_index:i32,
        bump:u8   
    )->Result<(u64,u64)>{
        
        require!(
            self.user_token_0.mint == self.token_mint_0.key(),
            ErrorCode::InvalidMint
        );
        require!(
            self.user_token_1.mint == self.token_mint_1.key(),
            ErrorCode::InvalidMint
        );
        require!(
            self.user_token_0.owner == owner,
            ErrorCode::Unauthorized
        );
        require!(
            self.user_token_1.owner == owner,
            ErrorCode::Unauthorized
        );
        let pool = &mut self.pool;

        require!(
            lower_tick < uppar_tick
                && lower_tick % pool.tick_spacing == 0
                && uppar_tick % pool.tick_spacing == 0,
            ErrorCode::InvalidTickRange
        );
        let position = &mut self.position;

        require!(liquidity_amount > 0 ,ErrorCode::InsufficentAmount);

        let lower_tick_array = &mut self.lower_tick_array;
        let uppar_tick_array = &mut self.uppar_tick_array;

        if lower_tick_array.starting_tick == 0 && lower_tick_array.pool ==Pubkey::default(){
            lower_tick_array.pool = pool.key();
            lower_tick_array.starting_tick = _tick_array_lower_start_index;
        }

        if uppar_tick_array.starting_tick == 0  && uppar_tick_array.pool == Pubkey::default() {
            uppar_tick_array.pool = pool.key();
            uppar_tick_array.starting_tick = _tick_array_uppar_start_index;
        }

        let lower_tick_info = lower_tick_array
            .get_tick_info_mutable(lower_tick, pool.tick_spacing)?;

        let uppar_tick_info  = uppar_tick_array
            .get_tick_info_mutable(uppar_tick, pool.tick_spacing)?;

        lower_tick_info.update_liquidity(liquidity_amount as i128, true)?;
        uppar_tick_info.update_liquidity(liquidity_amount as i128, false)?;

        let (amount_0,ampunt_1) = get_amount_for_liquidity(
            pool.sqrt_price_x96,
            get_sqrt_price_from_tick(lower_tick)?,
            get_sqrt_price_from_tick(uppar_tick)?,
            liquidity_amount
        )?;

        if position.liquidity == 0 && position.owner == Pubkey::default() {
            position.owner = owner;
            position.pool = pool.key();
            position.tick_uppar = uppar_tick;
            position.tick_lower = lower_tick;
            position.liquidity = liquidity_amount;
            position.bump = bump;
        }else{
            require!(position.owner == owner , ErrorCode::Unauthorized);
            require!(
                position.tick_lower == lower_tick && position.tick_uppar == uppar_tick,
                ErrorCode::InvalidPositionRange
            );
            position.liquidity = position
                .liquidity
                .checked_add(liquidity_amount)
                .ok_or(ErrorCode::ArithmeticOverflow)?;
        }

        //TODO: liq should  added global liq only when current price inside the range
        pool.global_liquidity = pool
            .global_liquidity
            .checked_add(liquidity_amount)
            .ok_or(ErrorCode::ArithmeticOverflow)?;

        if amount_0 > 0 {
            token::transfer(
                CpiContext::new(
                    self.token_program.to_account_info(),
                    Transfer{
                        from:self.user_token_0.to_account_info(),
                        to:self.pool_token_0.to_account_info(),
                        authority : self.payer.to_account_info()
                    },
                ),
                amount_0
            )?;
        }
        if ampunt_1 > 0 {
            token::transfer(
                CpiContext::new(
                    self.token_program.to_account_info(),
                    Transfer{
                        from : self.user_token_1.to_account_info(),
                        to : self.pool_token_1.to_account_info(),
                        authority : self.payer.to_account_info()
                    }
                ),
                ampunt_1
            )?;
        }
        Ok((amount_0,ampunt_1))
    }
}