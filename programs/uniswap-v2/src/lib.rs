use anchor_lang::prelude::*;
use instructions::*;

pub mod instructions;
pub mod state;
pub mod error;

declare_id!("942q1SpjUkHDK2vGQEdL2uTVW7eWRRJ2yTxyhYshvfDc");

#[program]
pub mod uniswap_v2 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, fee_to: Pubkey, fee: u64) -> Result<()> {
        instructions::initialize(ctx, fee_to, fee)
    }
}