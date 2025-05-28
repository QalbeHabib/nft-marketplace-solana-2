// use anchor_lang::prelude::*;
// use anchor_spl::associated_token::AssociatedToken;
// use anchor_spl::metadata::{
//     create_master_edition_v3, create_metadata_accounts_v3, CreateMasterEditionV3,
//     CreateMetadataAccountsV3, Metadata, verify_collection, VerifyCollection,
//     set_and_verify_collection, SetAndVerifyCollection, unverify_collection, UnverifyCollection,
// };
// use anchor_spl::token::{mint_to, Mint, MintTo, Token, TokenAccount, transfer, Transfer, close_account, CloseAccount};
// use mpl_token_metadata::types::{Collection, Creator, DataV2};

// declare_id!("48Afa15ypgAHQr7qNm2QqW8WL114Ynwer556CV9chARa");

// #[program]
// pub mod nft_program {
//     use super::*;

//     pub fn create_single_nft(
//         ctx: Context<CreateNFT>,
//         id: u64,
//         name: String,
//         symbol: String,
//         uri: String,
//         price: f32,
//         cant: u64,
//     ) -> Result<()> {
//         msg!("Creating seeds");
//         let id_bytes = id.to_le_bytes();
//         let seeds = &[
//             "mint".as_bytes(),
//             id_bytes.as_ref(),
//             &[ctx.bumps.mint],
//         ];

//         msg!("Run mint_to");

//         mint_to(
//             CpiContext::new_with_signer(
//                 ctx.accounts.token_program.to_account_info(),
//                 MintTo {
//                     authority: ctx.accounts.authority.to_account_info(),
//                     to: ctx.accounts.token_account.to_account_info(),
//                     mint: ctx.accounts.mint.to_account_info(),
//                 },
//                 &[&seeds[..]],
//             ),
//             1, // 1 token
//         )?;

//         msg!("Run create metadata accounts v3");

//         create_metadata_accounts_v3(
//             CpiContext::new_with_signer(
//                 ctx.accounts.metadata_program.to_account_info(),
//                 CreateMetadataAccountsV3 {
//                     payer: ctx.accounts.payer.to_account_info(),
//                     mint: ctx.accounts.mint.to_account_info(),
//                     metadata: ctx.accounts.nft_metadata.to_account_info(),
//                     mint_authority: ctx.accounts.authority.to_account_info(),
//                     update_authority: ctx.accounts.authority.to_account_info(),
//                     system_program: ctx.accounts.system_program.to_account_info(),
//                     rent: ctx.accounts.rent.to_account_info(),
//                 },
//                 &[&seeds[..]],
//             ),
//             DataV2 {
//                 name,
//                 symbol,
//                 uri,
//                 seller_fee_basis_points: 0,
//                 creators: None,
//                 collection: None,
//                 uses: None,
//             },
//             true,
//             true,
//             None,
//         )?;

//         msg!("Run create master edition v3");

//         create_master_edition_v3(
//             CpiContext::new_with_signer(
//                 ctx.accounts.metadata_program.to_account_info(),
//                 CreateMasterEditionV3 {
//                     edition: ctx.accounts.master_edition_account.to_account_info(),
//                     payer: ctx.accounts.payer.to_account_info(),
//                     mint: ctx.accounts.mint.to_account_info(),
//                     metadata: ctx.accounts.nft_metadata.to_account_info(),
//                     mint_authority: ctx.accounts.authority.to_account_info(),
//                     update_authority: ctx.accounts.authority.to_account_info(),
//                     system_program: ctx.accounts.system_program.to_account_info(),
//                     token_program: ctx.accounts.token_program.to_account_info(),
//                     rent: ctx.accounts.rent.to_account_info(),
//                 },
//                 &[&seeds[..]],
//             ),
//             Some(1),
//         )?;

//         msg!("Minted NFT successfully");

//         Ok(())
//     }

//     pub fn mint_to_collection(
//         ctx: Context<MintToCollection>,
//         id_nft: u64,
//         name: String,
//         symbol: String,
//         uri: String,
//         price: f32,
//         cant: u64,
//     ) -> Result<()> {
//         msg!("Creating seeds for NFT in collection");
//         let collection_pubkey_val: Pubkey = *ctx.accounts.collection.key;
//         let collection_pubkey_bytes = collection_pubkey_val.to_bytes();
//         let id_nft_bytes = id_nft.to_le_bytes();
//         let seeds = &[
//             "item_mint".as_bytes(),
//             collection_pubkey_bytes.as_ref(),
//             id_nft_bytes.as_ref(),
//             &[ctx.bumps.mint]
//         ];

//         msg!("Run mint_to");

//         mint_to(
//             CpiContext::new_with_signer(
//                 ctx.accounts.token_program.to_account_info(),
//                 MintTo {
//                     authority: ctx.accounts.authority.to_account_info(),
//                     to: ctx.accounts.token_account.to_account_info(),
//                     mint: ctx.accounts.mint.to_account_info(),
//                 },
//                 &[&seeds[..]],
//             ),
//             1, // 1 token
//         )?;

//         msg!("Run create metadata accounts v3");

