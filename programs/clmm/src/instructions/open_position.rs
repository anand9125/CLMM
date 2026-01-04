use anchor_lang::prelude::*;
use anchor_spl::{ token_interface::{Mint, TokenAccount, TokenInterface}};
use crate::states::{Pool, Position, TickArray};


#[derive(Accounts)]
#[instruction(lower_tick:i32,uppar_tick:i32,liquidity_amount:u128,tick_array_lower_start_index:i32,tick_array_uppar_start_index:i32)]
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
            payer.key().as_ref(),
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
