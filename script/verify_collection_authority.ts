import * as anchor from "@coral-xyz/anchor";
import { Connection, Keypair, PublicKey } from "@solana/web3.js";
import idl from "../target/idl/nft_program.json"; // Adjust path to your IDL
import type { NftProgram } from "../target/types/nft_program"; // Adjust path to your IDL

const PROGRAM_ID = new PublicKey(
  "48Afa15ypgAHQr7qNm2QqW8WL114Ynwer556CV9chARa"
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

  const collectionMint = new PublicKey("YOUR_COLLECTION_MINT_PUBLIC_KEY"); // Replace

  const [collectionMetadataPda] = await PublicKey.findProgramAddress(
    [
      Buffer.from("metadata"),
      METAPLEX_PROGRAM_ID.toBuffer(),
      collectionMint.toBuffer(),
    ],
    METAPLEX_PROGRAM_ID
  );

  const tx = await program.methods
    .verifyCollectionAuthority()
    .accounts({
      collectionAuthority: collectionAuthority.publicKey,
      collectionMetadata: collectionMetadataPda,
    })
    .signers([collectionAuthority])
    .rpc();

  console.log(`Collection authority verified with transaction: ${tx}`);
}

main().catch((err) => {
  console.error("Error:", err);
});
