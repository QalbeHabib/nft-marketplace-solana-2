// use anchor_lang::prelude::*;
// use anchor_spl::associated_token::AssociatedToken;
// use anchor_spl::metadata::{
//     create_master_edition_v3, create_metadata_accounts_v3, CreateMasterEditionV3,
//     CreateMetadataAccountsV3, Metadata as SplMetadata,
// };
// use anchor_spl::token::{self, Mint, MintTo, Token, TokenAccount};
// use mpl_token_metadata::{MAX_NAME_LENGTH, MAX_SYMBOL_LENGTH, MAX_URI_LENGTH};

// declare_id!("5tADc3XAnLaYKUgMNaJB3RHvmn9JF8UPJGzpHhTFyCRJ");

// #[program]
// pub mod collection_creator {
//     use super::*;

//     /// Create a collection NFT with authority and size tracking
//     /// This function creates a new collection that can be used in an NFT marketplace
//     pub fn create_collection(
//         ctx: Context<CreateCollection>,
//         name: String,
//         symbol: String,
//         uri: String,
//         creators: Vec<CollectionCreatorInput>,
//         seller_fee_basis_points: u16,
//         max_size: u64,
//     ) -> Result<()> {
//         // Input validation with proper error messages
//         if name.trim().is_empty() {
//             return err!(ErrorCode::EmptyName);
//         }
//         if name.len() > MAX_NAME_LENGTH {
//             return err!(ErrorCode::NameTooLong);
//         }
//         if symbol.trim().is_empty() {
//             return err!(ErrorCode::EmptySymbol);
//         }
//         if symbol.len() > MAX_SYMBOL_LENGTH {
//             return err!(ErrorCode::SymbolTooLong);
//         }
//         if uri.trim().is_empty() {
//             return err!(ErrorCode::EmptyUri);
//         }
//         if uri.len() > MAX_URI_LENGTH {
//             return err!(ErrorCode::UriTooLong);
//         }
//         if max_size == 0 {
//             return err!(ErrorCode::ZeroMaxSize);
//         }
//         if max_size > 10_000_000 {
//             // Higher limit for large collections
//             return err!(ErrorCode::MaxSizeTooLarge);
//         }
//         if creators.is_empty() {
//             return err!(ErrorCode::NoCreators);
//         }
//         // Validate total creator shares add up to 100%
//         let total_shares = creators.iter().fold(0, |acc, creator| acc + creator.share);
//         if total_shares != 100 {
//             return err!(ErrorCode::InvalidCreatorShares);
//         }
//         if seller_fee_basis_points > 10000 {
//             // Max is 100% (10000 = 100.00%)
//             return err!(ErrorCode::InvalidSellerFee);
//         }

//         // Initialize collection authority account with marketplace tracking data
//         let collection_auth = &mut ctx.accounts.collection_authority;
//         collection_auth.collection_mint = ctx.accounts.collection_mint.key();
//         collection_auth.authority = ctx.accounts.payer.key();
//         collection_auth.current_size = 0;
//         collection_auth.max_size = max_size;
//         collection_auth.created_at = Clock::get()?.unix_timestamp;
//         collection_auth.marketplace_id = ctx.accounts.marketplace_authority.key();

//         // Mint 1 token to the collection token account
//         token::mint_to(
//             CpiContext::new(
//                 ctx.accounts.token_program.to_account_info(),
//                 MintTo {
//                     mint: ctx.accounts.collection_mint.to_account_info(),
//                     to: ctx.accounts.collection_token_account.to_account_info(),
//                     authority: ctx.accounts.mint_authority.to_account_info(),
//                 },
//             ),
//             1, // Mint exactly 1 token for the collection NFT
//         )?;

