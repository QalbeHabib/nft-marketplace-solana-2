use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::metadata::Metadata;
use anchor_spl::token::{Mint, Token, TokenAccount};
use mpl_token_metadata::types::Creator;
use crate::constants::*;

#[account]
pub struct Listing {
    pub seller: Pubkey,
    pub mint: Pubkey,
    pub collection_mint: Pubkey,
    pub price: u64,
    pub is_active: bool,
    pub listed_at: i64,
    pub bump: u8,
}

impl Listing {
    pub const LEN: usize = 8 + 32 + 32 + 32 + 8 + 1 + 8 + 1;
}

#[account]
pub struct Offer {
    pub buyer: Pubkey,
    pub mint: Pubkey,
    pub price: u64,
    pub expiry_time: i64,
    pub is_active: bool,
    pub created_at: i64,
    pub bump: u8,
}

impl Offer {
    pub const LEN: usize = 8 + 32 + 32 + 8 + 8 + 1 + 8 + 1;
}

// FIXED: Enhanced ProgramState with cleanup reward
#[account]
pub struct ProgramState {
    pub admin: Pubkey,
    pub minting_price: u64,
}

impl ProgramState {
    pub const LEN: usize = 8 + 32 + 8 + 8;
}

#[account]
pub struct UserCollection {
    pub authority: Pubkey,            // User who created the collection
    pub collection_mint: Pubkey,      // Collection mint address
    pub name: String,                 // Collection name
    pub symbol: String,               // Collection symbol
    pub uri: String,                  // Collection metadata URI
    pub created_at: i64,             // Timestamp when collection was created
    pub total_items: u64,            // Total items in collection (can be updated)
    pub verified: bool,              // Whether collection is verified
    pub bump: u8,                    // PDA bump
}

impl UserCollection {
    // Calculate space needed for the account
    pub const LEN: usize = 8 +  // discriminator
        32 +                    // authority
        32 +                    // collection_mint
        64 +                    // name (max length 32)
        16 +                    // symbol (max length 8)
        200 +                   // uri (max length 200)
        8 +                     // created_at
        8 +                     // total_items
        1 +                     // verified
        1;                      // bump

    pub fn get_collection_by_authority(
        program_id: &Pubkey,
        authority: &Pubkey,
        collection_mint: &Pubkey,
    ) -> Pubkey {
        Pubkey::find_program_address(
            &[
                PROGRAM_SEED_PREFIX,
                b"user_collection",
                authority.as_ref(),
                collection_mint.as_ref(),
            ],
            program_id,
        )
        .0
    }
}

// Context Structs
#[derive(Accounts)]
#[instruction(id: u64)]
pub struct CreateNFT<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        payer = payer,
        mint::decimals = 0,
        mint::authority = authority,
        mint::freeze_authority = authority,
        seeds = [
            PROGRAM_SEED_PREFIX,
            MINT_SEED_PREFIX,
            crate::ID.as_ref(),
            id.to_le_bytes().as_ref(),
        ],
        bump
    )]
    pub mint: Account<'info, Mint>,
    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = payer,
    )]
    pub token_account: Account<'info, TokenAccount>,
    #[account(
        seeds = [PROGRAM_SEED_PREFIX, PROGRAM_STATE_SEED, crate::ID.as_ref()],
        bump
    )]
    pub program_state: Account<'info, ProgramState>,
    /// CHECK: This is the admin's account to receive the minting fee
    #[account(mut, address = program_state.admin)]
    pub mint_fee_account: AccountInfo<'info>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub metadata_program: Program<'info, Metadata>,
    #[account(
        mut,
        seeds = [
            b"metadata".as_ref(),
            metadata_program.key().as_ref(),
            mint.key().as_ref(),
            b"edition".as_ref(),
        ],
        bump,
        seeds::program = metadata_program.key()
    )]
    /// CHECK:
    pub master_edition_account: UncheckedAccount<'info>,
    #[account(
        mut,
        seeds = [
            b"metadata".as_ref(),
            metadata_program.key().as_ref(),
            mint.key().as_ref(),
        ],
        bump,
        seeds::program = metadata_program.key()
    )]
    /// CHECK:
    pub nft_metadata: UncheckedAccount<'info>,
}

