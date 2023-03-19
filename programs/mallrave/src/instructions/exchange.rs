use anchor_lang::{
    prelude::*,
    solana_program::system_instruction,
    solana_program::pubkey::Pubkey,
}; 
use crate::state::accounts::*;
use crate::errors::ErrorCode;

pub fn exchange(
        ctx: Context<Exchange>,
        supply: u64
    ) -> Result<()> {
        require!(ctx.accounts.seller.key() == ctx.accounts.offer.pubkey.key(), ErrorCode::PubkeyError);
        require!(supply <= ctx.accounts.offer.supply, ErrorCode::OfferError);
        require!(ctx.accounts.offer.supply > 0, ErrorCode::SupplyError);
        let lamports: u64 = ctx.accounts.offer.price * supply;
        anchor_lang::solana_program::program::invoke(
            &system_instruction::transfer(
                &ctx.accounts.buyer.key(), 
                &ctx.accounts.offer.pubkey.key(), 
                lamports
            ),
            &[
                ctx.accounts.buyer.to_account_info(), 
                ctx.accounts.offer.to_account_info()
                .clone()
            ],
        ).expect("Error");
        let main_account: &mut Account<MainAccount> = &mut ctx.accounts.main_account;
        main_account.transactions += 1;
        let offer: &mut Account<Sell> = &mut ctx.accounts.offer;
        offer.supply -= supply;
        if offer.supply == 0 {
            main_account.active_offers -= 1;
            offer.active = false
        }
        Ok(())
    }

#[derive(Accounts)]
pub struct Exchange<'info> {
    #[account(mut, seeds = [b"Main Account"], bump = main_account.bump_original)]
    pub main_account: Account<'info, MainAccount>,
    #[account(mut, seeds = [offer.seed.to_be_bytes().as_ref()], bump = offer.bump_original)]
    pub offer: Account<'info, Sell>,
    /// CHECK: This is not dangerous
    #[account(mut, signer)]
    pub buyer: AccountInfo<'info>,
    /// CHECK: This is not dangerous
    #[account(mut)]
    pub seller: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}