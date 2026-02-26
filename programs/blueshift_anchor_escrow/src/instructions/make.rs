use anchor_lang::prelude::*;

use crate::errors::EscrowError;
use crate::state::Escrow;

#[derive(Accounts)]
#[instruction(seed: u64, _receive: u64, amount: u64)]
pub struct Make<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,

    #[account(
        init,
        payer = maker,
        space = Escrow::LEN,
        seeds = [b"escrow", maker.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump
    )]
    pub escrow: Account<'info, Escrow>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Make>, seed: u64, receive: u64, amount: u64) -> Result<()> {
    require!(amount > 0, EscrowError::InvalidAmount);
    require!(receive > 0, EscrowError::InvalidReceive);

    let escrow = &mut ctx.accounts.escrow;
    let bump = ctx.bumps.escrow;

    escrow.seed = seed;
    escrow.amount = amount;
    escrow.receive = receive;
    escrow.maker = ctx.accounts.maker.key();
    escrow.bump = bump;

    Ok(())
}
