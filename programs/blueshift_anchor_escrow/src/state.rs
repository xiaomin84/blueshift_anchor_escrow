use anchor_lang::prelude::*;

#[account]
pub struct Escrow {
    pub seed: u64,
    pub amount: u64,
    pub receive: u64,
    pub maker: Pubkey,
    pub bump: u8,
}

impl Escrow {
    pub const LEN: usize = 8 + 8 + 8 + 8 + 32 + 1;
}
