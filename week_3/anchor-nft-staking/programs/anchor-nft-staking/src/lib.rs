use anchor_lang::prelude::*;

declare_id!("CLumJ1r87JCJGh5SqURotjZX2eHvQeJd2LifFwwA158k");

#[program]
pub mod anchor_nft_staking {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