#[derive(Accounts)]
#[instruction(id_collection: u64, name: String, symbol: String)]
pub struct CreateCollection<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    
    #[account(mut)]
    pub payer: Signer<'info>,
    
    #[account(
        init,
        payer = payer,
        mint::decimals = 0,
        mint::authority = authority,
        mint::freeze_authority = authority,
        seeds = [
            PROGRAM_SEED_PREFIX,
            COLLECTION_MINT_SEED_PREFIX,
            crate::ID.as_ref(),
            id_collection.to_le_bytes().as_ref(),
        ],
        bump
    )]
    pub mint: Account<'info, Mint>,

    #[account(
        init,
        payer = payer,
        space = UserCollection::LEN,
        seeds = [
            PROGRAM_SEED_PREFIX,
            b"user_collection",
            authority.key().as_ref(),
            mint.key().as_ref(),
        ],
        bump
    )]
    pub user_collection: Account<'info, UserCollection>,
    
    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = payer,
    )]
    pub token_account: Account<'info, TokenAccount>,
    
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub metadata_program: Program<'info, Metadata>,
    
    #[account(
        mut,
        seeds = [
            b"metadata".as_ref(),
            metadata_program.key().as_ref(),
            mint.key().as_ref(),
            b"edition".as_ref(),
        ],
        bump,
        seeds::program = metadata_program.key()
    )]
    /// CHECK:
    pub master_edition_account: UncheckedAccount<'info>,
    
    #[account(
        mut,
        seeds = [
            b"metadata".as_ref(),
            metadata_program.key().as_ref(),
            mint.key().as_ref(),
        ],
        bump,
        seeds::program = metadata_program.key()
    )]
    /// CHECK:
    pub nft_metadata: UncheckedAccount<'info>,
}

// FIXED: Enhanced MintToCollection context with collection verification support
#[derive(Accounts)]
#[instruction(id_nft: u64)]
pub struct MintToCollection<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        payer = payer,
        mint::decimals = 0,
        mint::authority = authority,
        mint::freeze_authority = authority,
        seeds = [
            PROGRAM_SEED_PREFIX,
            COLLECTION_ITEM_SEED_PREFIX,
            crate::ID.as_ref(),
            collection.key().as_ref(),
            id_nft.to_le_bytes().as_ref()
        ],
        bump,
    )]
    pub mint: Account<'info, Mint>,
    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = payer,
    )]
    pub token_account: Account<'info, TokenAccount>,
    #[account(
        seeds = [PROGRAM_SEED_PREFIX, PROGRAM_STATE_SEED, crate::ID.as_ref()],
        bump
    )]
    pub program_state: Account<'info, ProgramState>,
    /// CHECK: This is the admin's account to receive the minting fee
    #[account(mut, address = program_state.admin)]
    pub mint_fee_account: AccountInfo<'info>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub metadata_program: Program<'info, Metadata>,
    #[account(
        mut,
        seeds = [
            b"metadata".as_ref(),
            metadata_program.key().as_ref(),
            mint.key().as_ref(),
            b"edition".as_ref(),
        ],
        bump,
        seeds::program = metadata_program.key()
    )]
    /// CHECK:
    pub master_edition_account: UncheckedAccount<'info>,
    #[account(
        mut,
        seeds = [
            b"metadata".as_ref(),
            metadata_program.key().as_ref(),
            mint.key().as_ref(),
        ],
        bump,
        seeds::program = metadata_program.key()
    )]
    /// CHECK:
    pub nft_metadata: UncheckedAccount<'info>,
    /// CHECK: Collection mint account
    pub collection: UncheckedAccount<'info>,
    /// CHECK: Optional collection metadata for verification
    pub collection_metadata: Option<UncheckedAccount<'info>>,
    /// CHECK: Optional collection authority for verification
    pub collection_authority: Option<Signer<'info>>,
    /// CHECK: Optional collection master edition for verification
    pub collection_master_edition: Option<UncheckedAccount<'info>>,
}

