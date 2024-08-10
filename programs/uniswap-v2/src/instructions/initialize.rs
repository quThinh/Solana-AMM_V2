use crate::state::Config;
use anchor_lang::prelude::*;

#[derive(Accounts)]

pub struct Initialize<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(init, payer = owner, seeds = [b"config"], bump, space = 8 + Config::INIT_SPACE)]
    pub config: Account<'info, Config>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn initialize(ctx: Context<Initialize>, fee_to: Pubkey, fee: u64) -> Result<()> {
    let config = &mut ctx.accounts.config;
    Ok(config.initialize(*ctx.accounts.owner.key, fee_to, fee)?)
}
