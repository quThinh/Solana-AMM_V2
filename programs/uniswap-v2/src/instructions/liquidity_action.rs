use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, TokenAccount, Transfer};
use crate::state::{Pool, Config};
use anchor_spl::associated_token::AssociatedToken;

#[derive(Accounts)]
pub struct AddLiquidity<'info> {
    #[account(seeds= [b"config"], bump)]
    pub config: Box<Account<'info, Config>>,

    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(has_one = owner)]
    pub user_ata0: Account<'info, TokenAccount>,

    #[account(has_one = owner)]
    pub user_ata1: Account<'info, TokenAccount>,

    #[account(init_if_needed, payer = owner, assosciated_token::mint = lp_mint, assosciated_token::authority = pool_authority)]
    pub user_lp_ata: Account<'info, TokenAccount>,

    #[account(mut)]
    pub pool: Account<'info, Pool>,

    ///CHECK: This is the pool authority account
    #[account(seeds = [b"authority", pool.mint0.key().as_ref(), pool.mint1.key().as_ref()], bump)]
    pub pool_authority: AccountInfo<'info>,

    #[account(constraint = vault1.mint == user_ata1.mint)]
    pub vault1: Account<'info, TokenAccount>,

    #[account(constraint = vault0.mint == user_ata0.mint)]
    pub vault0: Account<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, token::Token>,
    pub system_program: Program<'info, System>,
}

pub fn add_liquidity(
    ctx: Context<AddLiquidity>, 
    amount0_min: u64,
    amount1_min: u64,
    amount0_desired: u64,
    amount1_desired: u64,
) -> Result<()> {
    let pool : &mut Pool = &mut ctx.accounts.pool;
    let config : &Config = &ctx.accounts.config;

    let (reserve0, reserve1) = (ctx.accounts.vault0.amount, ctx.accounts.vault1.amount);

    Ok(())

}