//         create_metadata_accounts_v3(
//             CpiContext::new_with_signer(
//                 ctx.accounts.metadata_program.to_account_info(),
//                 CreateMetadataAccountsV3 {
//                     payer: ctx.accounts.payer.to_account_info(),
//                     mint: ctx.accounts.mint.to_account_info(),
//                     metadata: ctx.accounts.nft_metadata.to_account_info(),
//                     mint_authority: ctx.accounts.authority.to_account_info(),
//                     update_authority: ctx.accounts.authority.to_account_info(),
//                     system_program: ctx.accounts.system_program.to_account_info(),
//                     rent: ctx.accounts.rent.to_account_info(),
//                 },
//                 &[&seeds[..]],
//             ),
//             DataV2 {
//                 name,
//                 symbol,
//                 uri,
//                 seller_fee_basis_points: 0,
//                 creators: Some(vec![Creator {
//                     address: ctx.accounts.payer.key(),
//                     verified: true,
//                     share: 100,
//                 }]),
//                 collection: Some(Collection {
//                     key: ctx.accounts.collection.key(),
//                     verified: false,
//                 }),
//                 uses: None,
//             },
//             true,
//             true,
//             None,
//         )?;

//         msg!("Run create master edition v3");

//         create_master_edition_v3(
//             CpiContext::new_with_signer(
//                 ctx.accounts.metadata_program.to_account_info(),
//                 CreateMasterEditionV3 {
//                     edition: ctx.accounts.master_edition_account.to_account_info(),
//                     payer: ctx.accounts.payer.to_account_info(),
//                     mint: ctx.accounts.mint.to_account_info(),
//                     metadata: ctx.accounts.nft_metadata.to_account_info(),
//                     mint_authority: ctx.accounts.authority.to_account_info(),
//                     update_authority: ctx.accounts.authority.to_account_info(),
//                     system_program: ctx.accounts.system_program.to_account_info(),
//                     token_program: ctx.accounts.token_program.to_account_info(),
//                     rent: ctx.accounts.rent.to_account_info(),
//                 },
//                 &[&seeds[..]],
//             ),
//             Some(1),
//         )?;

//         msg!("Minted NFT successfully");

//         Ok(())
//     }

//     pub fn create_collection(
//         ctx: Context<CreateCollection>,
//         id_collection: u64,
//         name: String,
//         symbol: String,
//         uri: String,
//     ) -> Result<()> {
//         msg!("Creating seeds for collection");
//         let id_bytes = id_collection.to_le_bytes();
//         let seeds = &[
//             "mint".as_bytes(),
//             id_bytes.as_ref(),
//             &[ctx.bumps.mint],
//         ];

//         msg!("Run mint_to for collection");
//         mint_to(
//             CpiContext::new_with_signer(
//                 ctx.accounts.token_program.to_account_info(),
//                 MintTo {
//                     authority: ctx.accounts.authority.to_account_info(),
//                     to: ctx.accounts.token_account.to_account_info(),
//                     mint: ctx.accounts.mint.to_account_info(),
//                 },
//                 &[&seeds[..]],
//             ),
//             1, // 1 token for the collection NFT
//         )?;

//         msg!("Run create metadata accounts v3 for collection");
//         create_metadata_accounts_v3(
//             CpiContext::new_with_signer(
//                 ctx.accounts.metadata_program.to_account_info(),
//                 CreateMetadataAccountsV3 {
//                     payer: ctx.accounts.payer.to_account_info(),
//                     mint: ctx.accounts.mint.to_account_info(),
//                     metadata: ctx.accounts.nft_metadata.to_account_info(),
//                     mint_authority: ctx.accounts.authority.to_account_info(),
//                     update_authority: ctx.accounts.authority.to_account_info(),
//                     system_program: ctx.accounts.system_program.to_account_info(),
//                     rent: ctx.accounts.rent.to_account_info(),
//                 },
//                 &[&seeds[..]],
//             ),
//             DataV2 {
//                 name,
//                 symbol,
//                 uri,
//                 seller_fee_basis_points: 0,
//                 creators: Some(vec![Creator { // Typically, the authority creating the collection is a creator
//                     address: ctx.accounts.authority.key(),
//                     verified: true, // The authority is signing, so this can be true
//                     share: 100,
//                 }]),
//                 collection: None, // A collection NFT does not belong to another collection
//                 uses: None,
//             },
//             true, // is_mutable
//             true, // update_authority_is_signer
//             None, // collection_details
//         )?;

//         msg!("Run create master edition v3 for collection");
//         create_master_edition_v3(
//             CpiContext::new_with_signer(
//                 ctx.accounts.metadata_program.to_account_info(),
//                 CreateMasterEditionV3 {
//                     edition: ctx.accounts.master_edition_account.to_account_info(),
//                     payer: ctx.accounts.payer.to_account_info(),
//                     mint: ctx.accounts.mint.to_account_info(),
//                     metadata: ctx.accounts.nft_metadata.to_account_info(),
//                     mint_authority: ctx.accounts.authority.to_account_info(),
//                     update_authority: ctx.accounts.authority.to_account_info(),
//                     system_program: ctx.accounts.system_program.to_account_info(),
//                     token_program: ctx.accounts.token_program.to_account_info(),
//                     rent: ctx.accounts.rent.to_account_info(),
//                 },
//                 &[&seeds[..]],
//             ),
//             Some(0), // Max supply for a collection NFT (non-fungible)
//         )?;

//         msg!("Created collection NFT successfully");
//         Ok(())
//     }

//     // NEW VERIFICATION METHODS BELOW

//     /// Verify an NFT as part of a collection (standard verification)
//     pub fn verify_nft_in_collection(
//         ctx: Context<VerifyNFTInCollection>,
//     ) -> Result<()> {
//         msg!("Verifying NFT in collection");

//         verify_collection(
//             CpiContext::new(
//                 ctx.accounts.metadata_program.to_account_info(),
//                 VerifyCollection {
//                     payer: ctx.accounts.payer.to_account_info(),
//                     metadata: ctx.accounts.nft_metadata.to_account_info(),
//                     collection_authority: ctx.accounts.collection_authority.to_account_info(),
//                     collection_mint: ctx.accounts.collection_mint.to_account_info(),
//                     collection_metadata: ctx.accounts.collection_metadata.to_account_info(),
//                     collection_master_edition: ctx.accounts.collection_master_edition.to_account_info(),
//                 }
//             ),
//             None  // collection_authority_record - typically None for most cases
//         )?;

