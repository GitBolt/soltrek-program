use anchor_lang::prelude::*;


#[account]
pub struct User {
    pub playground_count: u128,
    pub authority: Pubkey,
    pub bump: u8,
}
impl User {
    pub const LEN: usize = 8 + 8 + 32 + 1;
}


#[account]
pub struct Playground {
    pub data: String,
    pub created_at: u64,
    pub authority: Pubkey,
    pub number: u128,
}


impl Playground {
    pub const LEN: usize = 8 + 8 + 32 + 16;
}
