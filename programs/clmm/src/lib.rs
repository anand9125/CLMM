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
        ctx.accounts.new(tick_spacing, initial_sqrt_price)?;
        Ok(())
    }

   
}

