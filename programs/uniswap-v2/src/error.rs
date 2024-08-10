use anchor_lang::prelude::*;

#[error_code]

pub enum ErrorCode {
    #[msg("Invalid Fee")]
    InvalidFee,
    #[msg("Invalid Mint")]
    InvalidMint,
}