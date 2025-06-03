use crate::state::*;
use crate::utils::validate_royalty_percent;
use anchor_lang::prelude::*;
use anchor_spl::metadata::{
    create_master_edition_v3, create_metadata_accounts_v3, verify_collection,
    CreateMasterEditionV3, CreateMetadataAccountsV3, VerifyCollection,
};
use anchor_spl::token::{mint_to, MintTo};
use mpl_token_metadata::types::{Collection, Creator, DataV2};

pub fn mint_and_verify_to_collection(
    ctx: Context<MintAndVerifyToCollection>,
    id_nft: u64,
    name: String,
    symbol: String,
    uri: String,
    royalty_percent: u16,
) -> Result<()> {
    // Enforce minting price payment
    let minting_price = ctx.accounts.program_state.minting_price;
    let transfer_instruction = anchor_lang::system_program::Transfer {
        from: ctx.accounts.payer.to_account_info(),
        to: ctx.accounts.mint_fee_account.to_account_info(),
    };
    anchor_lang::system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            transfer_instruction,
        ),
        minting_price,
    )?;

    // Validate and convert royalty percentage to basis points
    let seller_fee_basis_points = validate_royalty_percent(royalty_percent)?;

    msg!("Minting and verifying NFT in collection with unique seeds");

    let program_id_bytes = ctx.program_id.to_bytes();
    let collection_pubkey_val: Pubkey = ctx.accounts.collection_mint.key();
    let collection_pubkey_bytes = collection_pubkey_val.to_bytes();
    let id_nft_bytes = id_nft.to_le_bytes();

    let seeds = &[
        crate::constants::PROGRAM_SEED_PREFIX,
        crate::constants::COLLECTION_ITEM_SEED_PREFIX,
        program_id_bytes.as_ref(),
        collection_pubkey_bytes.as_ref(),
        id_nft_bytes.as_ref(),
        &[ctx.bumps.mint],
    ];

    // Initialize UserCollection if it's new
    let user_collection = &mut ctx.accounts.user_collection;

    // Only initialize if it's a new account (total_items will be 0 by default for new accounts)
    if user_collection.authority == Pubkey::default() {
        msg!("Initializing new UserCollection");
        user_collection.authority = ctx.accounts.collection_authority.key();
        user_collection.collection_mint = ctx.accounts.collection_mint.key();
        user_collection.name = name.clone();
        user_collection.symbol = symbol.clone();
        user_collection.uri = uri.clone();
        user_collection.created_at = Clock::get()?.unix_timestamp;
        user_collection.total_items = 0;
        user_collection.verified = true;
        user_collection.bump = ctx.bumps.user_collection;
    }

    // Clone strings for metadata to avoid move errors
    let name_for_metadata = name.clone();
    let symbol_for_metadata = symbol.clone();
    let uri_for_metadata = uri.clone();

    mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                authority: ctx.accounts.authority.to_account_info(),
                to: ctx.accounts.token_account.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
            },
            &[&seeds[..]],
        ),
        1,
    )?;

    msg!("Run create metadata accounts v3");
    create_metadata_accounts_v3(
        CpiContext::new_with_signer(
            ctx.accounts.metadata_program.to_account_info(),
            CreateMetadataAccountsV3 {
                payer: ctx.accounts.payer.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                metadata: ctx.accounts.nft_metadata.to_account_info(),
                mint_authority: ctx.accounts.authority.to_account_info(),
                update_authority: ctx.accounts.authority.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
            &[&seeds[..]],
        ),
        DataV2 {
            name: name_for_metadata,
            symbol: symbol_for_metadata,
            uri: uri_for_metadata,
            seller_fee_basis_points,
            creators: Some(vec![Creator {
                address: ctx.accounts.payer.key(),
                verified: true,
                share: 100,
            }]),
            collection: Some(Collection {
                key: ctx.accounts.collection_mint.key(),
                verified: false, // Will be verified in the next step
            }),
            uses: None,
        },
        true,
        true,
        None,
    )?;

    create_master_edition_v3(
        CpiContext::new_with_signer(
            ctx.accounts.metadata_program.to_account_info(),
            CreateMasterEditionV3 {
                edition: ctx.accounts.master_edition_account.to_account_info(),
                payer: ctx.accounts.payer.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                metadata: ctx.accounts.nft_metadata.to_account_info(),
                mint_authority: ctx.accounts.authority.to_account_info(),
                update_authority: ctx.accounts.authority.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
            &[&seeds[..]],
        ),
        Some(1),
    )?;

    // FIXED: Collection is properly verified here
    verify_collection(
        CpiContext::new(
            ctx.accounts.metadata_program.to_account_info(),
            VerifyCollection {
                payer: ctx.accounts.payer.to_account_info(),
                metadata: ctx.accounts.nft_metadata.to_account_info(),
                collection_authority: ctx.accounts.collection_authority.to_account_info(),
                collection_mint: ctx.accounts.collection_mint.to_account_info(),
                collection_metadata: ctx.accounts.collection_metadata.to_account_info(),
                collection_master_edition: ctx.accounts.collection_master_edition.to_account_info(),
            },
        ),
        None,
    )?;

    // Increment the total_items counter in UserCollection
    let user_collection = &mut ctx.accounts.user_collection;
    user_collection.total_items = user_collection
        .total_items
        .checked_add(1)
        .ok_or(crate::errors::ErrorCode::ArithmeticOverflow)?;

    msg!("NFT minted and verified in collection successfully");
    emit!(CollectionNftMinted {
        id_nft,
        mint: ctx.accounts.mint.key(),
        payer: ctx.accounts.payer.key(),
        authority: ctx.accounts.authority.key(),
        name,
        symbol,
        uri,
        collection: ctx.accounts.collection_mint.key(),
        royalty_percent,
        seller_fee_basis_points,
        creators: vec![CreatorEventData {
            address: ctx.accounts.payer.key(),
            verified: true,
            share: 100,
        }],
    });

    // Emit new event for collection item count update
    emit!(CollectionItemCountUpdated {
        collection_mint: ctx.accounts.collection_mint.key(),
        total_items: user_collection.total_items,
        timestamp: Clock::get()?.unix_timestamp,
    });

    Ok(())
}
