use crate::state::Config;
use anchor_lang::{prelude::*, solana_program::vote::instruction};
use crate::error::ErrorCode;
#[derive(Accounts)]
#[instruction(fee: u64)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(init, payer = owner, seeds = [b"config"], bump, space = 8 + Config::INIT_SPACE, constraint = fee < 10000 @ ErrorCode::InvalidFee)]
    pub config: Account<'info, Config>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn initialize(ctx: Context<Initialize>, fee_to: Pubkey, fee: u64) -> Result<()> {
    let config = &mut ctx.accounts.config;
    Ok(config.initialize(*ctx.accounts.owner.key, fee_to, fee)?)
}
