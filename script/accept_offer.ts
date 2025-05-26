import * as anchor from "@coral-xyz/anchor";
import { Connection, Keypair, PublicKey, SystemProgram } from "@solana/web3.js";
import idl from "../target/idl/nft_program.json"; // Adjust path to your IDL
import type { NftProgram } from "../target/types/nft_program"; // Adjust path to your IDL

const PROGRAM_ID = new PublicKey("QaQX5WUroY6mHE8RPXXiQUnU73YFRVwKGkSaFcFj6yw");
const connection = new Connection("https://api.devnet.solana.com", "confirmed");

async function main() {
  const seller = Keypair.generate();
  const buyer = Keypair.generate();
  //   const airdropSignature = await connection.requestAirdrop(
  //     seller.publicKey,
  //     1_000_000_000
  //   );
  //   await connection.confirmTransaction(airdropSignature);

  const wallet = new anchor.Wallet(seller);
  const provider = new anchor.AnchorProvider(connection, wallet, {
    commitment: "confirmed",
  });
  anchor.setProvider(provider);

  const program = new anchor.Program<NftProgram>(idl as NftProgram, provider);

  const PROGRAM_SEED_PREFIX = Buffer.from("nft_marketplace_v1");
  const OFFER_SEED_PREFIX = Buffer.from("offer");

  const mint = new PublicKey("YOUR_NFT_MINT_PUBLIC_KEY"); // Replace
  const offerId = new anchor.BN(1);
  const marketplaceAuthority = new PublicKey("YOUR_MARKETPLACE_AUTHORITY"); // Replace

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
    .acceptOffer(marketplaceFeeBps)
    .accounts({
      seller: seller.publicKey,
      buyer: buyer.publicKey,
      marketplaceAuthority: marketplaceAuthority,
      mint: mint,
      sellerTokenAccount: sellerTokenAccount,
      //   @ts-ignore

      buyerTokenAccount: buyerTokenAccount,
      offer: offerPda,
      associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
      tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
      systemProgram: SystemProgram.programId,
    })
    .signers([seller])
    .rpc();

  console.log(`Offer accepted with transaction: ${tx}`);
}

main().catch((err) => {
  console.error("Error:", err);
});
