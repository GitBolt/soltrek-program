use crate::state::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(bounty_number: u128)]
pub struct InitPlaygroundAccount<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        has_one = authority,
        seeds=[authority.key().as_ref(), bounty_number.to_string().as_ref()],
        bump=user_account.bump
    )]
    pub user_account: Account<'info, User>,

    #[account(
        init,
        payer=authority,
        seeds=[authority.key().as_ref()],
        bump,
        space=Playground::LEN
    )]
    pub playground_account: Account<'info, Playground>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitPlaygroundAccount>, data: String) -> Result<()> {
    // if data == "" {
    //     return Err(error!(ErrorCode::UriCannotBeEmpty))
    // }

    let playground_account = &mut ctx.accounts.playground_account;
    let user_account = &mut ctx.accounts.user_account;

    playground_account.data = data;
    playground_account.authority = ctx.accounts.authority.key();
    playground_account.number = user_account.playground_count;
    playground_account.created_at = Clock::get().unwrap().unix_timestamp as u64;

    user_account.playground_count += 1;
    msg!("Playground created");
    Ok(())
}
