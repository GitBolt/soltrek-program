use anchor_lang::prelude::*;
use crate::state::*;

#[derive(Accounts)]
#[instruction(bounty_number: u128)]
pub struct InitUserAccount<'info> {

    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        seeds = [authority.key().as_ref(), ], 
        bump, 
        space=User::LEN
    )]
    pub user_account: Account<'info, User>,
    pub system_program: Program<'info, System>,
}


pub fn handler(ctx: Context<InitUserAccount>) -> Result<()> {
    let user_account = &mut ctx.accounts.user_account;
    user_account.authority = ctx.accounts.authority.key();
    user_account.bump = *ctx.bumps.get("user_account").unwrap();
    user_account.playground_count = 0;
    msg!("User account initialized");
    Ok(())
}