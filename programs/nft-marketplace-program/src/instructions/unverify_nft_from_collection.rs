use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::metadata::{unverify_collection, UnverifyCollection};

pub fn unverify_nft_from_collection(ctx: Context<UnverifyNFTFromCollection>) -> Result<()> {
    msg!("Unverifying NFT from collection");
    unverify_collection(
        CpiContext::new(
            ctx.accounts.metadata_program.to_account_info(),
            UnverifyCollection {
                metadata: ctx.accounts.nft_metadata.to_account_info(),
                collection_authority: ctx.accounts.collection_authority.to_account_info(),
                collection_mint: ctx.accounts.collection_mint.to_account_info(),
                collection: ctx.accounts.collection_metadata.to_account_info(),
                collection_master_edition_account: ctx
                    .accounts
                    .collection_master_edition
                    .to_account_info(),
            },
        ),
        None,
    )?;
    msg!("NFT unverified from collection successfully");
    Ok(())
}
