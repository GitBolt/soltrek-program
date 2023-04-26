use anchor_lang::prelude::*;
use crate::state::*;

#[derive(Accounts)]
pub struct InitUserAccount<'info> {

    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        seeds = [authority.key().as_ref()], 
        bump, 
        space=User::LEN
    )]
    pub user_account: Account<'info, User>,
    
    pub system_program: Program<'info, System>,
}


pub fn handler(ctx: Context<InitUserAccount>) -> Result<()> {
    let user = &mut ctx.accounts.user_account;

    user.authority = ctx.accounts.authority.key();
    user.playground_count = 0;

    msg!("User account initialized");
    Ok(())
}