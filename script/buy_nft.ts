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
  const seller = Keypair.generate();
  const airdropSignature = await connection.requestAirdrop(
    buyer.publicKey,
    2_000_000_000
  );
  await connection.confirmTransaction(airdropSignature);

  const wallet = new anchor.Wallet(buyer);
  const provider = new anchor.AnchorProvider(connection, wallet, {
    commitment: "confirmed",
  });
  anchor.setProvider(provider);

  const program = new anchor.Program<NftProgram>(idl as NftProgram, provider);

  const PROGRAM_SEED_PREFIX = Buffer.from("nft_marketplace_v1");
  const LISTING_SEED_PREFIX = Buffer.from("listing");

  const mint = new PublicKey("YOUR_NFT_MINT_PUBLIC_KEY"); // Replace
  const listingId = new anchor.BN(1);
  const marketplaceAuthority = new PublicKey("YOUR_MARKETPLACE_AUTHORITY"); // Replace with program state admin

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

  const sellerTokenAccount = await anchor.utils.token.associatedAddress({
    mint: mint,
    owner: seller.publicKey,
  });

  const buyerTokenAccount = await anchor.utils.token.associatedAddress({
    mint: mint,
    owner: buyer.publicKey,
  });

  const marketplaceFeeBps = 500; // 5%

  const tx = await program.methods
    .buyNft(marketplaceFeeBps)
    .accounts({
      buyer: buyer.publicKey,
      seller: seller.publicKey,
      marketplaceAuthority: marketplaceAuthority,
      mint: mint,
      sellerTokenAccount: sellerTokenAccount,
      //   @ts-ignore

      buyerTokenAccount: buyerTokenAccount,
      listing: listingPda,
      associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
      tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
      systemProgram: SystemProgram.programId,
    })
    .signers([buyer])
    .rpc();

  console.log(`NFT purchased with transaction: ${tx}`);
}

main().catch((err) => {
  console.error("Error:", err);
});
