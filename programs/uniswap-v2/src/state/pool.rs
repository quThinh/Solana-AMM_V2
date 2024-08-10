use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)] // automatically calculate the space required for the account
pub struct Pool {
    pub token0: Pubkey,
    pub token1: Pubkey,
    pub k_last: u128,
}

impl Pool {
    pub fn initialize(&mut self, token0: Pubkey, token1: Pubkey) -> Result<()> {
        self.token0 = token0;
        self.token1 = token1;
        self.k_last = 0;

        Ok(())
    }

    pub fn set_k_last(&mut self, reserve0: u64, reserve1: u64) -> Result<()> {
        self.k_last = reserve0 as u128 * reserve1 as u128;
        Ok(())
    }
}