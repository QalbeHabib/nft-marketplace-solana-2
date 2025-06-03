use crate::state::*;
use anchor_lang::prelude::*;

pub fn set_minting_price(ctx: Context<SetMintingPrice>, new_price: u64) -> Result<()> {
    let program_state = &mut ctx.accounts.program_state;
    program_state.minting_price = new_price;
    msg!("Minting price updated to: {} lamports", new_price);
    Ok(())
}
