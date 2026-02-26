use anchor_lang::prelude::*;

use crate::state::Escrow;

#[derive(Accounts)]
pub struct Take<'info> {
    pub taker: Signer<'info>,

    #[account(
        mut,
        close = taker,
        seeds = [b"escrow", escrow.maker.as_ref(), escrow.seed.to_le_bytes().as_ref()],
        bump = escrow.bump
    )]
    pub escrow: Account<'info, Escrow>,
}

pub fn handler(_ctx: Context<Take>) -> Result<()> {
    // Transfer logic / receive tokens can be implemented here
    Ok(())
}
