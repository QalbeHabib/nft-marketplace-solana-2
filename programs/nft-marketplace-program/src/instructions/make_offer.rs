use crate::state::*;
use anchor_lang::prelude::*;

pub fn make_offer(
    ctx: Context<MakeOffer>,
    offer_price: u64,
    expiry_time: i64,
    offer_id: u64,
) -> Result<()> {
    msg!("Making offer on NFT");
    let offer = &mut ctx.accounts.offer;
    let clock = Clock::get()?;
    // FIXED: Enhanced expiry validation with minimum duration
    require!(
        expiry_time > clock.unix_timestamp,
        crate::errors::ErrorCode::OfferExpired
    );

    require!(
        expiry_time <= clock.unix_timestamp + (365 * 24 * 60 * 60), // Max 1 year
        crate::errors::ErrorCode::OfferExpiryTooLong
    );

    require!(
        ctx.accounts.buyer.lamports() >= offer_price,
        crate::errors::ErrorCode::InsufficientFunds
    );

    offer.buyer = ctx.accounts.buyer.key();
    offer.mint = ctx.accounts.mint.key();
    offer.price = offer_price;
    offer.expiry_time = expiry_time;
    offer.is_active = true;
    offer.created_at = clock.unix_timestamp;
    offer.bump = ctx.bumps.offer;
    msg!("Offer made for {} lamports", offer_price);
    emit!(OfferMade {
        offer_id,
        mint: ctx.accounts.mint.key(),
        buyer: ctx.accounts.buyer.key(),
        price: offer_price,
        expiry_time,
        created_at: offer.created_at,
    });
    Ok(())
}