//         msg!("NFT verified in collection successfully");
//         Ok(())
//     }

//     pub fn set_and_verify_collection(
//         ctx: Context<SetAndVerifyCollectionCtx>,
//         collection_key: Pubkey,
//     ) -> Result<()> {
//         msg!("Setting and verifying collection");

//         anchor_spl::metadata::set_and_verify_collection(
//             CpiContext::new(
//                 ctx.accounts.metadata_program.to_account_info(),
//                 anchor_spl::metadata::SetAndVerifyCollection {
//                     metadata: ctx.accounts.nft_metadata.to_account_info(),
//                     collection_authority: ctx.accounts.collection_authority.to_account_info(),
//                     payer: ctx.accounts.payer.to_account_info(),
//                     update_authority: ctx.accounts.update_authority.to_account_info(),
//                     collection_mint: ctx.accounts.collection_mint.to_account_info(),
//                     collection_metadata: ctx.accounts.collection_metadata.to_account_info(),
//                     collection_master_edition: ctx.accounts.collection_master_edition.to_account_info(),
//                 }
//             ),
//             Some(collection_key)  // Pass the Pubkey directly
//         )?;

//         msg!("Collection set and verified successfully");
//         Ok(())
//     }

//     /// Unverify an NFT from a collection (if needed)
//     pub fn unverify_nft_from_collection(
//         ctx: Context<UnverifyNFTFromCollection>,
//     ) -> Result<()> {
//         msg!("Unverifying NFT from collection");

//         unverify_collection(
//             CpiContext::new(
//                 ctx.accounts.metadata_program.to_account_info(),
//                 UnverifyCollection {
//                     metadata: ctx.accounts.nft_metadata.to_account_info(),
//                     collection_authority: ctx.accounts.collection_authority.to_account_info(),
//                     collection_mint: ctx.accounts.collection_mint.to_account_info(),
//                     collection: ctx.accounts.collection_metadata.to_account_info(),  // Renamed to match struct
//                     collection_master_edition_account: ctx.accounts.collection_master_edition.to_account_info(),  // Renamed to match struct
//                 }
//             ),
//             None  // collection_authority_record - typically None for most cases
//         )?;

//         msg!("NFT unverified from collection successfully");
//         Ok(())
//     }

//     /// Verify collection authority (for batch operations)
//     pub fn verify_collection_authority(
//         ctx: Context<VerifyCollectionAuthority>,
//     ) -> Result<()> {
//         msg!("Verifying collection authority");

//         // Check if the signer is the update authority of the collection
//         require!(
//             ctx.accounts.collection_authority.key() == *ctx.accounts.collection_metadata.to_account_info().owner,
//             ErrorCode::InvalidCollectionAuthority
//         );

//         msg!("Collection authority verified successfully");
//         Ok(())
//     }

//     /// Mint and verify in collection in one transaction (most efficient)
//     pub fn mint_and_verify_to_collection(
//         ctx: Context<MintAndVerifyToCollection>,
//         id_nft: u64,
//         name: String,
//         symbol: String,
//         uri: String,
//         price: f32,
//         cant: u64,
//     ) -> Result<()> {
//         msg!("Minting and verifying NFT in collection");

//         let collection_pubkey_val: Pubkey = *ctx.accounts.collection_mint.key;
//         let collection_pubkey_bytes = collection_pubkey_val.to_bytes();
//         let id_nft_bytes = id_nft.to_le_bytes();
//         let seeds = &[
//             "item_mint".as_bytes(),
//             collection_pubkey_bytes.as_ref(),
//             id_nft_bytes.as_ref(),
//             &[ctx.bumps.mint]
//         ];

//         // Mint token
//         mint_to(
//             CpiContext::new_with_signer(
//                 ctx.accounts.token_program.to_account_info(),
//                 MintTo {
//                     authority: ctx.accounts.authority.to_account_info(),
//                     to: ctx.accounts.token_account.to_account_info(),
//                     mint: ctx.accounts.mint.to_account_info(),
//                 },
//                 &[&seeds[..]],
//             ),
//             1,
//         )?;

//         // Create metadata with verified collection
//         create_metadata_accounts_v3(
//             CpiContext::new_with_signer(
//                 ctx.accounts.metadata_program.to_account_info(),
//                 CreateMetadataAccountsV3 {
//                     payer: ctx.accounts.payer.to_account_info(),
//                     mint: ctx.accounts.mint.to_account_info(),
//                     metadata: ctx.accounts.nft_metadata.to_account_info(),
//                     mint_authority: ctx.accounts.authority.to_account_info(),
//                     update_authority: ctx.accounts.authority.to_account_info(),
//                     system_program: ctx.accounts.system_program.to_account_info(),
//                     rent: ctx.accounts.rent.to_account_info(),
//                 },
//                 &[&seeds[..]],
//             ),
//             DataV2 {
//                 name,
//                 symbol,
//                 uri,
//                 seller_fee_basis_points: 0,
//                 creators: Some(vec![Creator {
//                     address: ctx.accounts.payer.key(),
//                     verified: true,
//                     share: 100,
//                 }]),
//                 collection: Some(Collection {
//                     key: ctx.accounts.collection_mint.key(),
//                     verified: false, // Will be verified in next step
//                 }),
//                 uses: None,
//             },
//             true,
//             true,
//             None,
//         )?;

