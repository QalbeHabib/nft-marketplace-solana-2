use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::metadata::{
    create_master_edition_v3, create_metadata_accounts_v3, CreateMasterEditionV3,
    CreateMetadataAccountsV3,
};
use anchor_spl::token::{mint_to, MintTo};
use mpl_token_metadata::types::{Creator, DataV2};

pub fn create_collection(
    ctx: Context<CreateCollection>,
    id_collection: u64,
    name: String,
    symbol: String,
    uri: String,
) -> Result<()> {
    msg!("Creating collection with unique seeds");

    let program_id_bytes = ctx.program_id.to_bytes();
    let id_bytes = id_collection.to_le_bytes();

    let seeds = &[
        crate::constants::PROGRAM_SEED_PREFIX,
        crate::constants::COLLECTION_MINT_SEED_PREFIX,
        program_id_bytes.as_ref(),
        id_bytes.as_ref(),
        &[ctx.bumps.mint],
    ];

    // Mint the collection NFT
    msg!("Minting collection NFT");
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

    // Create metadata
    msg!("Creating collection metadata");
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
            name: name.clone(),
            symbol: symbol.clone(),
            uri: uri.clone(),
            seller_fee_basis_points: 0,
            creators: Some(vec![Creator {
                address: ctx.accounts.authority.key(),
                verified: true,
                share: 100,
            }]),
            collection: None,
            uses: None,
        },
        true,
        true,
        None,
    )?;

    // Create master edition
    msg!("Creating master edition");
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
        Some(0),
    )?;

    // Initialize the UserCollection PDA
    let user_collection = &mut ctx.accounts.user_collection;
    let clock = Clock::get()?;

    user_collection.authority = ctx.accounts.authority.key();
    user_collection.collection_mint = ctx.accounts.mint.key();
    user_collection.name = name.clone();
    user_collection.symbol = symbol.clone();
    user_collection.uri = uri.clone();
    user_collection.created_at = clock.unix_timestamp;
    user_collection.total_items = 0;
    user_collection.verified = true;
    user_collection.bump = ctx.bumps.user_collection;

    msg!("Created collection NFT and stored collection data successfully");

    // Emit the event with additional collection data
    emit!(CollectionCreated {
        id_collection,
        mint: ctx.accounts.mint.key(),
        authority: ctx.accounts.authority.key(),
        payer: ctx.accounts.payer.key(),
        name,
        symbol,
        uri,
        created_at: clock.unix_timestamp,
    });

    Ok(())
}
