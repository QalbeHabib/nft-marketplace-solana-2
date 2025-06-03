use crate::state::*;
use crate::utils::calculate_marketplace_fee;
use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Transfer};

pub fn buy_nft(ctx: Context<BuyNFT>, marketplace_fee_bps: u16) -> Result<()> {
    msg!("Buying NFT from listing");
    let listing = &mut ctx.accounts.listing;
    require!(
        listing.is_active,
        crate::errors::ErrorCode::ListingNotActive
    );
    require!(
        listing.seller != ctx.accounts.buyer.key(),
        crate::errors::ErrorCode::SellerCannotBuy
    );

    let total_price = listing.price;

    // FIXED: Safe fee calculation with overflow protection
    let marketplace_fee = calculate_marketplace_fee(total_price, marketplace_fee_bps)?;
    let seller_amount = total_price
        .checked_sub(marketplace_fee)
        .ok_or(crate::errors::ErrorCode::ArithmeticOverflow)?;

    let transfer_instruction = anchor_lang::system_program::Transfer {
        from: ctx.accounts.buyer.to_account_info(),
        to: ctx.accounts.seller.to_account_info(),
    };
    anchor_lang::system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            transfer_instruction,
        ),
        seller_amount,
    )?;

    if marketplace_fee > 0 {
        let fee_transfer_instruction = anchor_lang::system_program::Transfer {
            from: ctx.accounts.buyer.to_account_info(),
            to: ctx.accounts.marketplace_authority.to_account_info(),
        };
        anchor_lang::system_program::transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                fee_transfer_instruction,
            ),
            marketplace_fee,
        )?;
    }

    let cpi_accounts = Transfer {
        from: ctx.accounts.seller_token_account.to_account_info(),
        to: ctx.accounts.buyer_token_account.to_account_info(),
        authority: ctx.accounts.seller.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    transfer(cpi_ctx, 1)?;

    listing.is_active = false;
    msg!("NFT purchased successfully for {} lamports", total_price);
    emit!(NftPurchased {
        listing: ctx.accounts.listing.key(),
        mint: ctx.accounts.mint.key(),
        buyer: ctx.accounts.buyer.key(),
        seller: ctx.accounts.seller.key(),
        price: total_price,
        marketplace_fee,
    });
    Ok(())
}
