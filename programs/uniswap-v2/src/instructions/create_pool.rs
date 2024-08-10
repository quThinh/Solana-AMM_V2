use crate::error::ErrorCode;
use crate::state::Config;
use crate::state::Pool;
use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;
#[derive(Accounts)]
pub struct CreatePool<'info> {
    pub mint0: Account<'info, Mint>,
    pub mint1: Account<'info, Mint>,

    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(mut, seeds = [b"config"], bump, has_one = owner)]
    pub config: Account<'info, Config>,

    #[account(init, payer = owner, seeds = [b"pool", mint0.key().as_ref(), mint1.key().as_ref()], bump, space = 8 + Pool::INIT_SPACE, constraint = mint0.key() < mint1.key() @ ErrorCode::InvalidMint)]
    pub pool: Box<Account<'info, Pool>>,

    /// CHECK: This is the pool authority account
    #[account(seeds = [b"authority", mint0.key().as_ref(), mint1.key().as_ref()], bump)]
    pub pool_authority: AccountInfo<'info>,

    #[account(seeds = [b"lp_mint", mint0.key().as_ref(), mint1.key().as_ref()], bump, mint::decimals = 6, mint::authority = pool_authority)]
    pub lp_mint: Box<Account<'info, Mint>>,

    #[account(
        associated_token::mint = mint1,
        associated_token::authority = pool_authority,
    )]
    pub vault1: Box<Account<'info, TokenAccount>>,

    #[account(
        associated_token::mint = mint0,
        associated_token::authority = pool_authority,
    )]
    pub vault0: Box<Account<'info, TokenAccount>>,

    pub assosciated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn create_pool(ctx: Context<CreatePool>) -> Result<()> {
    let pool = &mut ctx.accounts.pool;
    Ok(pool.initialize(ctx.accounts.mint0.key(), ctx.accounts.mint1.key())?)
}