use crate::{
    state::accounts::*,
    utils::constants::{MAIN_ACCOUNT, MAX_DESCRIPTION, MAX_IMAGE_URL, MAX_NAME},
};
use anchor_lang::{
    prelude::*,
    solana_program::{pubkey::Pubkey, rent::Rent},
};

pub fn init_sell_offer_(
    ctx: Context<InitSellOffer>,
    name: String,
    description: String,
    supply: u32,
    price: u64,
    image_url: String,
) -> Result<()> {
    let signer: Pubkey = ctx.accounts.user.key();
    let pda: Pubkey = ctx.accounts.pda.key();
    let balance: u64 = **ctx.accounts.user.to_account_infos()[0].lamports.borrow();
    let total_amount: u64 = Rent::default().minimum_balance(Product::SIZE);
    let (pda_product, bump) = Pubkey::find_program_address(&[&signer.to_bytes()], ctx.program_id);
    // validate input & balances
    require_keys_eq!(pda, pda_product);
    require_gte!(balance, total_amount);
    require_gte!(MAX_NAME, name.len());
    require_gte!(MAX_DESCRIPTION, description.len());
    require_gte!(MAX_IMAGE_URL, image_url.len());
    require_gt!(supply, 0);
    // update state
    let sell: &mut Account<Product> = &mut ctx.accounts.sell_account;
    let account_data: &mut Account<AccountData> = &mut ctx.accounts.account_data;
    account_data.add_product(pda_product);
    sell.set_bump_original(bump);
    sell.set_authority(signer);
    sell.set_name(name);
    sell.set_description(description);
    sell.set_supply(supply);
    sell.set_price(price);
    sell.set_image_url(image_url);
    Ok(())
}

#[derive(Accounts)]
#[instruction(image_url: String)]
pub struct InitSellOffer<'info> {
    #[account(mut, seeds = [MAIN_ACCOUNT], bump = account_data.bump_original)]
    pub account_data: Account<'info, AccountData>,
    #[account(
        init,
        seeds = [&user.key().to_bytes()],
        bump,
        payer = user,
        space = Product::SIZE + image_url.len())]
    pub sell_account: Account<'info, Product>,
    /// CHECK: This is not dangerous
    #[account(mut, signer)]
    pub user: AccountInfo<'info>,
    /// CHECK: This is not dangerous
    #[account(mut)]
    pub pda: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
