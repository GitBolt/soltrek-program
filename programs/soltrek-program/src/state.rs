use anchor_lang::prelude::*;

pub const MAX_LEN_URI: usize = 128;

#[account]
pub struct User {
    pub playground_count: u128,
    pub authority: Pubkey,
}

impl User {
    pub const LEN: usize = 8 + 16 + 32;
}

#[account]
pub struct Playground {
    pub data_uri: String,
    pub created_at: u64,
    pub authority: Pubkey,
    pub number: String,
}

impl Playground {
    pub const LEN: usize = 8 + MAX_LEN_URI + 8 + 32 + 16;
}
