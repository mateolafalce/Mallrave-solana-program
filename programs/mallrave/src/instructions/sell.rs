use anchor_lang::{
    prelude::*,
    solana_program::{
        pubkey::Pubkey,
        program::invoke,
        system_instruction::transfer,
        rent::Rent
    }
};
use crate::state::accounts::*;
use crate::errors::ErrorCode;

pub fn sell(
        ctx: Context<Sell>,
        product: String, // name
        description: String,
        supply: u32,
        price: u64,
        image_url: String, // variable size
        sol: u64
    ) -> Result<()> {
        let balance: u64 = **ctx.accounts.user.to_account_infos()[0].lamports.borrow();     //get balance of the account
        let total_amount: u64 = Rent::default().minimum_balance(
            sol as usize + Product::SIZE + image_url.len() + 8
        ); // amount to upload + sol for positioning
        require!(image_url.len() <= 9686, ErrorCode::LenghtError);
        require!(product.len() <= 32, ErrorCode::LenghtError);
        require!(description.len() <= 200, ErrorCode::LenghtError);
        require!(supply > 0, ErrorCode::LenghtError);
        require!(ctx.accounts.pda.key() > ctx.accounts.sell_account.key(), ErrorCode::PubkeyError);
        require!(balance > total_amount, ErrorCode::BalanceError);
        let transfer = transfer(
            &ctx.accounts.user.key(),
            &ctx.accounts.pda.key(),
            sol,
        );
        invoke(
            &transfer,
            &[
                ctx.accounts.user.to_account_info(),
                ctx.accounts.pda.to_account_info().clone(),
            ],
        )
        .expect("Error");
        let (pda, bump) = Pubkey::find_program_address(&[
                ctx.accounts.sell.key().to_bytes().as_ref(),
                ctx.accounts.iter.iter.to_be_bytes().as_ref(),
            ],
            ctx.program_id
        );
        let sell: &mut Account<Product> = &mut ctx.accounts.sell_account;
        let account_data: &mut Account<AccountData> = &mut ctx.accounts.sell;
        account_data.positioning.push(pda);
        sell.bump_original = bump;
        sell.pubkey = ctx.accounts.user.key();
        sell.seed = ctx.accounts.iter.iter;
        sell.product = product;
        sell.description = description;
        sell.supply = supply;
        sell.price = price;
        sell.image_url = image_url;
        sell.active = true;
        sell.sol_for_positioning = sol;
        let iter: &mut Account<IterData> = &mut ctx.accounts.iter;
        iter.iter += 1;
        Ok(())
    }

#[derive(Accounts)]
#[instruction(image_url: String)]
pub struct Sell<'info> {
    #[account(
        mut,
        seeds = [b"iter"],
        bump = iter.bump_original,
        )]
    pub iter: Account<'info, IterData>,
    #[account(
        init,
        seeds = [
            sell.key().to_bytes().as_ref(),
            iter.iter.to_be_bytes().as_ref()
        ],
        bump,
        payer = user,
        space = Product::SIZE + image_url.len() + 8)]
    pub sell_account: Account<'info, Product>,
    #[account(
        mut,
        seeds = [b"sell"],
        bump = sell.bump_original,
        )]
    pub sell: Account<'info, AccountData>,
    /// CHECK: This is not dangerous
    #[account(mut, signer)]
    pub user: AccountInfo<'info>,
    /// CHECK: This is not dangerous
    #[account(mut)]
    pub pda: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