#[derive(Accounts)]
#[instruction(id_nft: u64)]
pub struct MintAndVerifyToCollection<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK:
    pub collection_authority: Signer<'info>,
    #[account(
        init,
        payer = payer,
        mint::decimals = 0,
        mint::authority = authority,
        mint::freeze_authority = authority,
        seeds = [PROGRAM_SEED_PREFIX,
                 COLLECTION_ITEM_SEED_PREFIX,
                 crate::ID.as_ref(),
                 collection_mint.key().as_ref(),
                 id_nft.to_le_bytes().as_ref()],
        bump,
    )]
    pub mint: Account<'info, Mint>,
    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = payer,
    )]
    pub token_account: Account<'info, TokenAccount>,
    #[account(
        seeds = [PROGRAM_SEED_PREFIX, PROGRAM_STATE_SEED, crate::ID.as_ref()],
        bump
    )]
    pub program_state: Account<'info, ProgramState>,
    /// CHECK: This is the admin's account to receive the minting fee
    #[account(mut, address = program_state.admin)]
    pub mint_fee_account: AccountInfo<'info>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub metadata_program: Program<'info, Metadata>,
    #[account(
        mut,
        seeds = [
            b"metadata".as_ref(),
            metadata_program.key().as_ref(),
            mint.key().as_ref(),
            b"edition".as_ref(),
        ],
        bump,
        seeds::program = metadata_program.key()
    )]
    /// CHECK:
    pub master_edition_account: UncheckedAccount<'info>,
    #[account(
        mut,
        seeds = [
            b"metadata".as_ref(),
            metadata_program.key().as_ref(),
            mint.key().as_ref(),
        ],
        bump,
        seeds::program = metadata_program.key()
    )]
    /// CHECK:
    pub nft_metadata: UncheckedAccount<'info>,
    /// CHECK:
    pub collection_mint: UncheckedAccount<'info>,
    /// CHECK:
    pub collection_metadata: UncheckedAccount<'info>,
    /// CHECK:
    pub collection_master_edition: UncheckedAccount<'info>,
    
    #[account(
        init_if_needed,
        payer = payer,
        space = UserCollection::LEN,
        seeds = [
            PROGRAM_SEED_PREFIX,
            b"user_collection",
            collection_authority.key().as_ref(),
            collection_mint.key().as_ref(),
        ],
        bump,
    )]
    pub user_collection: Account<'info, UserCollection>,
}

#[derive(Accounts)]
pub struct VerifyNFTInCollection<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK:
    pub collection_authority: Signer<'info>,
    /// CHECK:
    #[account(mut)]
    pub nft_metadata: UncheckedAccount<'info>,
    /// CHECK:
    pub collection_mint: UncheckedAccount<'info>,
    /// CHECK:
    pub collection_metadata: UncheckedAccount<'info>,
    /// CHECK:
    pub collection_master_edition: UncheckedAccount<'info>,
    pub metadata_program: Program<'info, Metadata>,
}

#[derive(Accounts)]
pub struct SetAndVerifyCollectionCtx<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK:
    pub update_authority: Signer<'info>,
    /// CHECK:
    pub collection_authority: Signer<'info>,
    /// CHECK:
    #[account(mut)]
    pub nft_metadata: UncheckedAccount<'info>,
    /// CHECK:
    pub collection_mint: UncheckedAccount<'info>,
    /// CHECK:
    pub collection_metadata: UncheckedAccount<'info>,
    /// CHECK:
    pub collection_master_edition: UncheckedAccount<'info>,
    pub metadata_program: Program<'info, Metadata>,
}

#[derive(Accounts)]
pub struct UnverifyNFTFromCollection<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK:
    pub collection_authority: Signer<'info>,
    /// CHECK:
    #[account(mut)]
    pub nft_metadata: UncheckedAccount<'info>,
    /// CHECK:
    pub collection_mint: UncheckedAccount<'info>,
    /// CHECK:
    pub collection_metadata: UncheckedAccount<'info>,
    /// CHECK:
    pub collection_master_edition: UncheckedAccount<'info>,
    pub metadata_program: Program<'info, Metadata>,
}

#[derive(Accounts)]
pub struct VerifyCollectionAuthority<'info> {
    /// CHECK:
    pub collection_authority: Signer<'info>,
    /// CHECK:
    pub collection_metadata: UncheckedAccount<'info>,
}

