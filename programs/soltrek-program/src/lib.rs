use anchor_lang::prelude::*;
pub mod instructions;
pub mod state;


use instructions::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");


#[program]
pub mod soltrek_program {
    use super::*;

    pub fn create_user(ctx: Context<InitUserAccount>) -> Result<()>{
        create_user::handler(ctx)
    }

    pub fn create_playground(ctx: Context<InitPlaygroundAccount>, data: String) -> Result<()>{
        create_playground::handler(ctx, data)
    }

}
