use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Pool{
    pub token_mint_0 : Pubkey,
    pub token_mint_1 : Pubkey,
    pub token_vault_0 : Pubkey,
    pub token_vault_1 : Pubkey,
    pub global_liquidity : u128, //Amount of liquidity currently active at the current price its not total liq its liq that is usable right now
    pub sqrt_price_x96 : u128,  //price = token1/token0  but we use in squre root casue of precision ,swap math become linear,prevent overflow
    //x96 means we use fixed point with 96 bits for fractional part (Solana has no floating points.)
    pub current_tick : i32, //current tick of the pool , tick is used to represent price in discrete steps
    pub tick_spacing : i32,
    pub bump : u8,
}