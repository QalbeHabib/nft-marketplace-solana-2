// use anchor_lang::prelude::*;
// use anchor_spl::associated_token::AssociatedToken;
// use anchor_spl::metadata::{
//     create_master_edition_v3, create_metadata_accounts_v3, set_and_verify_collection,
//     unverify_collection, verify_collection, CreateMasterEditionV3, CreateMetadataAccountsV3,
//     Metadata, SetAndVerifyCollection, UnverifyCollection, VerifyCollection,
// };
// use anchor_spl::token::{
//     close_account, mint_to, transfer, CloseAccount, Mint, MintTo, Token, TokenAccount, Transfer,
// };
// use mpl_token_metadata::accounts::Metadata as MetadataAccount;
// use mpl_token_metadata::types::{Collection, Creator, DataV2};

// declare_id!("GBRUTbNjxd7L8pSw14FEfsGPKkVz8rRhKyWiFFh4xkVC");

// #[event]
// pub struct NftMinted {
//     pub id: u64,
//     pub mint: Pubkey,
//     pub payer: Pubkey,
//     pub authority: Pubkey,
//     pub name: String,
//     pub symbol: String,
//     pub uri: String,
// }

// #[event]
// pub struct CollectionNftMinted {
//     pub id_nft: u64,
//     pub mint: Pubkey,
//     pub payer: Pubkey,
//     pub authority: Pubkey,
//     pub name: String,
//     pub symbol: String,
//     pub uri: String,
//     pub collection: Pubkey,
// }

// #[event]
// pub struct CollectionMinted {
//     pub id_collection: u64,
//     pub mint: Pubkey,
//     pub payer: Pubkey,
//     pub authority: Pubkey,
//     pub name: String,
//     pub symbol: String,
//     pub uri: String,
// }

// #[event]
// pub struct NftListed {
//     pub listing_id: u64,
//     pub mint: Pubkey,
//     pub seller: Pubkey,
//     pub price: u64,
//     pub listed_at: i64,
// }

// #[event]
// pub struct ListingPriceUpdated {
//     pub listing: Pubkey,
//     pub seller: Pubkey,
//     pub old_price: u64,
//     pub new_price: u64,
// }

// #[event]
// pub struct ListingCanceled {
//     pub listing: Pubkey,
//     pub mint: Pubkey,
//     pub seller: Pubkey,
// }

// #[event]
// pub struct NftPurchased {
//     pub listing: Pubkey,
//     pub mint: Pubkey,
//     pub buyer: Pubkey,
//     pub seller: Pubkey,
//     pub price: u64,
//     pub marketplace_fee: u64,
// }

// #[event]
// pub struct OfferMade {
//     pub offer_id: u64,
//     pub mint: Pubkey,
//     pub buyer: Pubkey,
//     pub price: u64,
//     pub expiry_time: i64,
//     pub created_at: i64,
// }

// #[event]
// pub struct OfferAccepted {
//     pub offer: Pubkey,
//     pub mint: Pubkey,
//     pub buyer: Pubkey,
//     pub seller: Pubkey,
//     pub price: u64,
//     pub marketplace_fee: u64,
// }

// #[event]
// pub struct OfferCanceled {
//     pub offer: Pubkey,
//     pub mint: Pubkey,
//     pub buyer: Pubkey,
// }

// #[program]
// pub mod nft_program {
//     use super::*;

//     pub fn create_single_nft(
//         ctx: Context<CreateNFT>,
//         id: u64,
//         name: String,
//         symbol: String,
//         uri: String,
//     ) -> Result<()> {
//         // Enforce minting price payment
//         let minting_price = ctx.accounts.program_state.minting_price;
//         let transfer_instruction = anchor_lang::system_program::Transfer {
//             from: ctx.accounts.payer.to_account_info(),
//             to: ctx.accounts.mint_fee_account.to_account_info(),
//         };
//         anchor_lang::system_program::transfer(
//             CpiContext::new(
//                 ctx.accounts.system_program.to_account_info(),
//                 transfer_instruction,
//             ),
//             minting_price,
//         )?;

//         msg!("Creating seeds");
//         let id_bytes = id.to_le_bytes();
//         let seeds = &["mint".as_bytes(), id_bytes.as_ref(), &[ctx.bumps.mint]];

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
//             1,
//         )?;

