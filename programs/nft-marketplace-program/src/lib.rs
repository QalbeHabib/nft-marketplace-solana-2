use anchor_lang::prelude::*;

declare_id!("Equiqs1Z5Q4F1gBuciqo6yrvqNERzwp5v9Fskhq2A5WB");

// Import all modules
mod constants;
mod errors;
mod instructions;
mod state;
mod utils;

// Re-export everything
pub use constants::*;
pub use errors::*;
pub use instructions::*;
pub use state::*;
pub use utils::*;

#[program]
pub mod nft_program {
    use super::*;

    pub fn create_single_nft(
        ctx: Context<CreateNFT>,
        id: u64,
        name: String,
        symbol: String,
        uri: String,
        royalty_percent: u16,
    ) -> Result<()> {
        instructions::create_single_nft(ctx, id, name, symbol, uri, royalty_percent)
    }

    pub fn mint_to_collection(
        ctx: Context<MintToCollection>,
        id_nft: u64,
        name: String,
        symbol: String,
        uri: String,
        royalty_percent: u16,
    ) -> Result<()> {
        instructions::mint_to_collection(ctx, id_nft, name, symbol, uri, royalty_percent)
    }

    pub fn create_collection(
        ctx: Context<CreateCollection>,
        id_collection: u64,
        name: String,
        symbol: String,
        uri: String,
    ) -> Result<()> {
        instructions::create_collection(ctx, id_collection, name, symbol, uri)
    }

    pub fn verify_nft_in_collection(ctx: Context<VerifyNFTInCollection>) -> Result<()> {
        instructions::verify_nft_in_collection(ctx)
    }

    pub fn set_and_verify_collection(
        ctx: Context<SetAndVerifyCollectionCtx>,
        collection_key: Pubkey,
    ) -> Result<()> {
        instructions::set_and_verify_collection(ctx, collection_key)
    }

    pub fn unverify_nft_from_collection(ctx: Context<UnverifyNFTFromCollection>) -> Result<()> {
        instructions::unverify_nft_from_collection(ctx)
    }

    pub fn verify_collection_authority(ctx: Context<VerifyCollectionAuthority>) -> Result<()> {
        instructions::verify_collection_authority(ctx)
    }

    pub fn mint_and_verify_to_collection(
        ctx: Context<MintAndVerifyToCollection>,
        id_nft: u64,
        name: String,
        symbol: String,
        uri: String,
        royalty_percent: u16,
    ) -> Result<()> {
        instructions::mint_and_verify_to_collection(ctx, id_nft, name, symbol, uri, royalty_percent)
    }

    pub fn list_nft(
        ctx: Context<ListNFT>,
        price: u64,
        listing_id: u64,
        collection_mint: Pubkey,
    ) -> Result<()> {
        instructions::list_nft(ctx, price, listing_id, collection_mint)
    }

    pub fn update_listing_price(ctx: Context<UpdateListing>, new_price: u64) -> Result<()> {
        instructions::update_listing_price(ctx, new_price)
    }

    pub fn cancel_listing(ctx: Context<CancelListing>) -> Result<()> {
        instructions::cancel_listing(ctx)
    }

    pub fn buy_nft(ctx: Context<BuyNFT>, marketplace_fee_bps: u16) -> Result<()> {
        instructions::buy_nft(ctx, marketplace_fee_bps)
    }

    pub fn make_offer(
        ctx: Context<MakeOffer>,
        offer_price: u64,
        expiry_time: i64,
        offer_id: u64,
    ) -> Result<()> {
        instructions::make_offer(ctx, offer_price, expiry_time, offer_id)
    }

    pub fn accept_offer(ctx: Context<AcceptOffer>, marketplace_fee_bps: u16) -> Result<()> {
        instructions::accept_offer(ctx, marketplace_fee_bps)
    }

    pub fn cancel_offer(ctx: Context<CancelOffer>) -> Result<()> {
        instructions::cancel_offer(ctx)
    }

    pub fn initialize_program_state(
        ctx: Context<InitializeProgramState>,
        minting_price: u64,
    ) -> Result<()> {
        instructions::initialize_program_state(ctx, minting_price)
    }

    pub fn set_minting_price(ctx: Context<SetMintingPrice>, new_price: u64) -> Result<()> {
        instructions::set_minting_price(ctx, new_price)
    }
}