// FIXED: Enhanced ListNFT context with unique seeds
#[derive(Accounts)]
#[instruction(listing_id: u64)]
pub struct ListNFT<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,
    #[account(
        constraint = mint.decimals == 0 @ crate::errors::ErrorCode::InvalidNFT,
        constraint = mint.supply == 1 @ crate::errors::ErrorCode::InvalidNFT,
    )]
    pub mint: Account<'info, Mint>,
    #[account(
        constraint = seller_token_account.mint == mint.key() @ crate::errors::ErrorCode::InvalidNFT,
        constraint = seller_token_account.owner == seller.key() @ crate::errors::ErrorCode::UnauthorizedSeller,
        constraint = seller_token_account.amount == 1 @ crate::errors::ErrorCode::SellerDoesNotOwnNFT,
    )]
    pub seller_token_account: Account<'info, TokenAccount>,
    /// CHECK: NFT metadata account for collection validation
    #[account(
        seeds = [
            b"metadata".as_ref(),
            anchor_spl::metadata::ID.as_ref(),
            mint.key().as_ref(),
        ],
        bump,
        seeds::program = anchor_spl::metadata::ID
    )]
    pub nft_metadata: UncheckedAccount<'info>,
    #[account(
        init,
        payer = seller,
        space = Listing::LEN,
        seeds = [PROGRAM_SEED_PREFIX, 
                 LISTING_SEED_PREFIX, 
                 crate::ID.as_ref(),
                 mint.key().as_ref(), 
                 seller.key().as_ref(), 
                 listing_id.to_le_bytes().as_ref()],
        bump
    )]
    pub listing: Account<'info, Listing>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateListing<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,
    #[account(
        mut,
        constraint = listing.seller == seller.key(),
    )]
    pub listing: Account<'info, Listing>,
}

#[derive(Accounts)]
pub struct CancelListing<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,
    #[account(
        mut,
        constraint = listing.seller == seller.key(),
    )]
    pub listing: Account<'info, Listing>,
}

#[derive(Accounts)]
pub struct BuyNFT<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,
    /// CHECK:
    #[account(mut)]
    pub seller: UncheckedAccount<'info>,
    /// CHECK:
    #[account(mut)]
    pub marketplace_authority: UncheckedAccount<'info>,
    pub mint: Account<'info, Mint>,
    #[account(
        mut,
        constraint = seller_token_account.mint == mint.key(),
        constraint = seller_token_account.amount == 1,
    )]
    pub seller_token_account: Account<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = buyer,
        associated_token::mint = mint,
        associated_token::authority = buyer,
    )]
    pub buyer_token_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        constraint = listing.mint == mint.key(),
        constraint = listing.seller == seller.key(),
    )]
    pub listing: Account<'info, Listing>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

// FIXED: Enhanced MakeOffer context with unique seeds
#[derive(Accounts)]
#[instruction(offer_id: u64)]
pub struct MakeOffer<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,
    pub mint: Account<'info, Mint>,
    #[account(
        init,
        payer = buyer,
        space = Offer::LEN,
        seeds = [PROGRAM_SEED_PREFIX, 
                 OFFER_SEED_PREFIX, 
                 crate::ID.as_ref(),
                 mint.key().as_ref(), 
                 buyer.key().as_ref(), 
                 offer_id.to_le_bytes().as_ref()],
        bump
    )]
    pub offer: Account<'info, Offer>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AcceptOffer<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,
    /// CHECK:
    #[account(mut)]
    pub buyer: UncheckedAccount<'info>,
    /// CHECK:
    #[account(mut)]
    pub marketplace_authority: UncheckedAccount<'info>,
    pub mint: Account<'info, Mint>,
    #[account(
        mut,
        constraint = seller_token_account.mint == mint.key(),
        constraint = seller_token_account.owner == seller.key(),
        constraint = seller_token_account.amount == 1,
    )]
    pub seller_token_account: Account<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = seller,
        associated_token::mint = mint,
        associated_token::authority = buyer,
    )]
    pub buyer_token_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        constraint = offer.mint == mint.key(),
        constraint = offer.buyer == buyer.key(),
    )]
    pub offer: Account<'info, Offer>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CancelOffer<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,
    #[account(
        mut,
        constraint = offer.buyer == buyer.key(),
    )]
    pub offer: Account<'info, Offer>,
}