//         msg!("Run create metadata accounts v3");
//         let name_for_metadata = name.clone();
//         let symbol_for_metadata = symbol.clone();
//         let uri_for_metadata = uri.clone();
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
//                 name: name_for_metadata,
//                 symbol: symbol_for_metadata,
//                 uri: uri_for_metadata,
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
//         emit!(NftMinted {
//             id,
//             mint: ctx.accounts.mint.key(),
//             payer: ctx.accounts.payer.key(),
//             authority: ctx.accounts.authority.key(),
//             name,
//             symbol,
//             uri,
//         });
//         Ok(())
//     }

//     pub fn mint_to_collection(
//         ctx: Context<MintToCollection>,
//         id_nft: u64,
//         name: String,
//         symbol: String,
//         uri: String,
//     ) -> Result<()> {
//         // Enforce minting price payment
//         let minting_price = ctx.accounts.program_state.minting_price;
//         let transfer_instruction = anchor_lang::system_program::Transfer {
//             from: ctx.accounts.payer.to_account_info(),
//             to: ctx.accounts.mint_fee_account.to_account_info(),
//         };
//         anchor_lang::system_program::transfer(
//             CpiContext::new(
//                 ctx.accounts.system_program.to_account_info(),
//                 transfer_instruction,
//             ),
//             minting_price,
//         )?;

//         msg!("Creating seeds for NFT in collection");
//         let collection_pubkey_val: Pubkey = *ctx.accounts.collection.key;
//         let collection_pubkey_bytes = collection_pubkey_val.to_bytes();
//         let id_nft_bytes = id_nft.to_le_bytes();
//         let seeds = &[
//             "item_mint".as_bytes(),
//             collection_pubkey_bytes.as_ref(),
//             id_nft_bytes.as_ref(),
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
//             1,
//         )?;

//         msg!("Run create metadata accounts v3");
//         let name_for_metadata = name.clone();
//         let symbol_for_metadata = symbol.clone();
//         let uri_for_metadata = uri.clone();
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
//                 name: name_for_metadata,
//                 symbol: symbol_for_metadata,
//                 uri: uri_for_metadata,
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
//         emit!(CollectionNftMinted {
//             id_nft,
//             mint: ctx.accounts.mint.key(),
//             payer: ctx.accounts.payer.key(),
//             authority: ctx.accounts.authority.key(),
//             name,
//             symbol,
//             uri,
//             collection: ctx.accounts.collection.key(),
//         });
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
//         let seeds = &["mint".as_bytes(), id_bytes.as_ref(), &[ctx.bumps.mint]];

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
//             1,
//         )?;

//         msg!("Run create metadata accounts v3 for collection");
//         let name_for_metadata = name.clone();
//         let symbol_for_metadata = symbol.clone();
//         let uri_for_metadata = uri.clone();
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
//                 name: name_for_metadata,
//                 symbol: symbol_for_metadata,
//                 uri: uri_for_metadata,
//                 seller_fee_basis_points: 0,
//                 creators: Some(vec![Creator {
//                     address: ctx.accounts.authority.key(),
//                     verified: true,
//                     share: 100,
//                 }]),
//                 collection: None,
//                 uses: None,
//             },
//             true,
//             true,
//             None,
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
//             Some(0),
//         )?;

//         msg!("Created collection NFT successfully");
//         emit!(CollectionMinted {
//             id_collection,
//             mint: ctx.accounts.mint.key(),
//             payer: ctx.accounts.payer.key(),
//             authority: ctx.accounts.authority.key(),
//             name,
//             symbol,
//             uri,
//         });
//         Ok(())
//     }

//     pub fn verify_nft_in_collection(ctx: Context<VerifyNFTInCollection>) -> Result<()> {
//         msg!("Verifying NFT in collection");

//         // Deserialize collection metadata account
//         let collection_metadata_account =
//             MetadataAccount::try_from(&ctx.accounts.collection_metadata.to_account_info())?;

//         // Verify collection authority matches update authority in collection metadata
//         require!(
//             ctx.accounts.collection_authority.key() == collection_metadata_account.update_authority,
//             ErrorCode::UnauthorizedCollectionUpdateAuthority
//         );

