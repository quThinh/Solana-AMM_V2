use anchor_lang::prelude::*;
use anchor_spl::token::{Mint};
use crate::state::Config;

#[derive(Accounts)]
pub struct CreatePool<'info> {
    pub mint0: Account<'info, Mint>,
    pub mint1: Account<'info, Mint>,

    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(mut, seeds = [b"config"], bump, has_one = owner)]
    pub config: Account<'info, Config>,


    
}