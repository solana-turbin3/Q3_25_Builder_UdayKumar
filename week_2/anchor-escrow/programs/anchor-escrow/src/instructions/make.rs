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