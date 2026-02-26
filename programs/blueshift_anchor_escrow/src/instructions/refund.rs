use anchor_lang::prelude::*;

use crate::state::Escrow;

#[derive(Accounts)]
pub struct Refund<'info> {
    pub maker: Signer<'info>,

    #[account(
        mut,
        close = maker,
        seeds = [b"escrow", escrow.maker.as_ref(), escrow.seed.to_le_bytes().as_ref()],
        bump = escrow.bump,
        constraint = escrow.maker == maker.key() @ crate::errors::EscrowError::InvalidMaker
    )]
    pub escrow: Account<'info, Escrow>,
}

pub fn handler(_ctx: Context<Refund>) -> Result<()> {
    // Refund logic: escrow is closed, lamports go back to maker
    Ok(())
}