//         // Create master edition
//         create_master_edition_v3(
//             CpiContext::new_with_signer(
//                 ctx.accounts.metadata_program.to_account_info(),
//                 CreateMasterEditionV3 {
//                     edition: ctx.accounts.master_edition_account.to_account_info(),
//                     payer: ctx.accounts.payer.to_account_info(),
//                     mint: ctx.accounts.mint.to_account_info(),
//                     metadata: ctx.accounts.nft_metadata.to_account_info(),
//                     mint_authority: ctx.accounts.authority.to_account_info(),
//                     update_authority: ctx.accounts.authority.to_account_info(),
//                     system_program: ctx.accounts.system_program.to_account_info(),
//                     token_program: ctx.accounts.token_program.to_account_info(),
//                     rent: ctx.accounts.rent.to_account_info(),
//                 },
//                 &[&seeds[..]],
//             ),
//             Some(1),
//         )?;

//         // Verify collection
//         verify_collection(
//             CpiContext::new(
//                 ctx.accounts.metadata_program.to_account_info(),
//                 VerifyCollection {
//                     payer: ctx.accounts.payer.to_account_info(),
//                     metadata: ctx.accounts.nft_metadata.to_account_info(),
//                     collection_authority: ctx.accounts.collection_authority.to_account_info(),
//                     collection_mint: ctx.accounts.collection_mint.to_account_info(),
//                     collection_metadata: ctx.accounts.collection_metadata.to_account_info(),
//                     collection_master_edition: ctx.accounts.collection_master_edition.to_account_info(),
//                 }
//             ),
//             None  // collection_authority_record - typically None for most cases
//         )?;

//         msg!("NFT minted and verified in collection successfully");
//         Ok(())
//     }

//     // MARKETPLACE LISTING METHODS

//     /// List an NFT for sale
//     pub fn list_nft(
//         ctx: Context<ListNFT>,
//         price: u64, // Price in lamports (1 SOL = 1_000_000_000 lamports)
//         listing_id: u64,
//     ) -> Result<()> {
//         msg!("Listing NFT for sale");

//         let listing = &mut ctx.accounts.listing;
//         let clock = Clock::get()?;

//         // Verify the seller owns the NFT
//         require!(
//             ctx.accounts.seller_token_account.amount == 1,
//             ErrorCode::SellerDoesNotOwnNFT
//         );

//         // Initialize listing data
//         listing.seller = ctx.accounts.seller.key();
//         listing.mint = ctx.accounts.mint.key();
//         listing.price = price;
//         listing.is_active = true;
//         listing.listed_at = clock.unix_timestamp;
//         listing.bump = ctx.bumps.listing;

//         msg!("NFT listed successfully for {} lamports", price);
//         Ok(())
//     }

//     /// Update listing price
//     pub fn update_listing_price(
//         ctx: Context<UpdateListing>,
//         new_price: u64,
//     ) -> Result<()> {
//         msg!("Updating listing price");

//         let listing = &mut ctx.accounts.listing;

//         // Verify listing is active
//         require!(listing.is_active, ErrorCode::ListingNotActive);

//         // Verify seller is the one updating
//         require!(
//             listing.seller == ctx.accounts.seller.key(),
//             ErrorCode::UnauthorizedSeller
//         );

//         listing.price = new_price;

//         msg!("Listing price updated to {} lamports", new_price);
//         Ok(())
//     }

//     /// Cancel/Delist an NFT
//     pub fn cancel_listing(
//         ctx: Context<CancelListing>,
//     ) -> Result<()> {
//         msg!("Canceling NFT listing");

//         let listing = &mut ctx.accounts.listing;

//         // Verify listing is active
//         require!(listing.is_active, ErrorCode::ListingNotActive);

//         // Verify seller is the one canceling
//         require!(
//             listing.seller == ctx.accounts.seller.key(),
//             ErrorCode::UnauthorizedSeller
//         );

//         listing.is_active = false;

//         msg!("NFT listing canceled successfully");
//         Ok(())
//     }

//     /// Buy an NFT from listing
//     pub fn buy_nft(
//         ctx: Context<BuyNFT>,
//         marketplace_fee_bps: u16, // Basis points (e.g., 250 = 2.5%)
//     ) -> Result<()> {
//         msg!("Buying NFT from listing");

//         let listing = &mut ctx.accounts.listing;

//         // Verify listing is active
//         require!(listing.is_active, ErrorCode::ListingNotActive);

//         // Verify buyer is not the seller
//         require!(
//             listing.seller != ctx.accounts.buyer.key(),
//             ErrorCode::SellerCannotBuy
//         );

//         // Calculate fees
//         let total_price = listing.price;
//         let marketplace_fee = (total_price as u128)
//             .checked_mul(marketplace_fee_bps as u128)
//             .unwrap()
//             .checked_div(10000)
//             .unwrap() as u64;
//         let seller_amount = total_price.checked_sub(marketplace_fee).unwrap();

//         // Transfer SOL from buyer to seller
//         let transfer_instruction = anchor_lang::system_program::Transfer {
//             from: ctx.accounts.buyer.to_account_info(),
//             to: ctx.accounts.seller.to_account_info(),
//         };
//         anchor_lang::system_program::transfer(
//             CpiContext::new(
//                 ctx.accounts.system_program.to_account_info(),
//                 transfer_instruction,
//             ),
//             seller_amount,
//         )?;

//         // Transfer marketplace fee (if any)
//         if marketplace_fee > 0 {
//             let fee_transfer_instruction = anchor_lang::system_program::Transfer {
//                 from: ctx.accounts.buyer.to_account_info(),
//                 to: ctx.accounts.marketplace_authority.to_account_info(),
//             };
//             anchor_lang::system_program::transfer(
//                 CpiContext::new(
//                     ctx.accounts.system_program.to_account_info(),
//                     fee_transfer_instruction,
//                 ),
//                 marketplace_fee,
//             )?;
//         }

