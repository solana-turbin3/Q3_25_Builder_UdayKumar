use anchor_lang::prelude::*;

declare_id!("4vrVwqf5txbavZ5viVRbTsPS8rTB986v3PWBtwbnLrXE");

#[program]
pub mod vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
