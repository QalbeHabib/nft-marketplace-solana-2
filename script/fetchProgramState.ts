import * as anchor from "@coral-xyz/anchor";
import {
  Connection,
  Keypair,
  PublicKey,
  SystemProgram,
  ComputeBudgetProgram,
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
// const connection = new Connection("https://api.devnet.solana.com", "confirmed");
const connection = new Connection(
  "https://staging-rpc.dev2.eclipsenetwork.xyz",
  "confirmed"
);

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
  const COLLECTION_ITEM_SEED_PREFIX = Buffer.from("collection_item");

  // Collection parameters (for "TurkeyDay" collection)
  const idCollection = new anchor.BN(1); // Collection ID used in create_collection

  const [programState] = PublicKey.findProgramAddressSync(
    [PROGRAM_SEED_PREFIX, PROGRAM_STATE_SEED, PROGRAM_ID.toBuffer()],
    PROGRAM_ID
  );
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

  //   // Derive the collection mint PDA
  //   const [collectionMintPda] = await PublicKey.findProgramAddressSync(
  //     [
  //       PROGRAM_SEED_PREFIX,
  //       COLLECTION_MINT_SEED_PREFIX,
  //       PROGRAM_ID.toBuffer(),
  //       Buffer.from(idCollection.toArray("le", 8)),
  //     ],
  //     PROGRAM_ID
  //   );

  //   // Derive the program_state PDA
  //   const [programStatePda] = await PublicKey.findProgramAddressSync(
  //     [PROGRAM_SEED_PREFIX, PROGRAM_STATE_SEED, PROGRAM_ID.toBuffer()],
  //     PROGRAM_ID
  //   );

  //   // NFT parameters
  //   const idNft = new anchor.BN(1); // Unique ID for the NFT
  //   const name = "$BABYELIPHANT";
  //   const symbol = "BELI";
  //   const uri =
  //     "https://rose-causal-albatross-891.mypinata.cloud/ipfs/QmPZTtrNPkLPyEkYFtReQ2r9NM5r54KNPrQiEfPE6cfrxW"; // Metadata URI for the NFT
  //   //   Derive the mint PDA for the NFT
  //   const [mintPda] = await PublicKey.findProgramAddressSync(
  //     [
  //       PROGRAM_SEED_PREFIX,
  //       COLLECTION_ITEM_SEED_PREFIX,
  //       PROGRAM_ID.toBuffer(),
  //       collectionMintPda.toBuffer(),
  //       Buffer.from(idNft.toArray("le", 8)),
  //     ],
  //     PROGRAM_ID
  //   );

  //   // Derive the associated token account for the NFT
  //   const tokenAccount = getAssociatedTokenAddressSync(
  //     mintPda,
  //     authority.publicKey,
  //     false,
  //     TOKEN_PROGRAM_ID,
  //     ASSOCIATED_TOKEN_PROGRAM_ID
  //   );

  //   // Derive metadata and master edition accounts
  //   const metadataProgram = new PublicKey(
  //     "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
  //   ); // Metaplex Token Metadata program
  //   const [metadataPda] = await PublicKey.findProgramAddressSync(
  //     [Buffer.from("metadata"), metadataProgram.toBuffer(), mintPda.toBuffer()],
  //     metadataProgram
  //   );

  //   const [masterEditionPda] = await PublicKey.findProgramAddressSync(
  //     [
  //       Buffer.from("metadata"),
  //       metadataProgram.toBuffer(),
  //       mintPda.toBuffer(),
  //       Buffer.from("edition"),
  //     ],
  //     metadataProgram
  //   );

  //   // Derive collection metadata and master edition accounts
  //   const [collectionMetadataPda] = await PublicKey.findProgramAddressSync(
  //     [
  //       Buffer.from("metadata"),
  //       metadataProgram.toBuffer(),
  //       collectionMintPda.toBuffer(),
  //     ],
  //     metadataProgram
  //   );

  //   const [collectionMasterEditionPda] = await PublicKey.findProgramAddressSync(
  //     [
  //       Buffer.from("metadata"),
  //       metadataProgram.toBuffer(),
  //       collectionMintPda.toBuffer(),
  //       Buffer.from("edition"),
  //     ],
  //     metadataProgram
  //   );

  //   // Derive the mint fee account (admin’s account for receiving minting fees)
  //   const mintFeeAccount = authority.publicKey; // Assuming the admin is the authority

  //   // Create a transaction with increased compute budget
  //   const computeBudgetIx = ComputeBudgetProgram.setComputeUnitLimit({
  //     units: 400000, // Increase to 400,000 CUs
  //   });

  //   // Build the mint_and_verify_to_collection instruction
  //   const mintIx = await program.methods
  //     .mintAndVerifyToCollection(idNft, name, symbol, uri)
  //     .accounts({
  //       authority: authority.publicKey,
  //       payer: authority.publicKey,
  //       collectionAuthority: authority.publicKey, // Assuming authority is also the collection authority
  //       // @ts-ignore
  //       mint: mintPda,
  //       tokenAccount: tokenAccount,
  //       programState: programStatePda,
  //       mintFeeAccount: mintFeeAccount,
  //       associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
  //       rent: anchor.web3.SYSVAR_RENT_PUBKEY,
  //       systemProgram: SystemProgram.programId,
  //       tokenProgram: TOKEN_PROGRAM_ID,
  //       metadataProgram: metadataProgram,
  //       masterEditionAccount: masterEditionPda,
  //       nftMetadata: metadataPda,
  //       collectionMint: collectionMintPda,
  //       collectionMetadata: collectionMetadataPda,
  //       collectionMasterEdition: collectionMasterEditionPda,
  //     })
  //     .instruction();

  // Create and send the transaction
  try {
    // const tx = new anchor.web3.Transaction().add(computeBudgetIx).add(mintIx);
    // const signature = await provider.sendAndConfirm(tx, [authority]);
    // console.log(
    //   `NFT minted and verified in TurkeyDay collection with ID ${idNft}. Tx: ${signature}`
    // );
    // console.log(`Mint PDA: ${mintPda.toBase58()}`);
    // console.log(`Token Account: ${tokenAccount.toBase58()}`);
    // console.log(`Metadata PDA: ${metadataPda.toBase58()}`);
    // console.log(`Master Edition PDA: ${masterEditionPda.toBase58()}`);
    // console.log(`Collection Mint PDA: ${collectionMintPda.toBase58()}`);
  } catch (err) {
    console.error("Error minting NFT to collection:", err);
    if (err instanceof anchor.web3.SendTransactionError) {
      console.error("Transaction Logs:", await err.getLogs(connection));
    }
  }
}

main().catch((err) => {
  console.error("Error:", err);
});