//         // Transfer NFT from seller to buyer
//         let cpi_accounts = Transfer {
//             from: ctx.accounts.seller_token_account.to_account_info(),
//             to: ctx.accounts.buyer_token_account.to_account_info(),
//             authority: ctx.accounts.seller.to_account_info(),
//         };
//         let cpi_program = ctx.accounts.token_program.to_account_info();
//         let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
//         transfer(cpi_ctx, 1)?;

//         // Mark listing as inactive
//         listing.is_active = false;

//         msg!("NFT purchased successfully for {} lamports", total_price);
//         Ok(())
//     }

//     /// Make an offer on an NFT
//     pub fn make_offer(
//         ctx: Context<MakeOffer>,
//         offer_price: u64,
//         expiry_time: i64, // Unix timestamp
//         offer_id: u64,
//     ) -> Result<()> {
//         msg!("Making offer on NFT");

//         let offer = &mut ctx.accounts.offer;
//         let clock = Clock::get()?;

//         // Verify offer hasn't expired
//         require!(
//             expiry_time > clock.unix_timestamp,
//             ErrorCode::OfferExpired
//         );

//         // Verify buyer has enough SOL
//         require!(
//             ctx.accounts.buyer.lamports() >= offer_price,
//             ErrorCode::InsufficientFunds
//         );

//         offer.buyer = ctx.accounts.buyer.key();
//         offer.mint = ctx.accounts.mint.key();
//         offer.price = offer_price;
//         offer.expiry_time = expiry_time;
//         offer.is_active = true;
//         offer.created_at = clock.unix_timestamp;
//         offer.bump = ctx.bumps.offer;

//         msg!("Offer made for {} lamports", offer_price);
//         Ok(())
//     }

//     /// Accept an offer
//     pub fn accept_offer(
//         ctx: Context<AcceptOffer>,
//         marketplace_fee_bps: u16,
//     ) -> Result<()> {
//         msg!("Accepting offer");

//         let offer = &mut ctx.accounts.offer;
//         let clock = Clock::get()?;

//         // Verify offer is active and not expired
//         require!(offer.is_active, ErrorCode::OfferNotActive);
//         require!(
//             offer.expiry_time > clock.unix_timestamp,
//             ErrorCode::OfferExpired
//         );

//         // Verify seller owns the NFT
//         require!(
//             ctx.accounts.seller_token_account.amount == 1,
//             ErrorCode::SellerDoesNotOwnNFT
//         );

//         // Calculate fees
//         let total_price = offer.price;
//         let marketplace_fee = (total_price as u128)
//             .checked_mul(marketplace_fee_bps as u128)
//             .unwrap()
//             .checked_div(10000)
//             .unwrap() as u64;
//         let seller_amount = total_price.checked_sub(marketplace_fee).unwrap();

//         // Transfer SOL from buyer to seller
//         let transfer_instruction = anchor_lang::system_program::Transfer {
//             from: ctx.accounts.buyer.to_account_info(),
//             to: ctx.accounts.seller.to_account_info(),
//         };
//         anchor_lang::system_program::transfer(
//             CpiContext::new(
//                 ctx.accounts.system_program.to_account_info(),
//                 transfer_instruction,
//             ),
//             seller_amount,
//         )?;

//         // Transfer marketplace fee (if any)
//         if marketplace_fee > 0 {
//             let fee_transfer_instruction = anchor_lang::system_program::Transfer {
//                 from: ctx.accounts.buyer.to_account_info(),
//                 to: ctx.accounts.marketplace_authority.to_account_info(),
//             };
//             anchor_lang::system_program::transfer(
//                 CpiContext::new(
//                     ctx.accounts.system_program.to_account_info(),
//                     fee_transfer_instruction,
//                 ),
//                 marketplace_fee,
//             )?;
//         }

//         // Transfer NFT from seller to buyer
//         let cpi_accounts = Transfer {
//             from: ctx.accounts.seller_token_account.to_account_info(),
//             to: ctx.accounts.buyer_token_account.to_account_info(),
//             authority: ctx.accounts.seller.to_account_info(),
//         };
//         let cpi_program = ctx.accounts.token_program.to_account_info();
//         let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
//         transfer(cpi_ctx, 1)?;

//         // Mark offer as inactive
//         offer.is_active = false;

//         msg!("Offer accepted for {} lamports", total_price);
//         Ok(())
//     }

//     /// Cancel an offer
//     pub fn cancel_offer(
//         ctx: Context<CancelOffer>,
//     ) -> Result<()> {
//         msg!("Canceling offer");

//         let offer = &mut ctx.accounts.offer;

//         // Verify offer is active
//         require!(offer.is_active, ErrorCode::OfferNotActive);

//         // Verify buyer is the one canceling
//         require!(
//             offer.buyer == ctx.accounts.buyer.key(),
//             ErrorCode::UnauthorizedBuyer
//         );

//         offer.is_active = false;

//         msg!("Offer canceled successfully");
//         Ok(())
//     }
// }

// // EXISTING ACCOUNT STRUCTURES (unchanged)

