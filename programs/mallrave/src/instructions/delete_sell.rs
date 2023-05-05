use anchor_lang::{
    prelude::*,
    solana_program::pubkey::Pubkey,
};
use crate::state::accounts::*;
use crate::errors::ErrorCode;

pub fn delete_sell(
    ctx: Context<DeleteSell>
) -> Result<()> {
    // Check that the user key matches the offer's pubkey key
    require!(ctx.accounts.user.key() == ctx.accounts.offer.pubkey.key(), ErrorCode::PubkeyError);
    // Calculate the amount of lamports to transfer
    let lamport: u64 = 9333360 - 890880;
    // Decrease the offer's account's lamports by the calculated amount
    **ctx.accounts.offer.to_account_info().try_borrow_mut_lamports()? -= lamport;
    // Increase the user's account's lamports by the calculated amount
    **ctx.accounts.user.to_account_info().try_borrow_mut_lamports()? += lamport;
    Ok(())
}

#[derive(Accounts)]
pub struct DeleteSell<'info> {
    #[account(mut, seeds = [b"Main Account"], bump = main_account.bump_original)]
    pub main_account: Account<'info, MainAccount>,
    #[account(
        mut,
        seeds = [offer.seed.to_be_bytes().as_ref()],
        bump = offer.bump_original,
        close = user // Close the offer's account and transfer its remaining lamports to the user's account
    )]
    pub offer: Account<'info, Sell>, // The account representing the offer being deleted
    #[account(mut, signer)]
    pub user: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