//         verify_collection(
//             CpiContext::new(
//                 ctx.accounts.metadata_program.to_account_info(),
//                 VerifyCollection {
//                     payer: ctx.accounts.payer.to_account_info(),
//                     metadata: ctx.accounts.nft_metadata.to_account_info(),
//                     collection_authority: ctx.accounts.collection_authority.to_account_info(),
//                     collection_mint: ctx.accounts.collection_mint.to_account_info(),
//                     collection_metadata: ctx.accounts.collection_metadata.to_account_info(),
//                     collection_master_edition: ctx
//                         .accounts
//                         .collection_master_edition
//                         .to_account_info(),
//                 },
//             ),
//             None,
//         )?;
//         msg!("NFT verified in collection successfully");
//         Ok(())
//     }

//     pub fn set_and_verify_collection(
//         ctx: Context<SetAndVerifyCollectionCtx>,
//         collection_key: Pubkey,
//     ) -> Result<()> {
//         msg!("Setting and verifying collection");

//         // Deserialize collection metadata account
//         let collection_metadata_account =
//             MetadataAccount::try_from(&ctx.accounts.collection_metadata.to_account_info())?;

//         // Verify collection authority matches update authority in collection metadata
//         require!(
//             ctx.accounts.collection_authority.key() == collection_metadata_account.update_authority,
//             ErrorCode::UnauthorizedCollectionUpdateAuthority
//         );

//         anchor_spl::metadata::set_and_verify_collection(
//             CpiContext::new(
//                 ctx.accounts.metadata_program.to_account_info(),
//                 SetAndVerifyCollection {
//                     metadata: ctx.accounts.nft_metadata.to_account_info(),
//                     collection_authority: ctx.accounts.collection_authority.to_account_info(),
//                     payer: ctx.accounts.payer.to_account_info(),
//                     update_authority: ctx.accounts.update_authority.to_account_info(),
//                     collection_mint: ctx.accounts.collection_mint.to_account_info(),
//                     collection_metadata: ctx.accounts.collection_metadata.to_account_info(),
//                     collection_master_edition: ctx
//                         .accounts
//                         .collection_master_edition
//                         .to_account_info(),
//                 },
//             ),
//             Some(collection_key),
//         )?;
//         msg!("Collection set and verified successfully");
//         Ok(())
//     }

//     pub fn unverify_nft_from_collection(ctx: Context<UnverifyNFTFromCollection>) -> Result<()> {
//         msg!("Unverifying NFT from collection");
//         unverify_collection(
//             CpiContext::new(
//                 ctx.accounts.metadata_program.to_account_info(),
//                 UnverifyCollection {
//                     metadata: ctx.accounts.nft_metadata.to_account_info(),
//                     collection_authority: ctx.accounts.collection_authority.to_account_info(),
//                     collection_mint: ctx.accounts.collection_mint.to_account_info(),
//                     collection: ctx.accounts.collection_metadata.to_account_info(),
//                     collection_master_edition_account: ctx
//                         .accounts
//                         .collection_master_edition
//                         .to_account_info(),
//                 },
//             ),
//             None,
//         )?;
//         msg!("NFT unverified from collection successfully");
//         Ok(())
//     }

//     pub fn verify_collection_authority(ctx: Context<VerifyCollectionAuthority>) -> Result<()> {
//         msg!("Verifying collection authority");
//         require!(
//             ctx.accounts.collection_authority.key()
//                 == *ctx.accounts.collection_metadata.to_account_info().owner,
//             ErrorCode::InvalidCollectionAuthority
//         );
//         msg!("Collection authority verified successfully");
//         Ok(())
//     }

//     pub fn mint_and_verify_to_collection(
//         ctx: Context<MintAndVerifyToCollection>,
//         id_nft: u64,
//         name: String,
//         symbol: String,
//         uri: String,
//     ) -> Result<()> {
//         // Enforce minting price payment
//         let minting_price = ctx.accounts.program_state.minting_price;
//         let transfer_instruction = anchor_lang::system_program::Transfer {
//             from: ctx.accounts.payer.to_account_info(),
//             to: ctx.accounts.mint_fee_account.to_account_info(),
//         };
//         anchor_lang::system_program::transfer(
//             CpiContext::new(
//                 ctx.accounts.system_program.to_account_info(),
//                 transfer_instruction,
//             ),
//             minting_price,
//         )?;

