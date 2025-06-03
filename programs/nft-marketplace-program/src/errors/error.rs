use anchor_lang::prelude::*;

// FIXED: Enhanced error codes with additional cleanup-related errors
#[error_code]
pub enum ErrorCode {
    #[msg("Invalid collection authority")]
    InvalidCollectionAuthority,
    #[msg("Collection verification failed")]
    CollectionVerificationFailed,
    #[msg("NFT is not part of the specified collection")]
    NFTNotInCollection,
    #[msg("Collection does not exist")]
    CollectionDoesNotExist,
    #[msg("Seller does not own the NFT")]
    SellerDoesNotOwnNFT,
    #[msg("Listing is not active")]
    ListingNotActive,
    #[msg("Unauthorized seller")]
    UnauthorizedSeller,
    #[msg("Seller cannot buy their own NFT")]
    SellerCannotBuy,
    #[msg("Offer has expired")]
    OfferExpired,
    #[msg("Offer is not active")]
    OfferNotActive,
    #[msg("Unauthorized buyer")]
    UnauthorizedBuyer,
    #[msg("Insufficient funds")]
    InsufficientFunds,
    #[msg("Only the admin can perform this action")]
    Unauthorized,
    #[msg("Invalid NFT in token account")]
    InvalidNFT,
    #[msg("Unauthorized collection update authority")]
    UnauthorizedCollectionUpdateAuthority,
    #[msg("Arithmetic overflow occurred")]
    ArithmeticOverflow,
    #[msg("Invalid fee basis points - must be <= 10000")]
    InvalidFeeBasisPoints,
    #[msg("Offer expiry time is too long - maximum 1 year")]
    OfferExpiryTooLong,
    #[msg("Unauthorized program initialization - only program authority can initialize")]
    UnauthorizedProgramInitialization,
    #[msg("Royalty percentage too high - maximum 50%")]
    RoyaltyTooHigh,
}
