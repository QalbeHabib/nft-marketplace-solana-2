import * as anchor from "@coral-xyz/anchor";
import { Connection, Keypair, PublicKey, SystemProgram } from "@solana/web3.js";
import idl from "../target/idl/nft_program.json"; // Adjust path to your IDL
import type { NftProgram } from "../target/types/nft_program"; // Adjust path to your IDL

const PROGRAM_ID = new PublicKey(
  "48Afa15ypgAHQr7qNm2QqW8WL114Ynwer556CV9chARa"
);
const connection = new Connection("https://api.devnet.solana.com", "confirmed");

async function main() {
  const buyer = Keypair.generate();
  const airdropSignature = await connection.requestAirdrop(
    buyer.publicKey,
    1_000_000_000
  );
  await connection.confirmTransaction(airdropSignature);

  const wallet = new anchor.Wallet(buyer);
  const provider = new anchor.AnchorProvider(connection, wallet, {
    commitment: "confirmed",
  });
  anchor.setProvider(provider);

  const program = new anchor.Program<NftProgram>(idl as NftProgram, provider);

  const PROGRAM_SEED_PREFIX = Buffer.from("nft_marketplace_v1");
  const OFFER_SEED_PREFIX = Buffer.from("offer");

  const mint = new PublicKey("YOUR_NFT_MINT_PUBLIC_KEY"); // Replace
  const offerId = new anchor.BN(1);

  const [offerPda] = await PublicKey.findProgramAddress(
    [
      PROGRAM_SEED_PREFIX,
      OFFER_SEED_PREFIX,
      PROGRAM_ID.toBuffer(),
      mint.toBuffer(),
      buyer.publicKey.toBuffer(),
      offerId.toArrayLike(Buffer, "le", 8),
    ],
    PROGRAM_ID
  );

  const tx = await program.methods
    .cancelOffer()
    .accounts({
      buyer: buyer.publicKey,
      offer: offerPda,
    })
    .signers([buyer])
    .rpc();

  console.log(`Offer canceled with transaction: ${tx}`);
}

main().catch((err) => {
  console.error("Error:", err);
});
