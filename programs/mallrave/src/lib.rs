use anchor_lang::prelude::*;
use instructions::*;

pub mod instructions;
pub mod state;
pub mod utils;

use instructions::{
    exchange::exchange_, init_accounts::init_global_account_, init_sell_offer::init_sell_offer_,
    update_supply::update_supply_,
};

declare_id!("BEtio1iiUCWv1k3L3fpM6dEKafEMAdzyznPFupXYhdD6");

#[program]
pub mod mallrave {
    use super::*;
    pub fn init_global_account(ctx: Context<InitGlobalAccount>) -> Result<()> {
        init_global_account_(ctx)
    }
    pub fn init_sell_offer(
        ctx: Context<InitSellOffer>,
        name: String,
        description: String,
        supply: u32,
        price: u64,
        image_url: String,
    ) -> Result<()> {
        init_sell_offer_(ctx, name, description, supply, price, image_url)
    }
    pub fn update_supply(ctx: Context<UpdateSupply>, supply: u32) -> Result<()> {
        update_supply_(ctx, supply)
    }
    pub fn exchange(ctx: Context<Exchange>, supply: u32) -> Result<()> {
        exchange_(ctx, supply)
    }
}