// Updated InitializeProgramState context with authority validation
#[derive(Accounts)]
pub struct InitializeProgramState<'info> {
    #[account(
        init,
        payer = admin,
        space = ProgramState::LEN,
        seeds = [PROGRAM_SEED_PREFIX, PROGRAM_STATE_SEED, crate::ID.as_ref()],
        bump
    )]
    pub program_state: Account<'info, ProgramState>,
    
    #[account(mut)]
    pub admin: Signer<'info>,
    
    /// CHECK: This should be the program's upgrade authority or a predefined authority
    /// The caller must provide this account to prove they have the right to initialize
    pub expected_authority: UncheckedAccount<'info>,
    
    pub system_program: Program<'info, System>,
}

// FIXED: Enhanced SetMintingPrice with unique seeds
#[derive(Accounts)]
pub struct SetMintingPrice<'info> {
    #[account(
        mut,
        seeds = [PROGRAM_SEED_PREFIX, PROGRAM_STATE_SEED, crate::ID.as_ref()],
        bump,
        has_one = admin @ crate::errors::ErrorCode::Unauthorized
    )]
    pub program_state: Account<'info, ProgramState>,
    pub admin: Signer<'info>,
}

// NEW: Context for setting cleanup reward
#[derive(Accounts)]
pub struct SetCleanupReward<'info> {
    #[account(
        mut,
        seeds = [PROGRAM_SEED_PREFIX, PROGRAM_STATE_SEED, crate::ID.as_ref()],
        bump,
        has_one = admin @ crate::errors::ErrorCode::Unauthorized
    )]
    pub program_state: Account<'info, ProgramState>,
    pub admin: Signer<'info>,
}

// Events - Enhanced with new cleanup events
#[event]
pub struct NftMinted {
    pub id: u64,
    pub mint: Pubkey,
    pub payer: Pubkey,
    pub authority: Pubkey,
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub royalty_percent: u16,
    pub seller_fee_basis_points: u16,
    pub creators: Vec<CreatorEventData>,
}

#[event]
pub struct CollectionNftMinted {
    pub id_nft: u64,
    pub mint: Pubkey,
    pub payer: Pubkey,
    pub authority: Pubkey,
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub collection: Pubkey,
    pub royalty_percent: u16,
    pub seller_fee_basis_points: u16,
    pub creators: Vec<CreatorEventData>,
}

#[event]
pub struct CollectionCreated {
    pub id_collection: u64,
    pub mint: Pubkey,
    pub authority: Pubkey,
    pub payer: Pubkey,
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub created_at: i64,
}

#[event]
pub struct NftListed {
    pub listing_id: u64,
    pub mint: Pubkey,
    pub seller: Pubkey,
    pub collection_mint: Pubkey,
    pub price: u64,
    pub listed_at: i64,
}

#[event]
pub struct ListingPriceUpdated {
    pub listing: Pubkey,
    pub seller: Pubkey,
    pub old_price: u64,
    pub new_price: u64,
}

#[event]
pub struct ListingCanceled {
    pub listing: Pubkey,
    pub mint: Pubkey,
    pub seller: Pubkey,
}

#[event]
pub struct NftPurchased {
    pub listing: Pubkey,
    pub mint: Pubkey,
    pub buyer: Pubkey,
    pub seller: Pubkey,
    pub price: u64,
    pub marketplace_fee: u64,
}

#[event]
pub struct OfferMade {
    pub offer_id: u64,
    pub mint: Pubkey,
    pub buyer: Pubkey,
    pub price: u64,
    pub expiry_time: i64,
    pub created_at: i64,
}

#[event]
pub struct OfferAccepted {
    pub offer: Pubkey,
    pub mint: Pubkey,
    pub buyer: Pubkey,
    pub seller: Pubkey,
    pub price: u64,
    pub marketplace_fee: u64,
}

#[event]
pub struct OfferCanceled {
    pub offer: Pubkey,
    pub mint: Pubkey,
    pub buyer: Pubkey,
}

#[event]
pub struct ProgramStateInitialized {
    pub admin: Pubkey,
    pub minting_price: u64,
}

#[event]
pub struct CollectionItemCountUpdated {
    pub collection_mint: Pubkey,
    pub total_items: u64,
    pub timestamp: i64,
}

// Add this new struct for events
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct CreatorEventData {
    pub address: Pubkey,
    pub verified: bool,
    pub share: u8,
}

impl From<Creator> for CreatorEventData {
    fn from(creator: Creator) -> Self {
        Self {
            address: creator.address,
            verified: creator.verified,
            share: creator.share,
        }
    }
} 