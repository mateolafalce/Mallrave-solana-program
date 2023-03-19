use anchor_lang::{
    prelude::*,
    solana_program::pubkey::Pubkey,
};
use crate::state::accounts::*;
use crate::errors::ErrorCode;

pub fn update_supply(
        ctx: Context<UpdateSupply>,
        supply: u64
    ) -> Result<()> {
        require!(ctx.accounts.user.key() == ctx.accounts.offer.pubkey.key(), ErrorCode::PubkeyError);
        require!(supply > 0, ErrorCode::SupplyError);
        let main_account: &mut Account<MainAccount> = &mut ctx.accounts.main_account;
        main_account.active_offers += 1;
        let offer: &mut Account<Sell> = &mut ctx.accounts.offer;
        offer.supply += supply;
        offer.active = true;
        Ok(())
    }

#[derive(Accounts)]
pub struct UpdateSupply<'info> {
    #[account(mut, seeds = [b"Main Account"], bump = main_account.bump_original)]
    pub main_account: Account<'info, MainAccount>,
    #[account(mut, seeds = [offer.seed.to_be_bytes().as_ref()], bump = offer.bump_original)]
    pub offer: Account<'info, Sell>,
    /// CHECK: This is not dangerous
    #[account(mut, signer)]
    pub user: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