//         // Convert marketplace creator inputs to Metaplex creators format
//         let metaplex_creators = creators
//             .iter()
//             .map(
//                 |creator| anchor_spl::metadata::mpl_token_metadata::types::Creator {
//                     address: creator.address,
//                     verified: creator.address == ctx.accounts.payer.key()
//                         || creator.address == ctx.accounts.marketplace_authority.key(),
//                     share: creator.share,
//                 },
//             ).collect::<Vec<_>>();

//         // Construct DataV2 for the collection with marketplace info
//         let data_v2_collection = anchor_spl::metadata::mpl_token_metadata::types::DataV2 {
//             name: name.clone(),
//             symbol: symbol.clone(),
//             uri: uri.clone(),
//             seller_fee_basis_points,
//             creators: Some(metaplex_creators),
//             collection: None, // This is a collection, so it has no parent collection
//             uses: None,
//         };

//         // CPI to create collection metadata and set its size
//         let cpi_context_create = CpiContext::new(
//             ctx.accounts.metadata_program.to_account_info(),
//             CreateMetadataAccountsV3 {
//                 metadata: ctx.accounts.collection_metadata.to_account_info(),
//                 mint: ctx.accounts.collection_mint.to_account_info(),
//                 mint_authority: ctx.accounts.mint_authority.to_account_info(),
//                 payer: ctx.accounts.payer.to_account_info(),
//                 update_authority: ctx.accounts.payer.to_account_info(),
//                 system_program: ctx.accounts.system_program.to_account_info(),
//                 rent: ctx.accounts.rent.to_account_info(),
//             },
//         );
//         create_metadata_accounts_v3(
//             cpi_context_create,
//             data_v2_collection,
//             true, // is_mutable
//             true, // update_authority_is_signer
//             Some(
//                 anchor_spl::metadata::mpl_token_metadata::types::CollectionDetails::V1 {
//                     size: max_size,
//                 },
//             ),
//         )?;

//         // CPI to create master edition for the collection NFT
//         let cpi_context_create_master_edition = CpiContext::new(
//             ctx.accounts.metadata_program.to_account_info(),
//             CreateMasterEditionV3 {
//                 edition: ctx.accounts.master_edition.to_account_info(),
//                 mint: ctx.accounts.collection_mint.to_account_info(),
//                 update_authority: ctx.accounts.payer.to_account_info(),
//                 mint_authority: ctx.accounts.mint_authority.to_account_info(),
//                 payer: ctx.accounts.payer.to_account_info(),
//                 metadata: ctx.accounts.collection_metadata.to_account_info(),
//                 token_program: ctx.accounts.token_program.to_account_info(),
//                 system_program: ctx.accounts.system_program.to_account_info(),
//                 rent: ctx.accounts.rent.to_account_info(),
//             },
//         );
//         create_master_edition_v3(cpi_context_create_master_edition, Some(0))?; // Max supply 0 for collection

//         // Emit marketplace collection creation event with all relevant info
//         emit!(MarketplaceCollectionCreated {
//             collection_mint: ctx.accounts.collection_mint.key(),
//             marketplace: ctx.accounts.marketplace_authority.key(),
//             creators: creators.iter().map(|c| c.address).collect::<Vec<_>>(),
//             name: name.clone(),
//             symbol: symbol.clone(),
//             max_size,
//             seller_fee_basis_points,
//             timestamp: Clock::get()?.unix_timestamp,
//         });

//         Ok(())
//     }

//     // Additional marketplace functions would go here:
//     // - update_collection
//     // - mint_to_collection
//     // - verify_collection_item
//     // - transfer_collection_authority
// }

// #[derive(AnchorSerialize, AnchorDeserialize, Clone)]
// pub struct CollectionCreatorInput {
//     pub address: Pubkey,
//     pub share: u8,
// }

// #[derive(Accounts)]
// pub struct CreateCollection<'info> {
//     #[account(
//         init,
//         payer = payer,
//         mint::decimals = 0,
//         mint::authority = mint_authority.key(),
//     )]
//     pub collection_mint: Account<'info, Mint>,

