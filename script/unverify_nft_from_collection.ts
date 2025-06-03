import * as anchor from "@coral-xyz/anchor";
import { Connection, Keypair, PublicKey } from "@solana/web3.js";
import idl from "../target/idl/nft_program.json"; // Adjust path to your IDL
import type { NftProgram } from "../target/types/nft_program"; // Adjust path to your IDL

const PROGRAM_ID = new PublicKey(
  "Equiqs1Z5Q4F1gBuciqo6yrvqNERzwp5v9Fskhq2A5WB"
);
const METAPLEX_PROGRAM_ID = new PublicKey(
  "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
);
const connection = new Connection("https://api.devnet.solana.com", "confirmed");

async function main() {
  const collectionAuthority = Keypair.generate(); // Replace with actual keypair
  const airdropSignature = await connection.requestAirdrop(
    collectionAuthority.publicKey,
    1_000_000_000
  );
  await connection.confirmTransaction(airdropSignature);

  const wallet = new anchor.Wallet(collectionAuthority);
  const provider = new anchor.AnchorProvider(connection, wallet, {
    commitment: "confirmed",
  });
  anchor.setProvider(provider);

  const program = new anchor.Program<NftProgram>(idl as NftProgram, provider);

  const nftMint = new PublicKey("YOUR_NFT_MINT_PUBLIC_KEY"); // Replace
  const collectionMint = new PublicKey("YOUR_COLLECTION_MINT_PUBLIC_KEY"); // Replace

  const [nftMetadataPda] = await PublicKey.findProgramAddress(
    [
      Buffer.from("metadata"),
      METAPLEX_PROGRAM_ID.toBuffer(),
      nftMint.toBuffer(),
    ],
    METAPLEX_PROGRAM_ID
  );

  const [collectionMetadataPda] = await PublicKey.findProgramAddress(
    [
      Buffer.from("metadata"),
      METAPLEX_PROGRAM_ID.toBuffer(),
      collectionMint.toBuffer(),
    ],
    METAPLEX_PROGRAM_ID
  );

  const [collectionMasterEditionPda] = await PublicKey.findProgramAddress(
    [
      Buffer.from("metadata"),
      METAPLEX_PROGRAM_ID.toBuffer(),
      collectionMint.toBuffer(),
      Buffer.from("edition"),
    ],
    METAPLEX_PROGRAM_ID
  );

  const tx = await program.methods
    .unverifyNftFromCollection()
    .accounts({
      payer: collectionAuthority.publicKey,
      collectionAuthority: collectionAuthority.publicKey,
      nftMetadata: nftMetadataPda,
      collectionMint: collectionMint,
      collectionMetadata: collectionMetadataPda,
      collectionMasterEdition: collectionMasterEditionPda,
      //   @ts-ignore
      metadataProgram: METAPLEX_PROGRAM_ID,
    })
    .signers([collectionAuthority])
    .rpc();

  console.log(`NFT unverified from collection with transaction: ${tx}`);
}

main().catch((err) => {
  console.error("Error:", err);
});
