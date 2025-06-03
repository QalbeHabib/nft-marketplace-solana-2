use crate::state::*;
use anchor_lang::prelude::*;
use mpl_token_metadata::accounts::Metadata as MetadataAccount;

pub fn list_nft(
    ctx: Context<ListNFT>,
    price: u64,
    listing_id: u64,
    collection_mint: Pubkey,
) -> Result<()> {
    msg!("Listing NFT for sale");
    let listing = &mut ctx.accounts.listing;
    let clock = Clock::get()?;

    // FIXED: Enhanced ownership verification
    require!(
        ctx.accounts.seller_token_account.amount == 1,
        crate::errors::ErrorCode::SellerDoesNotOwnNFT
    );

    // FIXED: Verify the token account holds the correct NFT mint
    require!(
        ctx.accounts.seller_token_account.mint == ctx.accounts.mint.key(),
        crate::errors::ErrorCode::InvalidNFT
    );

    // FIXED: Verify the seller actually owns the token account
    require!(
        ctx.accounts.seller_token_account.owner == ctx.accounts.seller.key(),
        crate::errors::ErrorCode::UnauthorizedSeller
    );

    // Additional validation: Ensure the mint account matches the expected mint
    require!(
        ctx.accounts.mint.key() == ctx.accounts.seller_token_account.mint,
        crate::errors::ErrorCode::InvalidNFT
    );

    // NEW: Validate that the NFT's metadata references the provided collection_mint
    let metadata_account = MetadataAccount::try_from(&ctx.accounts.nft_metadata.to_account_info())?;

    // Check if the NFT has a collection set and if it matches the provided collection_mint
    match &metadata_account.collection {
        Some(collection) => {
            require!(
                collection.key == collection_mint,
                crate::errors::ErrorCode::NFTNotInCollection
            );
            require!(
                collection.verified,
                crate::errors::ErrorCode::CollectionVerificationFailed
            );
        }
        None => {
            // If collection_mint is provided but NFT has no collection, reject
            require!(
                collection_mint == Pubkey::default(),
                crate::errors::ErrorCode::NFTNotInCollection
            );
        }
    }

    listing.seller = ctx.accounts.seller.key();
    listing.mint = ctx.accounts.mint.key();
    listing.collection_mint = collection_mint; // Store the collection_mint
    listing.price = price;
    listing.is_active = true;
    listing.listed_at = clock.unix_timestamp;
    listing.bump = ctx.bumps.listing;

    msg!(
        "NFT listed successfully for {} lamports with collection {}",
        price,
        collection_mint
    );
    emit!(NftListed {
        listing_id,
        mint: ctx.accounts.mint.key(),
        seller: ctx.accounts.seller.key(),
        collection_mint,
        price,
        listed_at: listing.listed_at,
    });
    Ok(())
}
