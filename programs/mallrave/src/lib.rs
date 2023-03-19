use anchor_lang::prelude::*;
use instructions::*;
use crate::errors::ErrorCode;

pub mod errors;
pub mod instructions;
pub mod state;

declare_id!("BEtio1iiUCWv1k3L3fpM6dEKafEMAdzyznPFupXYhdD6");

#[program]
pub mod mallrave {
    use super::*;
    pub fn init_accounts(
        ctx: Context<InitAccounts>
    ) -> Result<()> {
        instructions::init_accounts::init_accounts(
            ctx
        )
    }
    pub fn sell(
        ctx: Context<Sell>,
        product: String,
        description: String,
        supply: u32,
        price: u64,
        image_url: String,
        sol:u64
    ) -> Result<()> {
        instructions::sell::sell(
            ctx,
            product,
            description,
            supply,
            price,
            image_url,
            sol
        )
    }

}
