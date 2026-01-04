use anchor_lang::prelude::*;
pub mod states;
pub mod instructions;
pub mod utils;
pub use instructions::*;


declare_id!("6kaKTU4t5TcvmFotq62EGxs8yLd4DzxiDHhUzf1Y1Xeq");

#[program]
pub mod clmm {

    use super::*;

    pub fn initalise_pool(
        ctx:Context<InitializePool>,
        tick_spacing:i32,
        initial_sqrt_price : u128
    )->Result<()>{
        ctx.accounts.new(tick_spacing, initial_sqrt_price,ctx.bumps.pool)?;
        Ok(())
    }
    pub fn open_position(
        ctx: Context<OpenPosition>,
        owner: Pubkey,
        lower_tick : i32,
        uppar_tick : i32,
        liquidity_amount : u128,
        _tick_array_lower_start_index : i32,
        _tick_array_uppar_start_index : i32
    )->Result<()>{
        ctx.accounts.new(
            owner, 
            lower_tick, 
            uppar_tick,
            liquidity_amount, 
            _tick_array_lower_start_index, 
            _tick_array_uppar_start_index,
            ctx.bumps.position
        );
        Ok(())
    }

   
}

