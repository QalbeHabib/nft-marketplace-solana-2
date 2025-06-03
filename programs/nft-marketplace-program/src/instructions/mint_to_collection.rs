use crate::state::*;
use crate::utils::validate_royalty_percent;
use anchor_lang::prelude::*;
use anchor_spl::metadata::{
    create_master_edition_v3, create_metadata_accounts_v3, CreateMasterEditionV3,
    CreateMetadataAccountsV3,
};
use anchor_spl::token::{mint_to, MintTo};
use mpl_token_metadata::types::{Collection, Creator, DataV2};

pub fn mint_to_collection(
    ctx: Context<MintToCollection>,
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

    msg!("Creating seeds for NFT in collection with unique prefix");

    let program_id_bytes = ctx.program_id.to_bytes();
    let collection_pubkey_val: Pubkey = *ctx.accounts.collection.key;
    let collection_pubkey_bytes = collection_pubkey_val.to_bytes();
    let id_nft_bytes = id_nft.to_le_bytes();

    // FIXED: More unique seeds including program ID
    let seeds = &[
        crate::constants::PROGRAM_SEED_PREFIX,
        crate::constants::COLLECTION_ITEM_SEED_PREFIX,
        program_id_bytes.as_ref(),
        collection_pubkey_bytes.as_ref(),
        id_nft_bytes.as_ref(),
        &[ctx.bumps.mint],
    ];

    msg!("Run mint_to");
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
    let name_for_metadata = name.clone();
    let symbol_for_metadata = symbol.clone();
    let uri_for_metadata = uri.clone();

    // Create the creators vector for metadata
    let creators = vec![Creator {
        address: ctx.accounts.payer.key(),
        verified: true,
        share: 100,
    }];

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
            creators: Some(creators.clone()),
            collection: Some(Collection {
                key: ctx.accounts.collection.key(),
                verified: false, // Will be verified in the next step
            }),
            uses: None,
        },
        true,
        true,
        None,
    )?;

    msg!("Run create master edition v3");
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

    msg!("Minted NFT successfully");

    // Convert creators to CreatorEventData for the event
    let creator_event_data: Vec<CreatorEventData> = creators
        .iter()
        .map(|c| CreatorEventData {
            address: c.address,
            verified: c.verified,
            share: c.share,
        })
        .collect();

    emit!(CollectionNftMinted {
        id_nft,
        mint: ctx.accounts.mint.key(),
        payer: ctx.accounts.payer.key(),
        authority: ctx.accounts.authority.key(),
        name,
        symbol,
        uri,
        collection: ctx.accounts.collection.key(),
        royalty_percent,
        seller_fee_basis_points,
        creators: creator_event_data,
    });

    Ok(())
}
