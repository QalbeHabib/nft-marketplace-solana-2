import * as anchor from "@coral-xyz/anchor";
import { Connection, Keypair, PublicKey, SystemProgram } from "@solana/web3.js";
import idl from "../target/idl/nft_program.json"; // Adjust path to your IDL
import type { NftProgram } from "../target/types/nft_program"; // Adjust path to your IDL

const PROGRAM_ID = new PublicKey("QaQX5WUroY6mHE8RPXXiQUnU73YFRVwKGkSaFcFj6yw");
const METAPLEX_PROGRAM_ID = new PublicKey(
  "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
);
const connection = new Connection("https://api.devnet.solana.com", "confirmed");

async function main() {
  // Generate keypairs (in practice, load from a secure source)
  const authority = Keypair.generate();
  const payer = Keypair.generate();

  // Airdrop SOL to payer for fees
  const airdropSignature = await connection.requestAirdrop(
    payer.publicKey,
    5_000_000_000
  ); // 2 SOL
  await connection.confirmTransaction(airdropSignature);

  // Set up provider
  const wallet = new anchor.Wallet(payer);
  const provider = new anchor.AnchorProvider(connection, wallet, {
    commitment: "confirmed",
  });
  anchor.setProvider(provider);

  // Load program
  const program = new anchor.Program<NftProgram>(idl as NftProgram, provider);

  // Define seeds
  const PROGRAM_SEED_PREFIX = Buffer.from("nft_marketplace_v1");
  const MINT_SEED_PREFIX = Buffer.from("nft_mint");
  const PROGRAM_STATE_SEED = Buffer.from("program_state");

  // Derive program state PDA
  const [programStatePda] = await PublicKey.findProgramAddressSync(
    [PROGRAM_SEED_PREFIX, PROGRAM_STATE_SEED, PROGRAM_ID.toBuffer()],
    PROGRAM_ID
  );

  // Fetch program state to get admin (assumes program state is initialized)
  const programState = await program.account.programState.fetch(
    programStatePda
  );
  const adminPublicKey = programState.admin;

  // NFT parameters
  const id = new anchor.BN(1);
  const name = "My NFT";
  const symbol = "MNFT";
  const uri = "https://example.com/nft.json";

  // Derive mint PDA
  const [mintPda] = await PublicKey.findProgramAddressSync(
    [
      PROGRAM_SEED_PREFIX,
      MINT_SEED_PREFIX,
      PROGRAM_ID.toBuffer(),
      id.toArrayLike(Buffer, "le", 8),
    ],
    PROGRAM_ID
  );

  // Derive metadata PDA
  const [metadataPda] = await PublicKey.findProgramAddressSync(
    [
      Buffer.from("metadata"),
      METAPLEX_PROGRAM_ID.toBuffer(),
      mintPda.toBuffer(),
    ],
    METAPLEX_PROGRAM_ID
  );

  // Derive master edition PDA
  const [masterEditionPda] = await PublicKey.findProgramAddressSync(
    [
      Buffer.from("metadata"),
      METAPLEX_PROGRAM_ID.toBuffer(),
      mintPda.toBuffer(),
      Buffer.from("edition"),
    ],
    METAPLEX_PROGRAM_ID
  );

  // Get associated token account
  const tokenAccount = await anchor.utils.token.associatedAddress({
    mint: mintPda,
    owner: payer.publicKey,
  });

  // Execute transaction
  const tx = await program.methods
    .createSingleNft(id, name, symbol, uri)
    .accounts({
      authority: authority.publicKey,
      payer: payer.publicKey,
      //   @ts-ignore
      mint: mintPda,
      tokenAccount: tokenAccount,
      programState: programStatePda,
      mintFeeAccount: adminPublicKey,
      associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      systemProgram: SystemProgram.programId,
      tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
      metadataProgram: METAPLEX_PROGRAM_ID,
      masterEditionAccount: masterEditionPda,
      nftMetadata: metadataPda,
    })
    .signers([authority, payer])
    .rpc();

  console.log(`NFT created with transaction: ${tx}`);
}

main().catch((err) => {
  console.error("Error:", err);
});
