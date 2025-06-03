use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::metadata::{verify_collection, VerifyCollection};
use mpl_token_metadata::accounts::Metadata as MetadataAccount;

pub fn verify_nft_in_collection(ctx: Context<VerifyNFTInCollection>) -> Result<()> {
    msg!("Verifying NFT in collection");

    // Deserialize collection metadata account
    let collection_metadata_account =
        MetadataAccount::try_from(&ctx.accounts.collection_metadata.to_account_info())?;

    // Verify collection authority matches update authority in collection metadata
    require!(
        ctx.accounts.collection_authority.key() == collection_metadata_account.update_authority,
        crate::errors::ErrorCode::UnauthorizedCollectionUpdateAuthority
    );

    verify_collection(
        CpiContext::new(
            ctx.accounts.metadata_program.to_account_info(),
            VerifyCollection {
                payer: ctx.accounts.payer.to_account_info(),
                metadata: ctx.accounts.nft_metadata.to_account_info(),
                collection_authority: ctx.accounts.collection_authority.to_account_info(),
                collection_mint: ctx.accounts.collection_mint.to_account_info(),
                collection_metadata: ctx.accounts.collection_metadata.to_account_info(),
                collection_master_edition: ctx.accounts.collection_master_edition.to_account_info(),
            },
        ),
        None,
    )?;
    msg!("NFT verified in collection successfully");
    Ok(())
}
