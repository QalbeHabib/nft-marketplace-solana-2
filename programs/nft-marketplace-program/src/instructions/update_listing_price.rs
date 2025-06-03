use crate::state::*;
use anchor_lang::prelude::*;

pub fn update_listing_price(ctx: Context<UpdateListing>, new_price: u64) -> Result<()> {
    msg!("Updating listing price");
    let listing = &mut ctx.accounts.listing;
    require!(
        listing.is_active,
        crate::errors::ErrorCode::ListingNotActive
    );
    require!(
        listing.seller == ctx.accounts.seller.key(),
        crate::errors::ErrorCode::UnauthorizedSeller
    );
    let old_price = listing.price;
    listing.price = new_price;
    msg!("Listing price updated to {} lamports", new_price);
    emit!(ListingPriceUpdated {
        listing: ctx.accounts.listing.key(),
        seller: ctx.accounts.seller.key(),
        old_price,
        new_price,
    });
    Ok(())
}
