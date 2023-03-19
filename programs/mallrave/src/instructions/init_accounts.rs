use anchor_lang::{
    prelude::*,
    solana_program::pubkey::Pubkey,
};
use crate::state::accounts::*;

pub fn init_accounts(
    ctx: Context<InitAccounts>
) -> Result<()> {
    let iter: &mut Account<IterData> = &mut ctx.accounts.iter;
    let (_pda, iter_bump): (Pubkey, u8) = Pubkey::find_program_address(&[
        b"iter"
        ], ctx.program_id);
    iter.bump_original = iter_bump;
    iter.iter = 0;

    let sell: &mut Account<AccountData> = &mut ctx.accounts.sell;
    let (_pda, bump): (Pubkey, u8) = Pubkey::find_program_address(&[
        b"sell"
        ], ctx.program_id);
    sell.bump_original = bump;
    sell.transactions = 0;
    sell.average_exchange_time = 0;
    sell.positioning = [].to_vec();

    Ok(())
}

#[derive(Accounts)]
pub struct InitAccounts<'info> {
    #[account(
        init,
        seeds = [b"iter"],
        bump,
        payer = user,
        space = IterData::SIZE + 8)]
    pub iter: Account<'info, IterData>,

    #[account(
        init,
        seeds = [b"sell"],
        bump,
        payer = user,
        space = AccountData::SIZE + 8)]
    pub sell: Account<'info, AccountData>,
    /// CHECK: This is not dangerous
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
