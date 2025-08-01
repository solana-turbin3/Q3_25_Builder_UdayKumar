use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::Associated_token,
    token_interface::{Mint, TokenAccount, TokenInterface,Transfer_checked}
};
use crate::Escrow;
#[derive(Accounts)]

pub struct Initialize{}

pub struct Make<'info>{
    #[accounts(mut)]
    pub maker: Signer<'info>,
    #[account(
        mint::token_program = token_program
    )]
    pub mint_a: InterfaceAccount<'info, Mint>,
    #[account(
        mint::token_program = token_program
    )]
    pub mint_b: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = maker,
        associated_token::token_program = token_program
    )]
    pub maker_ata_a: InterfaceAccount<'info, TokenAccount>,
    #[accounts(
        init,
        payer = maker,
        seeds = [b"escrow",maker.key().as_ref(), seed.to_le_bytes().as_ref()],
        space = 8 + Escrow::INIT_SPACE,
        bump
    )]
    pub escrow:Account<'info,Escrow>,

    #[account(
        init,
        payer,
        associated_token::mint = mint_a,
        associated_token::token_program = token_program,
    )]
    pub vault: InterfaceAccount<'info,TokenAccount>,
    pub Token_program: InterfaceAccount<'info, TokenInterface>
}

impl<'info>Make<'info>{
    pub fn init_escrow(&mut self, seed:u64, recieve: u64, bumps: &MakeBumps) ->Result<()>{
        self.escrow.set_inner(
            Escrow{ 
                seed:(), 
                maker: self.maker.key(), 
                mint_a: self.mint_a.key(), 
                mint_b: self.mint_b.key(),
                recieve,
                bump: bumps.escrow
        });
        Ok(())
    }
    pub fn deposit(&mut self, deposit:u64)->Result<()>{
        let transfer_accounts: Transfer_checked<'_> = Transfer_checked{
            from: self.maker_ata_a.to_account_info(),
            mint: self.mint_a.to_account_info(),
            to: self.vault.to_account_info(),
            authority: self.maker.to_account_info()
        };
        let cpi_ctx: CpiContext= CpiContext::new(program: self.token_program.to_account_info(), transfer_accounts);
        transfer_checked(cpi_ctx, amount: deposit, self.mint_a.decimals)
    }
}