use anchor_lang::prelude::*;

declare_id!("6kaKTU4t5TcvmFotq62EGxs8yLd4DzxiDHhUzf1Y1Xeq");

#[program]
pub mod clmm {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
