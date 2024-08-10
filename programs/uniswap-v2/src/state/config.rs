use anchor_lang::prelude::*;
use crate::error::ErrorCode;

#[account]
#[derive(InitSpace)] // automatically calculate the space required for the account
pub struct Config {
    pub owner: Pubkey,
    pub fee_to: Pubkey,
    pub fee: u64,
}

impl Config {
    pub fn initialize(&mut self, owner: Pubkey, fee_to: Pubkey, fee: u64) -> Result<()> {
        self.owner = owner;
        self.fee_to = fee_to;
        self.fee = fee;

        Ok(())
    }

    pub fn set_fee_to(&mut self, fee_to: Pubkey) -> Result<()> {
        self.fee_to = fee_to;
        Ok(())
    }

    pub fn set_fee(&mut self, fee: u64) -> Result<()> {
        require!(fee < 10000, ErrorCode::InvalidFee);

        self.fee = fee;
        Ok(())
    }
}