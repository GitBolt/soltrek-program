use crate::state::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(playground_number: String)]
pub struct InitPlaygroundAccount<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        has_one = authority,
        seeds=[authority.key().as_ref()],
        bump
    )]
    pub user_account: Account<'info, User>,

    #[account(
        init,
        payer=authority,
        seeds=[authority.key().as_ref(), playground_number.as_ref()],
        bump,
        space=Playground::LEN
    )]
    pub playground_account: Account<'info, Playground>,

    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<InitPlaygroundAccount>,
    playground_number: String,
    data_uri: String,
) -> Result<()> {
    let playground_account = &mut ctx.accounts.playground_account;
    let user_account = &mut ctx.accounts.user_account;

    playground_account.data_uri = data_uri;
    playground_account.authority = ctx.accounts.authority.key();
    playground_account.created_at = Clock::get().unwrap().unix_timestamp as u64;

    let count = playground_number.parse::<i32>().unwrap_or(0) + 1;
    playground_account.number = count.to_string();

    user_account.playground_count += 1;
    msg!("Playground created");
    Ok(())
}
