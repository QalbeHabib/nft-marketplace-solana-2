import * as anchor from "@coral-xyz/anchor";
import { Connection, Keypair, PublicKey } from "@solana/web3.js";
import idl from "../target/idl/nft_program.json"; // Adjust path to your IDL
import type { NftProgram } from "../target/types/nft_program"; // Adjust path to your IDL

const PROGRAM_ID = new PublicKey("QaQX5WUroY6mHE8RPXXiQUnU73YFRVwKGkSaFcFj6yw");
const METAPLEX_PROGRAM_ID = new PublicKey(
  "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
);
const connection = new Connection("https://api.devnet.solana.com", "confirmed");

async function main() {
  // Load or generate keypair for collection authority
  const collectionAuthority = Keypair.generate(); // Replace with actual authority keypair

  // Airdrop SOL to collection authority
  const airdropSignature = await connection.requestAirdrop(
    collectionAuthority.publicKey,
    1_000_000_000
  );
  await connection.confirmTransaction(airdropSignature);

  // Set up provider
  const wallet = new anchor.Wallet(collectionAuthority);
  const provider = new anchor.AnchorProvider(connection, wallet, {
    commitment: "confirmed",
  });
  anchor.setProvider(provider);

  // Load program
  const program = new anchor.Program<NftProgram>(idl as NftProgram, provider);

  // Specify NFT mint (from mint_to_collection.ts or create_single_nft.ts)
  const nftMint = new PublicKey("YOUR_NFT_MINT_PUBLIC_KEY"); // Replace with actual NFT mint

  // Specify collection mint (from create_collection.ts)
  const collectionMint = new PublicKey("YOUR_COLLECTION_MINT_PUBLIC_KEY"); // Replace with actual collection mint

  // Derive metadata PDA for NFT
  const [nftMetadataPda] = await PublicKey.findProgramAddress(
    [
      Buffer.from("metadata"),
      METAPLEX_PROGRAM_ID.toBuffer(),
      nftMint.toBuffer(),
    ],
    METAPLEX_PROGRAM_ID
  );

  // Derive metadata PDA for collection
  const [collectionMetadataPda] = await PublicKey.findProgramAddress(
    [
      Buffer.from("metadata"),
      METAPLEX_PROGRAM_ID.toBuffer(),
      collectionMint.toBuffer(),
    ],
    METAPLEX_PROGRAM_ID
  );

  // Derive master edition PDA for collection
  const [collectionMasterEditionPda] = await PublicKey.findProgramAddress(
    [
      Buffer.from("metadata"),
      METAPLEX_PROGRAM_ID.toBuffer(),
      collectionMint.toBuffer(),
      Buffer.from("edition"),
    ],
    METAPLEX_PROGRAM_ID
  );

  // Execute transaction
  const tx = await program.methods
    .verifyNftInCollection()
    .accounts({
      payer: collectionAuthority.publicKey,
      collectionAuthority: collectionAuthority.publicKey,
      nftMetadata: nftMetadataPda,
      collectionMint: collectionMint,
      collectionMetadata: collectionMetadataPda,
      collectionMasterEdition: collectionMasterEditionPda,
      // @ts-ignore
      metadataProgram: METAPLEX_PROGRAM_ID,
    })
    .signers([collectionAuthority])
    .rpc();

  console.log(`NFT verified in collection with transaction: ${tx}`);
}

main().catch((err) => {
  console.error("Error:", err);
});
