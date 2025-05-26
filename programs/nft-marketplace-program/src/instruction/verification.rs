// use anchor_lang::prelude::*;
// use anchor_spl::associated_token::AssociatedToken;
// use anchor_spl::metadata::{
//     create_master_edition_v3, create_metadata_accounts_v3, CreateMasterEditionV3,
//     CreateMetadataAccountsV3, Metadata, verify_collection, VerifyCollection,
//     set_and_verify_collection, SetAndVerifyCollection, unverify_collection, UnverifyCollection,
// };
// use anchor_spl::token::{mint_to, Mint, MintTo, Token, TokenAccount};
// use mpl_token_metadata::types::{Collection, Creator, DataV2};

// declare_id!("QaQX5WUroY6mHE8RPXXiQUnU73YFRVwKGkSaFcFj6yw");

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
//             )
//         )?;

//         msg!("NFT verified in collection successfully");
//         Ok(())
//     }

//     /// Set and verify collection in one transaction (more efficient)
//     pub fn set_and_verify_collection(
//         ctx: Context<SetAndVerifyCollectionCtx>,
//         collection_key: Pubkey,
//     ) -> Result<()> {
//         msg!("Setting and verifying collection");

//         set_and_verify_collection(
//             CpiContext::new(
//                 ctx.accounts.metadata_program.to_account_info(),
//                 SetAndVerifyCollection {
//                     payer: ctx.accounts.payer.to_account_info(),
//                     metadata: ctx.accounts.nft_metadata.to_account_info(),
//                     update_authority: ctx.accounts.update_authority.to_account_info(),
//                     collection_mint: ctx.accounts.collection_mint.to_account_info(),
//                     collection_metadata: ctx.accounts.collection_metadata.to_account_info(),
//                     collection_master_edition: ctx.accounts.collection_master_edition.to_account_info(),
//                 }
//             ),
//             Some(Collection {
//                 key: collection_key,
//                 verified: true,
//             })
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
//                     payer: ctx.accounts.payer.to_account_info(),
//                     metadata: ctx.accounts.nft_metadata.to_account_info(),
//                     collection_authority: ctx.accounts.collection_authority.to_account_info(),
//                     collection_mint: ctx.accounts.collection_mint.to_account_info(),
//                     collection_metadata: ctx.accounts.collection_metadata.to_account_info(),
//                     collection_master_edition: ctx.accounts.collection_master_edition.to_account_info(),
//                 }
//             )
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
//             ctx.accounts.collection_authority.key() == ctx.accounts.collection_metadata.to_account_info().owner,
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
//             )
//         )?;

//         msg!("NFT minted and verified in collection successfully");
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
// }
