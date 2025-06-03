import * as anchor from "@coral-xyz/anchor";
import {
  ComputeBudgetProgram,
  Connection,
  Keypair,
  PublicKey,
  SystemProgram,
} from "@solana/web3.js";
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  TOKEN_PROGRAM_ID,
  getAssociatedTokenAddressSync,
} from "@solana/spl-token";
import idl from "../target/idl/nft_program.json";
import type { NftProgram } from "../target/types/nft_program";
import * as fs from "fs";

// Program ID from your lib.rs
const PROGRAM_ID = new PublicKey(
  "Equiqs1Z5Q4F1gBuciqo6yrvqNERzwp5v9Fskhq2A5WB"
);
const METADATA_PROGRAM_ID = new PublicKey(
  "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
);
const connection = new Connection("https://api.devnet.solana.com", "confirmed");

// Load the admin keypair (deployer or authorized admin)
const walletKeypairFile = fs.readFileSync("./wallet-keypair.json", "utf-8");
const walletKeypair = Keypair.fromSecretKey(
  Buffer.from(JSON.parse(walletKeypairFile))
);
const authority = walletKeypair;

// Define seed prefixes (must match lib.rs)
const PROGRAM_SEED_PREFIX = Buffer.from("nft_marketplace_v1");
const PROGRAM_STATE_SEED = Buffer.from("program_state");
const COLLECTION_MINT_SEED_PREFIX = Buffer.from("collection_mint");
const COLLECTION_ITEM_SEED_PREFIX = Buffer.from("collection_item");

async function mintToCollection() {
  try {
    // Set up the provider
    const provider = new anchor.AnchorProvider(
      connection,
      new anchor.Wallet(walletKeypair),
      { commitment: "confirmed" }
    );

    // Load the program
    const program = new anchor.Program<NftProgram>(idl as NftProgram, provider);

    // Parameters for minting
    const idNft = new anchor.BN(2); // Unique ID for the NFT in collection
    const idCollection = new anchor.BN(1); // Collection ID (must exist)
    const name = "DevDead";
    const symbol = "DDT";
    const uri =
      "https://rose-causal-albatross-891.mypinata.cloud/ipfs/QmZxM5i8N1iho2jTuEDBhsssNpPKrniv6dVJxmW653sKyF"; // Metadata URI for the NFT

    console.log("🚀 Starting mint to collection process...");

    // 1. Derive collection mint PDA
    const [collectionMint] = PublicKey.findProgramAddressSync(
      [
        PROGRAM_SEED_PREFIX,
        COLLECTION_MINT_SEED_PREFIX,
        PROGRAM_ID.toBuffer(),
        idCollection.toArrayLike(Buffer, "le", 8),
      ],
      PROGRAM_ID
    );

    console.log("📦 Collection Mint:", collectionMint.toString());

    // 2. Derive NFT mint PDA using collection mint
    const [nftMint] = PublicKey.findProgramAddressSync(
      [
        PROGRAM_SEED_PREFIX,
        COLLECTION_ITEM_SEED_PREFIX,
        PROGRAM_ID.toBuffer(),
        collectionMint.toBuffer(),
        idNft.toArrayLike(Buffer, "le", 8),
      ],
      PROGRAM_ID
    );

    console.log("🎨 NFT Mint:", nftMint.toString());

    // 3. Get associated token account for the minter
    const tokenAccount = getAssociatedTokenAddressSync(
      nftMint,
      authority.publicKey
    );

    console.log("💰 Token Account:", tokenAccount.toString());

    // 4. Derive program state PDA
    const [programState] = PublicKey.findProgramAddressSync(
      [PROGRAM_SEED_PREFIX, PROGRAM_STATE_SEED, PROGRAM_ID.toBuffer()],
      PROGRAM_ID
    );

    console.log("⚙️ Program State:", programState.toString());

    // 5. Get program state to find admin (for fee payment)
    try {
      const programStateAccount = await program.account.programState.fetch(
        programState
      );
      console.log(
        "💵 Minting fee will be paid to:",
        programStateAccount.admin.toString()
      );
      console.log(
        "💰 Minting price:",
        programStateAccount.mintingPrice.toString(),
        "lamports"
      );
    } catch (error) {
      console.error(
        "❌ Error fetching program state. Make sure it's initialized:",
        error
      );
      return;
    }

    // 6. Derive metadata PDA
    const [metadata] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("metadata"),
        METADATA_PROGRAM_ID.toBuffer(),
        nftMint.toBuffer(),
      ],
      METADATA_PROGRAM_ID
    );

    console.log("📄 Metadata Account:", metadata.toString());

    // 7. Derive master edition PDA
    const [masterEdition] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("metadata"),
        METADATA_PROGRAM_ID.toBuffer(),
        nftMint.toBuffer(),
        Buffer.from("edition"),
      ],
      METADATA_PROGRAM_ID
    );

    console.log("🏆 Master Edition:", masterEdition.toString());

    // 8. Get program state again for mint fee account
    const programStateAccount = await program.account.programState.fetch(
      programState
    );
    const mintFeeAccount = programStateAccount.admin;

    console.log("💸 Preparing to mint NFT to collection...");

    // 9. Call mint_to_collection
    const tx = await program.methods
      .mintToCollection(idNft, name, symbol, uri)
      .accounts({
        authority: authority.publicKey,
        payer: authority.publicKey,
        // @ts-ignore
        mint: nftMint,
        tokenAccount: tokenAccount,
        programState: programState,
        mintFeeAccount: mintFeeAccount,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        metadataProgram: METADATA_PROGRAM_ID,
        masterEditionAccount: masterEdition,
        nftMetadata: metadata,
        collection: collectionMint,
        // Optional accounts for verification (set to null if not verifying immediately)
        collectionMetadata: null,
        collectionAuthority: null,
        collectionMasterEdition: null,
      })
      .signers([authority])
      .preInstructions([
        ComputeBudgetProgram.setComputeUnitLimit({ units: 400000 }),
      ])
      .rpc();
    console.log("✅ Transaction successful!");
    console.log("🔗 Transaction signature:", tx);
    console.log("🎨 NFT Mint Address:", nftMint.toString());
    console.log("💰 Token Account:", tokenAccount.toString());
    console.log("📦 Collection:", collectionMint.toString());

    // Wait for confirmation
    await connection.confirmTransaction(tx, "confirmed");
    console.log("✅ Transaction confirmed!");

    // Verify the NFT was minted
    try {
      const tokenAccountInfo = await connection.getTokenAccountBalance(
        tokenAccount
      );
      console.log("🎯 NFT Balance:", tokenAccountInfo.value.amount);
    } catch (error) {
      console.log(
        "ℹ️ Token account balance check failed (might not exist yet)"
      );
    }
  } catch (error) {
    console.error("❌ Error minting NFT to collection:", error);

    // Enhanced error logging
    if (error.error) {
      console.error("Program Error Code:", error.error.errorCode?.code);
      console.error("Program Error Name:", error.error.errorCode?.name);
      console.error("Program Error Message:", error.error.errorMessage);
    }

    if (error.logs) {
      console.error("Transaction Logs:");
      error.logs.forEach((log, index) => {
        console.error(`${index}: ${log}`);
      });
    }
  }
}

mintToCollection();
