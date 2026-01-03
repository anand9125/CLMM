use anchor_lang::prelude::*;
use anchor_spl::{token::Token, token_interface::{Mint, TokenAccount, TokenInterface}};
use crate::{states::Pool, utils::get_tick_at_sqrt_price};

#[derive(Accounts)]
#[instruction(tick_spacing:i32)]
pub struct InistalsiePool<'info>{
    #[account(mut)]
    pub payer: Signer<'info>,
    
    #[account(
        init,
        payer = payer,
        space = 8 + Pool::INIT_SPACE,
        seeds =[
            b"pool".as_ref(),
            token_mint_0.key().as_ref(),
            token_mint_1.key().as_ref(),
            tick_spacing.to_le_bytes().as_ref()
        ],
        bump
    )]
    pub pool : Account<'info,Pool>,
    pub token_mint_0 : InterfaceAccount<'info,Mint>,
    pub token_mint_1 : InterfaceAccount<'info,Mint>,
    #[account(
        init,
        payer = payer,
        token::mint = token_mint_0,
        token::authority = pool
    )]
    pub token_vault_0 : InterfaceAccount<'info,TokenAccount>,
    #[account(
        init,
        payer = payer,
        token::mint = token_mint_1,
        token::authority = pool
    )]
    pub token_vault_1 : InterfaceAccount<'info,TokenAccount>,
    pub system_program : Program<'info,System>,
    pub associated_token_program : Program<'info,Token>,
    pub token_program : Interface<'info,TokenInterface>
}
impl <'info> InistalsiePool<'info>{
    pub fn new(&mut self,tick_spacing:i32,inital_sqrt_price:u128)->Result<()>{
        
        let pool = &mut self.pool;
        pool.token_mint_0 = self.token_mint_0.key();
        pool.token_mint_1 = self.token_mint_1.key();
        pool.token_vault_0 = self.token_vault_0.key();
        pool.token_vault_1 = self.token_vault_1.key();
        pool.global_liquidity = 0;
        pool.sqrt_price_x96 = inital_sqrt_price;
        pool.current_tick = get_tick_at_sqrt_price(inital_sqrt_price)?;
        pool.tick_spacing = tick_spacing;
        pool.bump = pool.bump;
        Ok(())
    }
}