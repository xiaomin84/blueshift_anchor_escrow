use anchor_lang::prelude::*;

#[error_code]
pub enum EscrowError {
    #[msg("Invalid amount")]
    InvalidAmount,

    #[msg("Invalid receive amount")]
    InvalidReceive,

    #[msg("Escrow not found or expired")]
    EscrowNotFound,

    #[msg("Unauthorized")]
    Unauthorized,
}
