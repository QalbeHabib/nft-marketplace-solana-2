import * as anchor from "@coral-xyz/anchor";
import {
  ComputeBudgetProgram,
  Connection,
  Keypair,
  PublicKey,
  Transaction,
} from "@solana/web3.js";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";
import idl from "../target/idl/nft_program.json"; // Adjust path to your IDL
import type { NftProgram } from "../target/types/nft_program"; // Adjust path to your IDL
import * as fs from "fs";
import {
  Metadata,
  createApproveCollectionAuthorityInstruction,
} from "@metaplex-foundation/mpl-token-metadata";

const PROGRAM_ID = new PublicKey("QaQX5WUroY6mHE8RPXXiQUnU73YFRVwKGkSaFcFj6yw");
const METAPLEX_PROGRAM_ID = new PublicKey(
  "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
);
const connection = new Connection("https://api.devnet.solana.com", "confirmed");

async function setAndVerifyCollection() {
  try {
    // Load the wallet keypair
    const walletKeypairFile = fs.readFileSync("./wallet-keypair.json", "utf-8");
    const walletKeypair = Keypair.fromSecretKey(
      Buffer.from(JSON.parse(walletKeypairFile))
    );
    const payer = walletKeypair;
    const updateAuthority = walletKeypair; // Adjust if different
    const collectionAuthority = walletKeypair; // Adjust if different

    // Check payer balance and airdrop SOL if needed
    const balance = await connection.getBalance(payer.publicKey);
    console.log(
      "💰 Payer Balance:",
      balance / anchor.web3.LAMPORTS_PER_SOL,
      "SOL"
    );
    if (balance < 1_000_000_000) {
      console.log("Requesting airdrop for payer...");
      const airdropSignature = await connection.requestAirdrop(
        payer.publicKey,
        1_000_000_000 // 1 SOL
      );
      await connection.confirmTransaction(airdropSignature, "confirmed");
      console.log("✅ Airdrop confirmed!");
    }

    // Set up provider
    const provider = new anchor.AnchorProvider(
      connection,
      new anchor.Wallet(payer),
      { commitment: "confirmed" }
    );
    anchor.setProvider(provider);

    // Load program
    const program = new anchor.Program<NftProgram>(idl as NftProgram, provider);

    // Specify NFT mint and collection mint
    const nftMint = new PublicKey(
      "Cm2D2A7YvdTMqfY7A8rDqsiJNKYo3C7bfGNoLFaCripF"
    );
    const collectionMint = new PublicKey(
      "7M3sDVLX9oct3vkLFS9WkNSKLZ2E1TdRixm8DwHhSSc7"
    );

    // Verify mint accounts exist
    const nftMintInfo = await connection.getAccountInfo(nftMint);
    if (!nftMintInfo) {
      throw new Error("❌ NFT mint does not exist!");
    }
    const collectionMintInfo = await connection.getAccountInfo(collectionMint);
    if (!collectionMintInfo) {
      throw new Error("❌ Collection mint does not exist!");
    }
    console.log("✅ Mint accounts verified!");

    // Derive metadata PDA for NFT
    const [nftMetadataPda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("metadata"),
        METAPLEX_PROGRAM_ID.toBuffer(),
        nftMint.toBuffer(),
      ],
      METAPLEX_PROGRAM_ID
    );
    console.log("📄 NFT Metadata PDA:", nftMetadataPda.toString());

    // Derive metadata PDA for collection
    const [collectionMetadataPda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("metadata"),
        METAPLEX_PROGRAM_ID.toBuffer(),
        collectionMint.toBuffer(),
      ],
      METAPLEX_PROGRAM_ID
    );
    console.log(
      "📦 Collection Metadata PDA:",
      collectionMetadataPda.toString()
    );

    // Derive master edition PDA for collection
    const [collectionMasterEditionPda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("metadata"),
        METAPLEX_PROGRAM_ID.toBuffer(),
        collectionMint.toBuffer(),
        Buffer.from("edition"),
      ],
      METAPLEX_PROGRAM_ID
    );
    console.log(
      "🏆 Collection Master Edition PDA:",
      collectionMasterEditionPda.toString()
    );

    // Derive collection authority record PDA
    const [collectionAuthorityRecordPda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("metadata"),
        METAPLEX_PROGRAM_ID.toBuffer(),
        collectionMint.toBuffer(),
        Buffer.from("collection_authority"),
        collectionAuthority.publicKey.toBuffer(),
      ],
      METAPLEX_PROGRAM_ID
    );
    console.log(
      "🔐 Collection Authority Record PDA:",
      collectionAuthorityRecordPda.toString()
    );

    // Verify metadata accounts exist
    const nftMetadataInfo = await connection.getAccountInfo(nftMetadataPda);
    if (!nftMetadataInfo) {
      throw new Error("❌ NFT metadata account does not exist!");
    }
    const collectionMetadataInfo = await connection.getAccountInfo(
      collectionMetadataPda
    );
    if (!collectionMetadataInfo) {
      throw new Error("❌ Collection metadata account does not exist!");
    }
    const collectionMasterEditionInfo = await connection.getAccountInfo(
      collectionMasterEditionPda
    );
    if (!collectionMasterEditionInfo) {
      throw new Error("❌ Collection master edition account does not exist!");
    }
    console.log("✅ Metadata and master edition accounts verified!");

    // Fetch and verify metadata using Metaplex SDK
    const nftMetadata = await Metadata.fromAccountAddress(
      connection,
      nftMetadataPda
    );
    if (
      !nftMetadata.collection ||
      !nftMetadata.collection.key.equals(collectionMint)
    ) {
      throw new Error(
        "❌ NFT metadata does not reference the correct collection!"
      );
    }
    if (nftMetadata.collection.verified) {
      throw new Error("❌ NFT is already verified in the collection!");
    }
    console.log("✅ NFT metadata collection field verified!");

    const collectionMetadata = await Metadata.fromAccountAddress(
      connection,
      collectionMetadataPda
    );
    if (!collectionMetadata.collectionDetails) {
      throw new Error(
        "❌ Collection metadata is not configured as a collection (missing collectionDetails)!"
      );
    }
    if (!collectionMetadata.updateAuthority.equals(updateAuthority.publicKey)) {
      throw new Error(
        `❌ Update authority mismatch! Expected: ${updateAuthority.publicKey}, Found: ${collectionMetadata.updateAuthority}`
      );
    }
    console.log("✅ Collection metadata and update authority verified!");

    // Check if collection authority record exists
    const collectionAuthorityRecordInfo = await connection.getAccountInfo(
      collectionAuthorityRecordPda
    );
    if (!collectionAuthorityRecordInfo) {
      console.log("🔐 Approving collection authority...");
      const approveTx = new Transaction().add(
        createApproveCollectionAuthorityInstruction({
          collectionAuthorityRecord: collectionAuthorityRecordPda,
          newCollectionAuthority: collectionAuthority.publicKey,
          updateAuthority: updateAuthority.publicKey,
          payer: payer.publicKey,
          metadata: collectionMetadataPda,
          mint: collectionMint,
        })
      );
      const approveTxSignature = await provider.sendAndConfirm(approveTx, [
        payer,
        updateAuthority,
      ]);
      console.log("✅ Collection authority approved! Tx:", approveTxSignature);
    } else {
      console.log("✅ Collection authority record already exists!");
    }

    // Execute set_and_verify_collection transaction
    console.log("🚀 Setting and verifying collection...");
    const tx = await program.methods
      .setAndVerifyCollection(collectionMint)
      .accounts({
        payer: payer.publicKey,
        updateAuthority: updateAuthority.publicKey,
        collectionAuthority: collectionAuthority.publicKey,
        nftMetadata: nftMetadataPda,
        collectionMint: collectionMint,
        collectionMetadata: collectionMetadataPda,
        collectionMasterEdition: collectionMasterEditionPda,
        collectionAuthorityRecord: collectionAuthorityRecordPda,
        metadataProgram: METAPLEX_PROGRAM_ID,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      })
      .signers([payer, collectionAuthority, updateAuthority])
      .preInstructions([
        ComputeBudgetProgram.setComputeUnitLimit({ units: 600000 }), // Increased for safety
      ])
      .rpc();

    console.log("✅ Transaction successful!");
    console.log("🔗 Transaction signature:", tx);
    console.log("🎨 NFT Mint:", nftMint.toString());
    console.log("📦 Collection Mint:", collectionMint.toString());
  } catch (err) {
    console.error("❌ Error setting and verifying collection:", err);
    if (err.transactionLogs) {
      console.error("Transaction Logs:");
      err.transactionLogs.forEach((log, index) => {
        console.error(`${index}: ${log}`);
      });
      if (
        err.transactionLogs.some((log) => log.includes("Derived key invalid"))
      ) {
        console.error(
          "❌ Derived key invalid: Ensure collectionAuthorityRecord is initialized and collection metadata has collectionDetails set."
        );
      }
    }
    throw err;
  }
}

setAndVerifyCollection();