// #[derive(Accounts)]
// #[instruction(id: u64)]
// pub struct CreateNFT<'info> {
//     #[account(mut)]
//     pub authority: Signer<'info>,
//     #[account(mut)]
//     pub payer: Signer<'info>,
//     #[account(
//     init,
//     payer = payer,
//     mint::decimals = 0,
//     mint::authority = authority,
//     mint::freeze_authority = authority,
//     seeds = ["mint".as_bytes(), id.to_le_bytes().as_ref()],
//     bump,
//     )]
//     pub mint: Account<'info, Mint>,
//     #[account(
//         init_if_needed,
//         payer = payer,
//         associated_token::mint = mint,
//         associated_token::authority = payer,
//     )]
//     pub token_account: Account<'info, TokenAccount>,
//     pub associated_token_program: Program<'info, AssociatedToken>,
//     pub rent: Sysvar<'info, Rent>,
//     pub system_program: Program<'info, System>,
//     pub token_program: Program<'info, Token>,
//     pub metadata_program: Program<'info, Metadata>,
//     #[account(
//         mut,
//         seeds = [
//             b"metadata".as_ref(),
//             metadata_program.key().as_ref(),
//             mint.key().as_ref(),
//             b"edition".as_ref(),
//         ],
//         bump,
//         seeds::program = metadata_program.key()
//     )]
//     /// CHECK:
//     pub master_edition_account: UncheckedAccount<'info>,
//     #[account(
//         mut,
//         seeds = [
//             b"metadata".as_ref(),
//             metadata_program.key().as_ref(),
//             mint.key().as_ref(),
//         ],
//         bump,
//         seeds::program = metadata_program.key()
//     )]
//     /// CHECK:
//     pub nft_metadata: UncheckedAccount<'info>,
// }

// #[derive(Accounts)]
// #[instruction(id_nft: u64)]
// pub struct MintToCollection<'info> {
//     #[account(mut)]
//     pub authority: Signer<'info>,
//     #[account(mut)]
//     pub payer: Signer<'info>,
//     #[account(
//     init,
//     payer = payer,
//     mint::decimals = 0,
//     mint::authority = authority,
//     mint::freeze_authority = authority,
//     seeds = ["item_mint".as_bytes(),
//              collection.key().as_ref(),
//              id_nft.to_le_bytes().as_ref()],
//     bump,
//     )]
//     pub mint: Account<'info, Mint>,
//     #[account(
//         init_if_needed,
//         payer = payer,
//         associated_token::mint = mint,
//         associated_token::authority = payer,
//     )]
//     pub token_account: Account<'info, TokenAccount>,
//     pub associated_token_program: Program<'info, AssociatedToken>,
//     pub rent: Sysvar<'info, Rent>,
//     pub system_program: Program<'info, System>,
//     pub token_program: Program<'info, Token>,
//     pub metadata_program: Program<'info, Metadata>,
//     #[account(
//         mut,
//         seeds = [
//             b"metadata".as_ref(),
//             metadata_program.key().as_ref(),
//             mint.key().as_ref(),
//             b"edition".as_ref(),
//         ],
//         bump,
//         seeds::program = metadata_program.key()
//     )]
//     /// CHECK:
//     pub master_edition_account: UncheckedAccount<'info>,
//     #[account(
//         mut,
//         seeds = [
//             b"metadata".as_ref(),
//             metadata_program.key().as_ref(),
//             mint.key().as_ref(),
//         ],
//         bump,
//         seeds::program = metadata_program.key()
//     )]
//     /// CHECK:
//     pub nft_metadata: UncheckedAccount<'info>,
//     /// CHECK:
//     pub collection: UncheckedAccount<'info>,
// }

// #[derive(Accounts)]
// #[instruction(id_collection: u64)]
// pub struct CreateCollection<'info> {
//     #[account(mut)]
//     pub authority: Signer<'info>,
//     #[account(mut)]
//     pub payer: Signer<'info>,
//     #[account(
//         init,
//         payer = payer,
//         mint::decimals = 0,
//         mint::authority = authority,
//         mint::freeze_authority = authority,
//         seeds = ["mint".as_bytes(), id_collection.to_le_bytes().as_ref()],
//         bump
//     )]
//     pub mint: Account<'info, Mint>,
//     #[account(
//         init_if_needed,
//         payer = payer,
//         associated_token::mint = mint,
//         associated_token::authority = payer, // Payer will own the collection NFT token
//     )]
//     pub token_account: Account<'info, TokenAccount>,
//     pub associated_token_program: Program<'info, AssociatedToken>,
//     pub rent: Sysvar<'info, Rent>,
//     pub system_program: Program<'info, System>,
//     pub token_program: Program<'info, Token>,
//     pub metadata_program: Program<'info, Metadata>,
//     #[account(
//         mut,
//         seeds = [
//             b"metadata".as_ref(),
//             metadata_program.key().as_ref(),
//             mint.key().as_ref(),
//             b"edition".as_ref(),
//         ],
//         bump,
//         seeds::program = metadata_program.key()
//     )]
//     /// CHECK: Handled by metaplex
//     pub master_edition_account: UncheckedAccount<'info>,
//     #[account(
//         mut,
//         seeds = [
//             b"metadata".as_ref(),
//             metadata_program.key().as_ref(),
//             mint.key().as_ref(),
//         ],
//         bump,
//         seeds::program = metadata_program.key()
//     )]
//     /// CHECK: Handled by metaplex
//     pub nft_metadata: UncheckedAccount<'info>,
// }

// // NEW ACCOUNT STRUCTURES FOR VERIFICATION

// #[derive(Accounts)]
// pub struct VerifyNFTInCollection<'info> {
//     #[account(mut)]
//     pub payer: Signer<'info>,
//     /// CHECK: Collection authority that can verify NFTs
//     pub collection_authority: Signer<'info>,
//     /// CHECK: NFT metadata account
//     #[account(mut)]
//     pub nft_metadata: UncheckedAccount<'info>,
//     /// CHECK: Collection mint account
//     pub collection_mint: UncheckedAccount<'info>,
//     /// CHECK: Collection metadata account
//     pub collection_metadata: UncheckedAccount<'info>,
//     /// CHECK: Collection master edition account
//     pub collection_master_edition: UncheckedAccount<'info>,
//     pub metadata_program: Program<'info, Metadata>,
// }

