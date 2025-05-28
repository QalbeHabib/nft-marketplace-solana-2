import * as anchor from "@coral-xyz/anchor";
import { Connection, Keypair, PublicKey, SystemProgram } from "@solana/web3.js";
import idl from "../target/idl/nft_program.json"; // Adjust path to your IDL
import type { NftProgram } from "../target/types/nft_program"; // Adjust path to your IDL
import * as fs from "fs";

const PROGRAM_ID = new PublicKey(
  "48Afa15ypgAHQr7qNm2QqW8WL114Ynwer556CV9chARa"
);
const connection = new Connection("https://api.devnet.solana.com", "confirmed");

const walletKeypairFile = fs.readFileSync("./wallet-keypair.json", "utf-8");
const walletKeypair = Keypair.fromSecretKey(
  Buffer.from(JSON.parse(walletKeypairFile))
);
const authority = walletKeypair;

async function main() {
  // Load or generate admin keypair (replace with actual keypair loading logic)
  const provider = new anchor.AnchorProvider(
    connection,
    new anchor.Wallet(walletKeypair),
    {
      commitment: "confirmed",
    }
  );

  const program = new anchor.Program<NftProgram>(idl as NftProgram, provider);

  const PROGRAM_SEED_PREFIX = Buffer.from("nft_marketplace_v1");
  const PROGRAM_STATE_SEED = Buffer.from("program_state");

  const [programStatePda] = await PublicKey.findProgramAddressSync(
    [PROGRAM_SEED_PREFIX, PROGRAM_STATE_SEED, PROGRAM_ID.toBuffer()],
    PROGRAM_ID
  );

  const mintingPrice = new anchor.BN(1000000); // 0.001 SOL in lamports

  const tx = await program.methods
    .initializeProgramState(mintingPrice)
    .accounts({
      // @ts-ignore
      programState: programStatePda,
      admin: authority.publicKey,
      expectedAuthority: authority.publicKey,
      systemProgram: SystemProgram.programId,
    })
    .signers([authority])
    .rpc();

  console.log(
    `Program state initialized with minting price ${mintingPrice} lamports. Tx: ${tx}`
  );
}

main().catch((err) => {
  console.error("Error:", err);
});