//     /// CHECK: This is the metadata account, initialized by Metaplex CPI
//     #[account(
//         mut,
//         seeds = [
//             b"metadata",
//             metadata_program.key().as_ref(),
//             collection_mint.key().as_ref()
//         ],
//         bump,
//         seeds::program = metadata_program.key()
//     )]
//     pub collection_metadata: UncheckedAccount<'info>,

//     /// CHECK: Master Edition account for the collection NFT
//     #[account(
//         mut,
//         seeds = [
//             b"metadata",
//             metadata_program.key().as_ref(),
//             collection_mint.key().as_ref(),
//             b"edition"
//         ],
//         bump,
//         seeds::program = metadata_program.key()
//     )]
//     pub master_edition: UncheckedAccount<'info>,

//     #[account(
//         init,
//         payer = payer,
//         associated_token::mint = collection_mint,
//         associated_token::authority = payer
//     )]
//     pub collection_token_account: Account<'info, TokenAccount>,

//     #[account(
//         init,
//         payer = payer,
//         space = 8 + // discriminator
//                32 + // collection_mint
//                32 + // authority
//                8 +  // current_size
//                8 +  // max_size
//                8 +  // created_at
//                32,  // marketplace_id
//         seeds = [b"collection_auth", collection_mint.key().as_ref()],
//         bump,
//     )]
//     pub collection_authority: Account<'info, CollectionAuthority>,

//     #[account(mut, signer)]
//     pub mint_authority: Signer<'info>,

//     #[account(mut)]
//     pub payer: Signer<'info>,

//     /// CHECK: Marketplace authority account, must be signer to ensure legitimacy
//     #[account(signer)]
//     pub marketplace_authority: UncheckedAccount<'info>,

//     pub system_program: Program<'info, System>,
//     pub rent: Sysvar<'info, Rent>,
//     pub token_program: Program<'info, Token>,
//     pub associated_token_program: Program<'info, AssociatedToken>,
//     pub metadata_program: Program<'info, SplMetadata>,
// }

// #[account]
// pub struct CollectionAuthority {
//     pub collection_mint: Pubkey,
//     pub authority: Pubkey,
//     pub current_size: u64,
//     pub max_size: u64,
//     pub created_at: i64,
//     pub marketplace_id: Pubkey,
// }

// #[error_code]
// pub enum ErrorCode {
//     #[msg("Collection size limit reached")]
//     CollectionSizeLimitReached,
//     #[msg("Unauthorized collection authority")]
//     UnauthorizedCollectionAuthority,
//     #[msg("Name too long")]
//     NameTooLong,
//     #[msg("Symbol too long")]
//     SymbolTooLong,
//     #[msg("URI too long")]
//     UriTooLong,
//     #[msg("Empty name provided")]
//     EmptyName,
//     #[msg("Empty symbol provided")]
//     EmptySymbol,
//     #[msg("Empty URI provided")]
//     EmptyUri,
//     #[msg("Zero max size not allowed")]
//     ZeroMaxSize,
//     #[msg("Max size too large")]
//     MaxSizeTooLarge,
//     #[msg("Invalid max size for collection")]
//     InvalidMaxSize,
//     #[msg("NFT mint must have 0 decimals")]
//     InvalidMintDecimals,
//     #[msg("No creators provided")]
//     NoCreators,
//     #[msg("Creator shares must total 100")]
//     InvalidCreatorShares,
//     #[msg("Invalid seller fee (max 10000 basis points)")]
//     InvalidSellerFee,
//     #[msg("Marketplace authority must be a signer")]
//     MarketplaceNotSigner,
// }

// #[event]
// pub struct MarketplaceCollectionCreated {
//     pub collection_mint: Pubkey,
//     pub marketplace: Pubkey,
//     pub creators: Vec<Pubkey>,
//     pub name: String,
//     pub symbol: String,
//     pub max_size: u64,
//     pub seller_fee_basis_points: u16,
//     pub timestamp: i64,
// }
