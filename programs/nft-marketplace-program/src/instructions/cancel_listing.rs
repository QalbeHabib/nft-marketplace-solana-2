use crate::state::*;
use anchor_lang::prelude::*;

pub fn cancel_listing(ctx: Context<CancelListing>) -> Result<()> {
    msg!("Canceling NFT listing");
    let listing = &mut ctx.accounts.listing;
    require!(
        listing.is_active,
        crate::errors::ErrorCode::ListingNotActive
    );
    require!(
        listing.seller == ctx.accounts.seller.key(),
        crate::errors::ErrorCode::UnauthorizedSeller
    );
    listing.is_active = false;
    msg!("NFT listing canceled successfully");
    emit!(ListingCanceled {
        listing: ctx.accounts.listing.key(),
        mint: ctx.accounts.listing.mint,
        seller: ctx.accounts.seller.key(),
    });
    Ok(())
}
