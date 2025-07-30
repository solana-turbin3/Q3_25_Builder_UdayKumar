use anchor_lang::prelude::*;

declare_id!("CBHZMZhB2VcN3oJFEtiggGAzFSXvZKNo4xZN8CeqUs5t");

#[program]
pub mod anchor_escrow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