//         msg!("Minting and verifying NFT in collection");
//         let collection_pubkey_val: Pubkey = ctx.accounts.collection_mint.key();
//         let collection_pubkey_bytes = collection_pubkey_val.to_bytes();
//         let id_nft_bytes = id_nft.to_le_bytes();
//         let seeds = &[
//             "item_mint".as_bytes(),
//             collection_pubkey_bytes.as_ref(),
//             id_nft_bytes.as_ref(),
//             &[ctx.bumps.mint],
//         ];

//         // Clone strings for metadata to avoid move errors
//         let name_for_metadata = name.clone();
//         let symbol_for_metadata = symbol.clone();
//         let uri_for_metadata = uri.clone();

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
//                 name: name_for_metadata,
//                 symbol: symbol_for_metadata,
//                 uri: uri_for_metadata,
//                 seller_fee_basis_points: 0,
//                 creators: Some(vec![Creator {
//                     address: ctx.accounts.payer.key(),
//                     verified: true,
//                     share: 100,
//                 }]),
//                 collection: Some(Collection {
//                     key: ctx.accounts.collection_mint.key(),
//                     verified: false,
//                 }),
//                 uses: None,
//             },
//             true,
//             true,
//             None,
//         )?;

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

//         verify_collection(
//             CpiContext::new(
//                 ctx.accounts.metadata_program.to_account_info(),
//                 VerifyCollection {
//                     payer: ctx.accounts.payer.to_account_info(),
//                     metadata: ctx.accounts.nft_metadata.to_account_info(),
//                     collection_authority: ctx.accounts.collection_authority.to_account_info(),
//                     collection_mint: ctx.accounts.collection_mint.to_account_info(),
//                     collection_metadata: ctx.accounts.collection_metadata.to_account_info(),
//                     collection_master_edition: ctx
//                         .accounts
//                         .collection_master_edition
//                         .to_account_info(),
//                 },
//             ),
//             None,
//         )?;

//         msg!("NFT minted and verified in collection successfully");
//         emit!(CollectionNftMinted {
//             id_nft,
//             mint: ctx.accounts.mint.key(),
//             payer: ctx.accounts.payer.key(),
//             authority: ctx.accounts.authority.key(),
//             name,
//             symbol,
//             uri,
//             collection: ctx.accounts.collection_mint.key(),
//         });
//         Ok(())
//     }

//     pub fn list_nft(ctx: Context<ListNFT>, price: u64, listing_id: u64) -> Result<()> {
//         msg!("Listing NFT for sale");
//         let listing = &mut ctx.accounts.listing;
//         let clock = Clock::get()?;

//         // Verify ownership
//         require!(
//             ctx.accounts.seller_token_account.amount == 1,
//             ErrorCode::SellerDoesNotOwnNFT
//         );
//         require!(
//             ctx.accounts.seller_token_account.mint == ctx.accounts.mint.key(),
//             ErrorCode::InvalidNFT
//         );
//         require!(
//             ctx.accounts.seller_token_account.owner == ctx.accounts.seller.key(),
//             ErrorCode::UnauthorizedSeller
//         );

//         listing.seller = ctx.accounts.seller.key();
//         listing.mint = ctx.accounts.mint.key();
//         listing.price = price;
//         listing.is_active = true;
//         listing.listed_at = clock.unix_timestamp;
//         listing.bump = ctx.bumps.listing;
//         msg!("NFT listed successfully for {} lamports", price);
//         emit!(NftListed {
//             listing_id,
//             mint: ctx.accounts.mint.key(),
//             seller: ctx.accounts.seller.key(),
//             price,
//             listed_at: listing.listed_at,
//         });
//         Ok(())
//     }

//     pub fn update_listing_price(ctx: Context<UpdateListing>, new_price: u64) -> Result<()> {
//         msg!("Updating listing price");
//         let listing = &mut ctx.accounts.listing;
//         require!(listing.is_active, ErrorCode::ListingNotActive);
//         require!(
//             listing.seller == ctx.accounts.seller.key(),
//             ErrorCode::UnauthorizedSeller
//         );
//         let old_price = listing.price;
//         listing.price = new_price;
//         msg!("Listing price updated to {} lamports", new_price);
//         emit!(ListingPriceUpdated {
//             listing: ctx.accounts.listing.key(),
//             seller: ctx.accounts.seller.key(),
//             old_price,
//             new_price,
//         });
//         Ok(())
//     }

