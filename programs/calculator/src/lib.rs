use anchor_lang::prelude::*;

declare_id!("GEMgAzfcdm1shi2Yx8Di15xk6MKgH9Va1MbkJXxniH7Q");

#[program]
pub mod calculator {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
