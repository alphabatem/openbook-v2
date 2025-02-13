use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{self};

#[derive(Accounts)]
pub struct StubOracleCreate<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub owner: Signer<'info>,
    #[account(
        init,
        seeds = [b"StubOracle".as_ref(), owner.key().as_ref(), mint.key().as_ref()],
        bump,
        payer = payer,
        space = 8 + std::mem::size_of::<StubOracle>(),
    )]
    pub oracle: AccountLoader<'info, StubOracle>,
    pub mint: InterfaceAccount<'info, token_interface::Mint>,
    pub system_program: Program<'info, System>,
}