// #[derive(Accounts)]
// pub struct SetAndVerifyCollectionCtx<'info> {
//     #[account(mut)]
//     pub payer: Signer<'info>,
//     /// CHECK: Update authority for the NFT
//     pub update_authority: Signer<'info>,
//     /// CHECK: Collection authority that can verify NFTs
//     pub collection_authority: Signer<'info>,
//     /// CHECK: NFT metadata account
//     #[account(mut)]
//     pub nft_metadata: UncheckedAccount<'info>,
//     /// CHECK: Collection mint account
//     pub collection_mint: UncheckedAccount<'info>,
//     /// CHECK: Collection metadata account
//     pub collection_metadata: UncheckedAccount<'info>,
//     /// CHECK: Collection master edition account
//     pub collection_master_edition: UncheckedAccount<'info>,
//     pub metadata_program: Program<'info, Metadata>,
// }

// #[derive(Accounts)]
// pub struct UnverifyNFTFromCollection<'info> {
//     #[account(mut)]
//     pub payer: Signer<'info>,
//     /// CHECK: Collection authority that can unverify NFTs
//     pub collection_authority: Signer<'info>,
//     /// CHECK: NFT metadata account
//     #[account(mut)]
//     pub nft_metadata: UncheckedAccount<'info>,
//     /// CHECK: Collection mint account
//     pub collection_mint: UncheckedAccount<'info>,
//     /// CHECK: Collection metadata account
//     pub collection_metadata: UncheckedAccount<'info>,
//     /// CHECK: Collection master edition account
//     pub collection_master_edition: UncheckedAccount<'info>,
//     pub metadata_program: Program<'info, Metadata>,
// }

// #[derive(Accounts)]
// pub struct VerifyCollectionAuthority<'info> {
//     /// CHECK: Collection authority to verify
//     pub collection_authority: Signer<'info>,
//     /// CHECK: Collection metadata account
//     pub collection_metadata: UncheckedAccount<'info>,
// }

// #[derive(Accounts)]
// #[instruction(id_nft: u64)]
// pub struct MintAndVerifyToCollection<'info> {
//     #[account(mut)]
//     pub authority: Signer<'info>,
//     #[account(mut)]
//     pub payer: Signer<'info>,
//     /// CHECK: Collection authority (must be signer for verification)
//     pub collection_authority: Signer<'info>,
//     #[account(
//         init,
//         payer = payer,
//         mint::decimals = 0,
//         mint::authority = authority,
//         mint::freeze_authority = authority,
//         seeds = ["item_mint".as_bytes(),
//                  collection_mint.key().as_ref(),
//                  id_nft.to_le_bytes().as_ref()],
//         bump,
//     )]
//     pub mint: Account<'info, Mint>,
//     #[account(
//         init_if_needed,
//         payer = payer,
//         associated_token::mint = mint,
//         associated_token::authority = payer,
//     )]
//     pub token_account: Account<'info, TokenAccount>,
//     pub associated_token_program: Program<'info, AssociatedToken>,
//     pub rent: Sysvar<'info, Rent>,
//     pub system_program: Program<'info, System>,
//     pub token_program: Program<'info, Token>,
//     pub metadata_program: Program<'info, Metadata>,
//     #[account(
//         mut,
//         seeds = [
//             b"metadata".as_ref(),
//             metadata_program.key().as_ref(),
//             mint.key().as_ref(),
//             b"edition".as_ref(),
//         ],
//         bump,
//         seeds::program = metadata_program.key()
//     )]
//     /// CHECK:
//     pub master_edition_account: UncheckedAccount<'info>,
//     #[account(
//         mut,
//         seeds = [
//             b"metadata".as_ref(),
//             metadata_program.key().as_ref(),
//             mint.key().as_ref(),
//         ],
//         bump,
//         seeds::program = metadata_program.key()
//     )]
//     /// CHECK:
//     pub nft_metadata: UncheckedAccount<'info>,
//     /// CHECK: Collection mint account
//     pub collection_mint: UncheckedAccount<'info>,
//     /// CHECK: Collection metadata account
//     pub collection_metadata: UncheckedAccount<'info>,
//     /// CHECK: Collection master edition account
//     pub collection_master_edition: UncheckedAccount<'info>,
// }

// // MARKETPLACE DATA STRUCTURES

// #[account]
// pub struct Listing {
//     pub seller: Pubkey,        // 32
//     pub mint: Pubkey,          // 32
//     pub price: u64,            // 8
//     pub is_active: bool,       // 1
//     pub listed_at: i64,        // 8
//     pub bump: u8,              // 1
// }

// impl Listing {
//     pub const LEN: usize = 8 + 32 + 32 + 8 + 1 + 8 + 1; // 90 bytes + discriminator
// }

// #[account]
// pub struct Offer {
//     pub buyer: Pubkey,         // 32
//     pub mint: Pubkey,          // 32
//     pub price: u64,            // 8
//     pub expiry_time: i64,      // 8
//     pub is_active: bool,       // 1
//     pub created_at: i64,       // 8
//     pub bump: u8,              // 1
// }

// impl Offer {
//     pub const LEN: usize = 8 + 32 + 32 + 8 + 8 + 1 + 8 + 1; // 98 bytes + discriminator
// }

// // MARKETPLACE ACCOUNT STRUCTURES

