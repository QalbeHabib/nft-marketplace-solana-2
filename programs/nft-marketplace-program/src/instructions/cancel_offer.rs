use crate::state::*;
use anchor_lang::prelude::*;

pub fn cancel_offer(ctx: Context<CancelOffer>) -> Result<()> {
    msg!("Canceling offer");
    let offer = &mut ctx.accounts.offer;
    require!(offer.is_active, crate::errors::ErrorCode::OfferNotActive);
    require!(
        offer.buyer == ctx.accounts.buyer.key(),
        crate::errors::ErrorCode::UnauthorizedBuyer
    );
    offer.is_active = false;
    msg!("Offer canceled successfully");
    emit!(OfferCanceled {
        offer: ctx.accounts.offer.key(),
        mint: ctx.accounts.offer.mint,
        buyer: ctx.accounts.buyer.key(),
    });
    Ok(())
}
