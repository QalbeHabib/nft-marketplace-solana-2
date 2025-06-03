use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::metadata::{
    set_and_verify_collection as spl_set_and_verify_collection, SetAndVerifyCollection,
};
use mpl_token_metadata::accounts::Metadata as MetadataAccount;

pub fn set_and_verify_collection(
    ctx: Context<SetAndVerifyCollectionCtx>,
    collection_key: Pubkey,
) -> Result<()> {
    msg!("Setting and verifying collection");

    // Deserialize collection metadata account
    let collection_metadata_account =
        MetadataAccount::try_from(&ctx.accounts.collection_metadata.to_account_info())?;

    // Verify collection authority matches update authority in collection metadata
    require!(
        ctx.accounts.collection_authority.key() == collection_metadata_account.update_authority,
        crate::errors::ErrorCode::UnauthorizedCollectionUpdateAuthority
    );

    spl_set_and_verify_collection(
        CpiContext::new(
            ctx.accounts.metadata_program.to_account_info(),
            SetAndVerifyCollection {
                metadata: ctx.accounts.nft_metadata.to_account_info(),
                collection_authority: ctx.accounts.collection_authority.to_account_info(),
                payer: ctx.accounts.payer.to_account_info(),
                update_authority: ctx.accounts.update_authority.to_account_info(),
                collection_mint: ctx.accounts.collection_mint.to_account_info(),
                collection_metadata: ctx.accounts.collection_metadata.to_account_info(),
                collection_master_edition: ctx.accounts.collection_master_edition.to_account_info(),
            },
        ),
        Some(collection_key),
    )?;
    msg!("Collection set and verified successfully");
    Ok(())
}