// #[derive(Accounts)]
// #[instruction(listing_id: u64)]
// pub struct ListNFT<'info> {
//     #[account(mut)]
//     pub seller: Signer<'info>,
//     pub mint: Account<'info, Mint>,
//     #[account(
//         constraint = seller_token_account.mint == mint.key(),
//         constraint = seller_token_account.owner == seller.key(),
//     )]
//     pub seller_token_account: Account<'info, TokenAccount>,
//     #[account(
//         init,
//         payer = seller,
//         space = Listing::LEN,
//         seeds = [b"listing", mint.key().as_ref(), seller.key().as_ref(), listing_id.to_le_bytes().as_ref()],
//         bump
//     )]
//     pub listing: Account<'info, Listing>,
//     pub system_program: Program<'info, System>,
// }

// #[derive(Accounts)]
// pub struct UpdateListing<'info> {
//     #[account(mut)]
//     pub seller: Signer<'info>,
//     #[account(
//         mut,
//         constraint = listing.seller == seller.key(),
//     )]
//     pub listing: Account<'info, Listing>,
// }

// #[derive(Accounts)]
// pub struct CancelListing<'info> {
//     #[account(mut)]
//     pub seller: Signer<'info>,
//     #[account(
//         mut,
//         constraint = listing.seller == seller.key(),
//     )]
//     pub listing: Account<'info, Listing>,
// }

// #[derive(Accounts)]
// pub struct BuyNFT<'info> {
//     #[account(mut)]
//     pub buyer: Signer<'info>,
//     /// CHECK: Seller account for SOL transfer
//     #[account(mut)]
//     pub seller: UncheckedAccount<'info>,
//     /// CHECK: Marketplace authority for fee collection
//     #[account(mut)]
//     pub marketplace_authority: UncheckedAccount<'info>,
//     pub mint: Account<'info, Mint>,
//     #[account(
//         mut,
//         constraint = seller_token_account.mint == mint.key(),
//         constraint = seller_token_account.amount == 1,
//     )]
//     pub seller_token_account: Account<'info, TokenAccount>,
//     #[account(
//         init_if_needed,
//         payer = buyer,
//         associated_token::mint = mint,
//         associated_token::authority = buyer,
//     )]
//     pub buyer_token_account: Account<'info, TokenAccount>,
//     #[account(
//         mut,
//         constraint = listing.mint == mint.key(),
//         constraint = listing.seller == seller.key(),
//     )]
//     pub listing: Account<'info, Listing>,
//     pub associated_token_program: Program<'info, AssociatedToken>,
//     pub token_program: Program<'info, Token>,
//     pub system_program: Program<'info, System>,
// }

// #[derive(Accounts)]
// #[instruction(offer_id: u64)]
// pub struct MakeOffer<'info> {
//     #[account(mut)]
//     pub buyer: Signer<'info>,
//     pub mint: Account<'info, Mint>,
//     #[account(
//         init,
//         payer = buyer,
//         space = Offer::LEN,
//         seeds = [b"offer", mint.key().as_ref(), buyer.key().as_ref(), offer_id.to_le_bytes().as_ref()],
//         bump
//     )]
//     pub offer: Account<'info, Offer>,
//     pub system_program: Program<'info, System>,
// }

// #[derive(Accounts)]
// pub struct AcceptOffer<'info> {
//     #[account(mut)]
//     pub seller: Signer<'info>,
//     /// CHECK: Buyer account for SOL transfer
//     #[account(mut)]
//     pub buyer: UncheckedAccount<'info>,
//     /// CHECK: Marketplace authority for fee collection
//     #[account(mut)]
//     pub marketplace_authority: UncheckedAccount<'info>,
//     pub mint: Account<'info, Mint>,
//     #[account(
//         mut,
//         constraint = seller_token_account.mint == mint.key(),
//         constraint = seller_token_account.owner == seller.key(),
//         constraint = seller_token_account.amount == 1,
//     )]
//     pub seller_token_account: Account<'info, TokenAccount>,
//     #[account(
//         init_if_needed,
//         payer = seller,
//         associated_token::mint = mint,
//         associated_token::authority = buyer,
//     )]
//     pub buyer_token_account: Account<'info, TokenAccount>,
//     #[account(
//         mut,
//         constraint = offer.mint == mint.key(),
//         constraint = offer.buyer == buyer.key(),
//     )]
//     pub offer: Account<'info, Offer>,
//     pub associated_token_program: Program<'info, AssociatedToken>,
//     pub token_program: Program<'info, Token>,
//     pub system_program: Program<'info, System>,
// }

// #[derive(Accounts)]
// pub struct CancelOffer<'info> {
//     #[account(mut)]
//     pub buyer: Signer<'info>,
//     #[account(
//         mut,
//         constraint = offer.buyer == buyer.key(),
//     )]
//     pub offer: Account<'info, Offer>,
// }

// // ERROR CODES
// #[error_code]
// pub enum ErrorCode {
//     #[msg("Invalid collection authority")]
//     InvalidCollectionAuthority,
//     #[msg("Collection verification failed")]
//     CollectionVerificationFailed,
//     #[msg("NFT is not part of the specified collection")]
//     NFTNotInCollection,
//     #[msg("Collection does not exist")]
//     CollectionDoesNotExist,
//     #[msg("Seller does not own the NFT")]
//     SellerDoesNotOwnNFT,
//     #[msg("Listing is not active")]
//     ListingNotActive,
//     #[msg("Unauthorized seller")]
//     UnauthorizedSeller,
//     #[msg("Seller cannot buy their own NFT")]
//     SellerCannotBuy,
//     #[msg("Offer has expired")]
//     OfferExpired,
//     #[msg("Offer is not active")]
//     OfferNotActive,
//     #[msg("Unauthorized buyer")]
//     UnauthorizedBuyer,
//     #[msg("Insufficient funds")]
//     InsufficientFunds,
// }