//     pub fn cancel_listing(ctx: Context<CancelListing>) -> Result<()> {
//         msg!("Canceling NFT listing");
//         let listing = &mut ctx.accounts.listing;
//         require!(listing.is_active, ErrorCode::ListingNotActive);
//         require!(
//             listing.seller == ctx.accounts.seller.key(),
//             ErrorCode::UnauthorizedSeller
//         );
//         listing.is_active = false;
//         msg!("NFT listing canceled successfully");
//         emit!(ListingCanceled {
//             listing: ctx.accounts.listing.key(),
//             mint: ctx.accounts.listing.mint,
//             seller: ctx.accounts.seller.key(),
//         });
//         Ok(())
//     }

//     pub fn buy_nft(ctx: Context<BuyNFT>, marketplace_fee_bps: u16) -> Result<()> {
//         msg!("Buying NFT from listing");
//         let listing = &mut ctx.accounts.listing;
//         require!(listing.is_active, ErrorCode::ListingNotActive);
//         require!(
//             listing.seller != ctx.accounts.buyer.key(),
//             ErrorCode::SellerCannotBuy
//         );
//         let total_price = listing.price;
//         let marketplace_fee = (total_price as u128)
//             .checked_mul(marketplace_fee_bps as u128)
//             .unwrap()
//             .checked_div(10000)
//             .unwrap() as u64;
//         let seller_amount = total_price.checked_sub(marketplace_fee).unwrap();

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

//         let cpi_accounts = Transfer {
//             from: ctx.accounts.seller_token_account.to_account_info(),
//             to: ctx.accounts.buyer_token_account.to_account_info(),
//             authority: ctx.accounts.seller.to_account_info(),
//         };
//         let cpi_program = ctx.accounts.token_program.to_account_info();
//         let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
//         transfer(cpi_ctx, 1)?;

//         listing.is_active = false;
//         msg!("NFT purchased successfully for {} lamports", total_price);
//         emit!(NftPurchased {
//             listing: ctx.accounts.listing.key(),
//             mint: ctx.accounts.mint.key(),
//             buyer: ctx.accounts.buyer.key(),
//             seller: ctx.accounts.seller.key(),
//             price: total_price,
//             marketplace_fee,
//         });
//         Ok(())
//     }

//     pub fn make_offer(
//         ctx: Context<MakeOffer>,
//         offer_price: u64,
//         expiry_time: i64,
//         offer_id: u64,
//     ) -> Result<()> {
//         msg!("Making offer on NFT");
//         let offer = &mut ctx.accounts.offer;
//         let clock = Clock::get()?;
//         require!(expiry_time > clock.unix_timestamp, ErrorCode::OfferExpired);
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
//         emit!(OfferMade {
//             offer_id,
//             mint: ctx.accounts.mint.key(),
//             buyer: ctx.accounts.buyer.key(),
//             price: offer_price,
//             expiry_time,
//             created_at: offer.created_at,
//         });
//         Ok(())
//     }

//     pub fn accept_offer(ctx: Context<AcceptOffer>, marketplace_fee_bps: u16) -> Result<()> {
//         msg!("Accepting offer");
//         let offer = &mut ctx.accounts.offer;
//         let clock = Clock::get()?;
//         require!(offer.is_active, ErrorCode::OfferNotActive);
//         require!(
//             offer.expiry_time > clock.unix_timestamp,
//             ErrorCode::OfferExpired
//         );
//         require!(
//             ctx.accounts.seller_token_account.amount == 1,
//             ErrorCode::SellerDoesNotOwnNFT
//         );
//         let total_price = offer.price;
//         let marketplace_fee = (total_price as u128)
//             .checked_mul(marketplace_fee_bps as u128)
//             .unwrap()
//             .checked_div(10000)
//             .unwrap() as u64;
//         let seller_amount = total_price.checked_sub(marketplace_fee).unwrap();

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

//         let cpi_accounts = Transfer {
//             from: ctx.accounts.seller_token_account.to_account_info(),
//             to: ctx.accounts.buyer_token_account.to_account_info(),
//             authority: ctx.accounts.seller.to_account_info(),
//         };
//         let cpi_program = ctx.accounts.token_program.to_account_info();
//         let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
//         transfer(cpi_ctx, 1)?;

