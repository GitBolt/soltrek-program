use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("GESymwz7zdZ7uu7QYkpxNEtgEmDhZUQntgMYbZueD5te");

#[program]
pub mod soltrek_program {
    use super::*;

    pub fn create_user(ctx: Context<InitUserAccount>) -> Result<()> {
        create_user::handler(ctx)
    }

    pub fn create_playground(
        ctx: Context<InitPlaygroundAccount>,
        playground_number: String,
        data_uri: String,
    ) -> Result<()> {
        create_playground::handler(ctx, playground_number, data_uri)
    }
}
