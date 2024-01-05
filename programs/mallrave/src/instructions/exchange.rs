use crate::{state::accounts::*, utils::constants::MAIN_ACCOUNT};
use anchor_lang::{
    prelude::*,
    solana_program::{program::invoke, pubkey::Pubkey, system_instruction::transfer},
};

pub fn exchange_(ctx: Context<Exchange>, supply: u32) -> Result<()> {
    let seller: Pubkey = ctx.accounts.seller.key();
    let offer: Pubkey = ctx.accounts.offer.authority.key();
    let buyer: Pubkey = ctx.accounts.buyer.key();
    let current_supply: u32 = ctx.accounts.offer.supply;
    // validations
    require_keys_eq!(seller, offer);
    require_gte!(current_supply, supply);
    require_gte!(current_supply, 0);
    // make the tx
    let lamports: u64 = ctx.accounts.offer.price * supply as u64;
    invoke(
        &transfer(&buyer, &offer, lamports),
        &[
            ctx.accounts.buyer.to_account_info(),
            ctx.accounts.offer.to_account_info().clone(),
        ],
    )
    .expect("Error");
    // update state
    let main_account: &mut Account<AccountData> = &mut ctx.accounts.main_account;
    let product: &mut Account<Product> = &mut ctx.accounts.offer;
    main_account.add_transactions();
    //product.supply -= supply;
    if product.supply == 0 {
        //main_account.active_offers -= 1;
    }
    Ok(())
}

#[derive(Accounts)]
pub struct Exchange<'info> {
    #[account(mut, seeds = [MAIN_ACCOUNT], bump = main_account.bump_original)]
    pub main_account: Account<'info, AccountData>,
    #[account(mut, seeds = [&seller.key().to_bytes()], bump = offer.bump_original)]
    pub offer: Account<'info, Product>,
    /// CHECK: This is not dangerous
    #[account(mut, signer)]
    pub buyer: AccountInfo<'info>,
    /// CHECK: This is not dangerous
    #[account(mut)]
    pub seller: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
