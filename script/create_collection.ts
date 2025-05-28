import * as anchor from "@coral-xyz/anchor";
import { Connection, Keypair, PublicKey, SystemProgram } from "@solana/web3.js";
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
  "48Afa15ypgAHQr7qNm2QqW8WL114Ynwer556CV9chARa"
);
const connection = new Connection("https://api.devnet.solana.com", "confirmed");

// Load the admin keypair (deployer or authorized admin)
const walletKeypairFile = fs.readFileSync("./wallet-keypair.json", "utf-8");
const walletKeypair = Keypair.fromSecretKey(
  Buffer.from(JSON.parse(walletKeypairFile))
);
const authority = walletKeypair;

async function main() {
  // Set up the provider
  const provider = new anchor.AnchorProvider(
    connection,
    new anchor.Wallet(walletKeypair),
    { commitment: "confirmed" }
  );

  // Load the program
  const program = new anchor.Program<NftProgram>(idl as NftProgram, provider);

  // Define seed prefixes (must match lib.rs)
  const PROGRAM_SEED_PREFIX = Buffer.from("nft_marketplace_v1");
  const PROGRAM_STATE_SEED = Buffer.from("program_state");
  const COLLECTION_MINT_SEED_PREFIX = Buffer.from("collection_mint");

  // Derive the program_state PDA
  const [programStatePda] = await PublicKey.findProgramAddressSync(
    [PROGRAM_SEED_PREFIX, PROGRAM_STATE_SEED, PROGRAM_ID.toBuffer()],
    PROGRAM_ID
  );

  // Collection parameters
  const idCollection = new anchor.BN(1); // Unique ID for the collection
  const name = "Turkey Day";
  const symbol = "Turkey";
  const uri =
    "https://rose-causal-albatross-891.mypinata.cloud/ipfs/bafkreibsbjymmiwvbwhl2bzspkksbguk7yjxkbbscqb3b3t6y4evfhq7pe"; // Metadata URI

  // Derive the mint PDA for the collection
  const [mintPda] = await PublicKey.findProgramAddressSync(
    [
      PROGRAM_SEED_PREFIX,
      COLLECTION_MINT_SEED_PREFIX,
      PROGRAM_ID.toBuffer(),
      Buffer.from(idCollection.toArray("le", 8)), // Convert number[] to Buffer
    ],
    PROGRAM_ID
  );

  // Derive the associated token account for the collection mint
  const tokenAccount = getAssociatedTokenAddressSync(
    mintPda,
    authority.publicKey,
    false,
    TOKEN_PROGRAM_ID,
    ASSOCIATED_TOKEN_PROGRAM_ID
  );

  // Derive metadata and master edition accounts
  const metadataProgram = new PublicKey(
    "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
  ); // Metaplex Token Metadata program
  const [metadataPda] = await PublicKey.findProgramAddressSync(
    [Buffer.from("metadata"), metadataProgram.toBuffer(), mintPda.toBuffer()],
    metadataProgram
  );

  const [masterEditionPda] = await PublicKey.findProgramAddressSync(
    [
      Buffer.from("metadata"),
      metadataProgram.toBuffer(),
      mintPda.toBuffer(),
      Buffer.from("edition"),
    ],
    metadataProgram
  );

  // Call the create_collection instruction
  try {
    const tx = await program.methods
      .createCollection(idCollection, name, symbol, uri)
      .accounts({
        authority: authority.publicKey,
        payer: authority.publicKey,
        // @ts-ignore
        mint: mintPda,
        tokenAccount: tokenAccount,
        programState: programStatePda,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        metadataProgram: metadataProgram,
        masterEditionAccount: masterEditionPda,
        nftMetadata: metadataPda,
      })
      .signers([authority])
      .rpc();

    console.log(
      `Collection created successfully with ID ${idCollection}. Tx: ${tx}`
    );
    console.log(`Mint PDA: ${mintPda.toBase58()}`);
    console.log(`Token Account: ${tokenAccount.toBase58()}`);
    console.log(`Metadata PDA: ${metadataPda.toBase58()}`);
    console.log(`Master Edition PDA: ${masterEditionPda.toBase58()}`);
  } catch (err) {
    console.error("Error creating collection:", err);
  }
}

main().catch((err) => {
  console.error("Error:", err);
});
