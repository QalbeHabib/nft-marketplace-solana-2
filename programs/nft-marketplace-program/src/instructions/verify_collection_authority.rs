use crate::state::*;
use anchor_lang::prelude::*;

pub fn verify_collection_authority(ctx: Context<VerifyCollectionAuthority>) -> Result<()> {
    msg!("Verifying collection authority");
    require!(
        ctx.accounts.collection_authority.key()
            == *ctx.accounts.collection_metadata.to_account_info().owner,
        crate::errors::ErrorCode::InvalidCollectionAuthority
    );
    msg!("Collection authority verified successfully");
    Ok(())
}
