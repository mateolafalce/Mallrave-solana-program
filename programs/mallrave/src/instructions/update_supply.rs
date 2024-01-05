use crate::state::accounts::*;
use anchor_lang::{prelude::*, solana_program::pubkey::Pubkey};

pub fn update_supply_(ctx: Context<UpdateSupply>, supply: u32) -> Result<()> {
    let signer: Pubkey = ctx.accounts.user.key();
    let auth_product: Pubkey = ctx.accounts.product.authority.key();
    require_keys_eq!(signer, auth_product);
    require_gt!(supply, 0);
    // update state
    let product: &mut Account<Product> = &mut ctx.accounts.product;
    product.update_supply(supply);
    Ok(())
}

#[derive(Accounts)]
pub struct UpdateSupply<'info> {
    #[account(mut, seeds = [&user.key().to_bytes()], bump = product.bump_original)]
    pub product: Account<'info, Product>,
    /// CHECK: This is not dangerous
    #[account(mut, signer)]
    pub user: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
