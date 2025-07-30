#![allow(unexpected_cfgs)] //allows use of unsupported cfg attributes.
#![allow(deprecated)] //suppresses warnings for deprecated features.
use anchor_lang::prelude::*;
use anchor_lang::system_program::{Transfer,transfer};

declare_id!("4vrVwqf5txbavZ5viVRbTsPS8rTB986v3PWBtwbnLrXE");

#[program]
pub mod anchor_vault {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>)-> Result <()>{
        ctx.accounts.initialize(&ctx.bumps)
    }
    pub fn deposit(ctx: Context<Payment>, amount:u64)-> Result<()>{
        ctx.accounts.deposit(amount)
    } 
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account( 
        mut  //since there will be a change in user balances acc should be mut here
    )]
    pub user: Signer<'info>,
    #[account(
        init,  // This account should be created (initialized)
        payer = user, // The user pays the rent to create this account
        space = VaultState::INIT_SPACE,  // How much space to allocate
        seeds = [b"state",user.key().as_ref()], // PDA seed
        bump  // Anchor will auto-fill the bump
    )]

    pub vault_state: Account<'info, VaultState>,
    #[account(  //no init needed here system accounts gets initialized when enough lamports is provided to them
        mut, //lamports in vaultstate is getting mutated
        seeds = [b"vault",vault_state.key().as_ref()],  // Another PDA based on the vault_state
        bump
    )]
    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>
}
//deposit instruction
#[derive(Accounts)]

pub struct Payment<'info>{
    #[account(mut)]
    pub user:Signer<'info>,
    #[account(
        mut,
        seeds = [b"vault",vault_state.key().as_ref()],
        bump= vault_state.state_bump
    )]
    pub vault: SystemAccount<'info>,
    #[account(
        seeds = [b"state",user.key().as_ref()], //no need to mutate here as we are only reading the values here not writing
        bump = vault_state.state_bump
    )]
    pub vault_state: Account<'info, VaultState>,
    pub system_program: Program<'info, System>
}

impl<'info> Payment<'info>{

// deposit function
    pub fn deposit(&mut self, amount:u64)-> Result<()>{
        let  cpi_program:AccountInfo<'_>= self.system_program.to_account_info();
        let cpi_accounts: Transfer<'_> = Transfer{
            from: self.user.to_account_info(),
            to: self.vault.to_account_info()
        };
        let cpi_ctx= CpiContext::new(cpi_program, cpi_accounts);
        transfer(cpi_ctx, amount)
    }
// withdraw function
    pub fn withdraw(&mut self, amount:u64) -> Result<()>{
        let cpi_program: AccountInfo<'_> = self.system_program.to_account_info();
        let cpi_accounts:Transfer<'_> = Transfer{
            from: self.vault.to_account_info(),
            to: self.user.to_account_info()
        };
        let vault_state_key = self.vault_state.to_account_info().key();
        let seeds: &[&[u8]; 3] = &[
            b"vault",
            vault_state_key.as_ref(),
            &[self.vault_state.vault_bump]
        ];
        let signer_seeds: &[&[&[u8]]; 1] = &[&seeds[..]];
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
        transfer(cpi_ctx, amount)
    }
}

impl<'info> Initialize<'info>{
    pub fn initialize(&mut self, bumps: &InitializeBumps) -> Result<()>{
        let rent_exempt: u64 = Rent::get()?.minimum_balance(self.vault.to_account_info().data_len());
        let cpi_program: AccountInfo<'_> = self.system_program.to_account_info();
        let cpi_accounts: Transfer<'_> = Transfer{
            from: self.user.to_account_info(),
            to: self.vault.to_account_info()
        };
        let cpi_ctx= CpiContext::new(cpi_program,cpi_accounts);
        transfer(cpi_ctx, rent_exempt)?;
        self.vault_state.vault_bump = bumps.vault;
        self.vault_state.state_bump = bumps.vault_state;
        self.vault_state.owner = self.user.key();
        self.vault_state.total_deposits = 0;
        Ok(())
    }
}
// vault state is linked to user and vault is linked to vault state
#[account]
pub struct VaultState {
    pub vault_bump: u8,
    pub state_bump: u8,
    pub owner: Pubkey,
    pub total_deposits: u64
}

impl Space for VaultState {
    const INIT_SPACE: usize = 8 + 1*2 +32 +8;
}