//         offer.is_active = false;
//         msg!("Offer accepted for {} lamports", total_price);
//         emit!(OfferAccepted {
//             offer: ctx.accounts.offer.key(),
//             mint: ctx.accounts.mint.key(),
//             buyer: ctx.accounts.buyer.key(),
//             seller: ctx.accounts.seller.key(),
//             price: total_price,
//             marketplace_fee,
//         });
//         Ok(())
//     }

//     pub fn cancel_offer(ctx: Context<CancelOffer>) -> Result<()> {
//         msg!("Canceling offer");
//         let offer = &mut ctx.accounts.offer;
//         require!(offer.is_active, ErrorCode::OfferNotActive);
//         require!(
//             offer.buyer == ctx.accounts.buyer.key(),
//             ErrorCode::UnauthorizedBuyer
//         );
//         offer.is_active = false;
//         msg!("Offer canceled successfully");
//         emit!(OfferCanceled {
//             offer: ctx.accounts.offer.key(),
//             mint: ctx.accounts.offer.mint,
//             buyer: ctx.accounts.buyer.key(),
//         });
//         Ok(())
//     }

//     pub fn initialize_program_state(
//         ctx: Context<InitializeProgramState>,
//         minting_price: u64,
//     ) -> Result<()> {
//         let program_state = &mut ctx.accounts.program_state;
//         program_state.admin = ctx.accounts.admin.key();
//         program_state.minting_price = minting_price;
//         msg!(
//             "Program state initialized with minting price: {} lamports",
//             minting_price
//         );
//         Ok(())
//     }

//     pub fn set_minting_price(ctx: Context<SetMintingPrice>, new_price: u64) -> Result<()> {
//         let program_state = &mut ctx.accounts.program_state;
//         program_state.minting_price = new_price;
//         msg!("Minting price updated to: {} lamports", new_price);
//         Ok(())
//     }
// }

// #[derive(Accounts)]
// #[instruction(id: u64)]
// pub struct CreateNFT<'info> {
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
//         seeds = ["mint".as_bytes(), id.to_le_bytes().as_ref()],
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
//     #[account(
//         seeds = [b"program-state"],
//         bump
//     )]
//     pub program_state: Account<'info, ProgramState>,
//     /// CHECK: This is the admin's account to receive the minting fee
//     #[account(mut, address = program_state.admin)]
//     pub mint_fee_account: AccountInfo<'info>,
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
//         init,
//         payer = payer,
//         mint::decimals = 0,
//         mint::authority = authority,
//         mint::freeze_authority = authority,
//         seeds = ["item_mint".as_bytes(),
//                  collection.key().as_ref(),
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
//     #[account(
//         seeds = [b"program-state"],
//         bump
//     )]
//     pub program_state: Account<'info, ProgramState>,
//     /// CHECK: This is the admin's account to receive the minting fee
//     #[account(mut, address = program_state.admin)]
//     pub mint_fee_account: AccountInfo<'info>,
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
// pub struct VerifyNFTInCollection<'info> {
//     #[account(mut)]
//     pub payer: Signer<'info>,
//     /// CHECK:
//     pub collection_authority: Signer<'info>,
//     /// CHECK:
//     #[account(mut)]
//     pub nft_metadata: UncheckedAccount<'info>,
//     /// CHECK:
//     pub collection_mint: UncheckedAccount<'info>,
//     /// CHECK:
//     pub collection_metadata: UncheckedAccount<'info>,
//     /// CHECK:
//     pub collection_master_edition: UncheckedAccount<'info>,
//     pub metadata_program: Program<'info, Metadata>,
// }

// #[derive(Accounts)]
// pub struct SetAndVerifyCollectionCtx<'info> {
//     #[account(mut)]
//     pub payer: Signer<'info>,
//     /// CHECK:
//     pub update_authority: Signer<'info>,
//     /// CHECK:
//     pub collection_authority: Signer<'info>,
//     /// CHECK:
//     #[account(mut)]
//     pub nft_metadata: UncheckedAccount<'info>,
//     /// CHECK:
//     pub collection_mint: UncheckedAccount<'info>,
//     /// CHECK:
//     pub collection_metadata: UncheckedAccount<'info>,
//     /// CHECK:
//     pub collection_master_edition: UncheckedAccount<'info>,
//     pub metadata_program: Program<'info, Metadata>,
// }

