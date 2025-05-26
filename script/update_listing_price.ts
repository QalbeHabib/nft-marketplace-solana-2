import * as anchor from "@coral-xyz/anchor";
import { Connection, Keypair, PublicKey, SystemProgram } from "@solana/web3.js";
import idl from "../target/idl/nft_program.json"; // Adjust path to your IDL
import type { NftProgram } from "../target/types/nft_program"; // Adjust path to your IDL

const PROGRAM_ID = new PublicKey("QaQX5WUroY6mHE8RPXXiQUnU73YFRVwKGkSaFcFj6yw");
const connection = new Connection("https://api.devnet.solana.com", "confirmed");

async function main() {
  const seller = Keypair.generate();
  const airdropSignature = await connection.requestAirdrop(
    seller.publicKey,
    1_000_000_000
  );
  await connection.confirmTransaction(airdropSignature);

  const wallet = new anchor.Wallet(seller);
  const provider = new anchor.AnchorProvider(connection, wallet, {
    commitment: "confirmed",
  });
  anchor.setProvider(provider);

  const program = new anchor.Program<NftProgram>(idl as NftProgram, provider);

  const PROGRAM_SEED_PREFIX = Buffer.from("nft_marketplace_v1");
  const LISTING_SEED_PREFIX = Buffer.from("listing");

  const mint = new PublicKey("YOUR_NFT_MINT_PUBLIC_KEY"); // Replace
  const listingId = new anchor.BN(1);
  const newPrice = new anchor.BN(2_000_000_000); // 2 SOL

  const [listingPda] = await PublicKey.findProgramAddress(
    [
      PROGRAM_SEED_PREFIX,
      LISTING_SEED_PREFIX,
      PROGRAM_ID.toBuffer(),
      mint.toBuffer(),
      seller.publicKey.toBuffer(),
      listingId.toArrayLike(Buffer, "le", 8),
    ],
    PROGRAM_ID
  );

  const tx = await program.methods
    .updateListingPrice(newPrice)
    .accounts({
      seller: seller.publicKey,
      listing: listingPda,
    })
    .signers([seller])
    .rpc();

  console.log(`Listing price updated with transaction: ${tx}`);
}

main().catch((err) => {
  console.error("Error:", err);
});
