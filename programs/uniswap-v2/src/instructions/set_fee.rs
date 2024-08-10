use anchor_lang::prelude::*;
use anchor_spl::token::{MintTo, TokenAccount, Transfer, mint_to};
use fixed::types::U128F0;
use crate::state::{Config, Pool};
#[derive(Accounts)]
pub struct SetFeeTo<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(mut, seeds = [b"config"], bump, has_one = owner)]
    pub config: Account<'info, Config>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SetFee<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(mut, seeds = [b"config"], bump, has_one = owner)]
    pub config: Account<'info, Config>,

    pub system_program: Program<'info, System>,
}

pub fn set_fee_to(ctx: Context<SetFeeTo>, fee_to: Pubkey) -> Result<()> {
    let config = &mut ctx.accounts.config;
    Ok(config.set_fee_to(fee_to)?)
}

pub fn set_fee(ctx: Context<SetFee>, fee: u64) -> Result<()> {
    let config = &mut ctx.accounts.config;
    Ok(config.set_fee(fee)?)
}

pub fn mint_fee<'info>(
    _config: &Config,
    pool: &Pool,
    reserve0: u64,
    reserve1: u64,
    lp_supplied: u64,
    mint_ctx: CpiContext<'_, '_, '_, 'info, MintTo<'info>>,
) -> Result<()> {
    let k_last = pool.k_last;

    if k_last != 0 {
        let root_k = U128F0::from(reserve0 as u128 * reserve1 as u128).sqrt().to_num::<u128>();
        let root_k_last = U128F0::from(k_last).sqrt().to_num::<u128>();
        if root_k > root_k_last {
            let numerator : u128 = lp_supplied as u128 * (root_k - root_k_last);
            let denominator : u128 = root_k * 5 + root_k_last;
            let liquidity : u128 = numerator / denominator as u128;
            if liquidity > 0 {
                mint_to(mint_ctx, liquidity as u64)?;
            }
        }
    }

    Ok(())
}

