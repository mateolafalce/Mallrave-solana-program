use anchor_lang::prelude::*;

#[account]
pub struct IterData {
    pub bump_original: u8,             // 1
    pub iter: u64,                     // 8
}

#[account]
pub struct AccountData {
    pub bump_original: u8,              // 1
    pub transactions: u64,              // 8
    pub average_exchange_time: i64,     // 8
    pub positioning: Vec<Pubkey>,        // 4 + 9971 (311 products)
}

#[account]
pub struct Product {
    pub bump_original: u8,          // 1
    pub pubkey: Pubkey,             // 32
    pub seed: u64,                  // 8
    pub active: bool,               // 1
    pub product: String,            // 4 + 32
    pub description: String,        // 4 + 200
    pub supply: u32,                // 4
    pub price: u64,                 // 8
    pub image_url: String,          // 4
    pub sol_for_positioning: u64,   // 8
}

impl IterData {
    pub const SIZE: usize = 1 + 8;
}

impl Product {
    pub const SIZE: usize =
    1 +
    32 +
    8 +
    1 +
    4 +
    32 +
    4 +
    200 +
    4 +
    8 +
    4;
}

impl AccountData {
    pub const SIZE: usize =
    1 +
    8 +
    8 +
    4 +
    9952;
}
