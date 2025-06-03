import * as anchor from "@coral-xyz/anchor";
import { Connection, Keypair, PublicKey, SystemProgram } from "@solana/web3.js";
import idl from "../target/idl/nft_program.json"; // Adjust path to your IDL
import type { NftProgram } from "../target/types/nft_program"; // Adjust path to your IDL

const PROGRAM_ID = new PublicKey(
  "Equiqs1Z5Q4F1gBuciqo6yrvqNERzwp5v9Fskhq2A5WB"
);
const connection = new Connection("https://api.devnet.solana.com", "confirmed");

async function main() {
  const admin = Keypair.generate();
  const airdropSignature = await connection.requestAirdrop(
    admin.publicKey,
    1_000_000_000
  );
  await connection.confirmTransaction(airdropSignature);

  const wallet = new anchor.Wallet(admin);
  const provider = new anchor.AnchorProvider(connection, wallet, {
    commitment: "confirmed",
  });
  anchor.setProvider(provider);
  const program = new anchor.Program<NftProgram>(idl as NftProgram, provider);

  const PROGRAM_SEED_PREFIX = Buffer.from("nft_marketplace_v1");
  const PROGRAM_STATE_SEED = Buffer.from("program_state");

  const [programStatePda] = await PublicKey.findProgramAddress(
    [PROGRAM_SEED_PREFIX, PROGRAM_STATE_SEED, PROGRAM_ID.toBuffer()],
    PROGRAM_ID
  );

  const newPrice = new anchor.BN(2_000_000); // 0.002 SOL

  const tx = await program.methods
    .setMintingPrice(newPrice)
    .accounts({
      programState: programStatePda,
      admin: admin.publicKey,
    })
    .signers([admin])
    .rpc();

  console.log(`Minting price updated with transaction: ${tx}`);
}

main().catch((err) => {
  console.error("Error:", err);
});