// #[derive(Accounts)]
// pub struct UnverifyNFTFromCollection<'info> {
//     #[account(mut)]
//     pub payer: Signer<'info>,
//     /// CHECK:
//     pub collection_authority: Signer<'info>,
//     /// CHECK:
//     #[account(mut)]
//     pub nft_metadata: UncheckedAccount<'info>,
//     /// CHECK:
//     pub collection_mint: UncheckedAccount<'info>,
//     /// CHECK:
//     pub collection_metadata: UncheckedAccount<'info>,
//     /// CHECK:
//     pub collection_master_edition: UncheckedAccount<'info>,
//     pub metadata_program: Program<'info, Metadata>,
// }

// #[derive(Accounts)]
// pub struct VerifyCollectionAuthority<'info> {
//     /// CHECK:
//     pub collection_authority: Signer<'info>,
//     /// CHECK:
//     pub collection_metadata: UncheckedAccount<'info>,
// }

// #[derive(Accounts)]
// #[instruction(id_nft: u64)]
// pub struct MintAndVerifyToCollection<'info> {
//     #[account(mut)]
//     pub authority: Signer<'info>,
//     #[account(mut)]
//     pub payer: Signer<'info>,
//     /// CHECK:
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
//     #[account(
//         seeds = [b"program-state"],
//         bump
//     )]
//     pub program_state: Account<'info, ProgramState>,
//     /// CHECK: This is the admin's account to receive the minting fee
//     #[account(mut, address = program_state.admin)]
//     pub mint_fee_account: AccountInfo<'info>,
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
//     pub collection_mint: UncheckedAccount<'info>,
//     /// CHECK:
//     pub collection_metadata: UncheckedAccount<'info>,
//     /// CHECK:
//     pub collection_master_edition: UncheckedAccount<'info>,
// }

// #[account]
// pub struct Listing {
//     pub seller: Pubkey,
//     pub mint: Pubkey,
//     pub price: u64,
//     pub is_active: bool,
//     pub listed_at: i64,
//     pub bump: u8,
// }

// impl Listing {
//     pub const LEN: usize = 8 + 32 + 32 + 8 + 1 + 8 + 1;
// }

// #[account]
// pub struct Offer {
//     pub buyer: Pubkey,
//     pub mint: Pubkey,
//     pub price: u64,
//     pub expiry_time: i64,
//     pub is_active: bool,
//     pub created_at: i64,
//     pub bump: u8,
// }

// impl Offer {
//     pub const LEN: usize = 8 + 32 + 32 + 8 + 8 + 1 + 8 + 1;
// }

// #[account]
// pub struct ProgramState {
//     pub admin: Pubkey,
//     pub minting_price: u64,
// }

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
//     /// CHECK:
//     #[account(mut)]
//     pub seller: UncheckedAccount<'info>,
//     /// CHECK:
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
//     /// CHECK:
//     #[account(mut)]
//     pub buyer: UncheckedAccount<'info>,
//     /// CHECK:
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

// #[derive(Accounts)]
// pub struct InitializeProgramState<'info> {
//     #[account(
//         init,
//         payer = admin,
//         space = 8 + 32 + 8,
//         seeds = [b"program-state"],
//         bump
//     )]
//     pub program_state: Account<'info, ProgramState>,
//     #[account(mut)]
//     pub admin: Signer<'info>,
//     pub system_program: Program<'info, System>,
// }

// #[derive(Accounts)]
// pub struct SetMintingPrice<'info> {
//     #[account(
//         mut,
//         seeds = [b"program-state"],
//         bump,
//         has_one = admin @ ErrorCode::Unauthorized
//     )]
//     pub program_state: Account<'info, ProgramState>,
//     pub admin: Signer<'info>,
// }

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
//     #[msg("Only the admin can perform this action")]
//     Unauthorized,
//     #[msg("Invalid NFT in token account")]
//     InvalidNFT,
//     #[msg("Unauthorized collection update authority")]
//     UnauthorizedCollectionUpdateAuthority,
// }
