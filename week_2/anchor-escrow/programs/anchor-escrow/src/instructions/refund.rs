use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::Associated_token,
    token_interface::{
        close_account, transfer_checked, CloseAccount, Mint, TokenAccount, TokenInterface, TransferChecked
    }
};
use crate:Escrow;

#[derive(Accounts )]

pub struct Refund<'info>{
    #[account(mut)]
    pub maker: Signer<'info>,

    #[account(
        mint::token_program = token_program
    )]
    pub mint_a: InterfaceAccount<'info, Mint>,
    #[account(
        mint::token_program= token_program
    )]
    pub mint_b: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token:authority = maker,
        associated_token::token_program = token_program
    )]
    pub maker_ata_a: InterfaceAccount<'info,TokenAccount>,
    #[account(
        init,
        payer = maker,
        seeds = [b"escrow",maker.key().as_ref(), seed.to_le_bytes().as_ref()],
        space = 8 + Escrow::INIT_SPACE,
        bump
    )]
    pub escrow: Account<'info, Escrow>,
    
}