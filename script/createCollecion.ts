import * as anchor from "@coral-xyz/anchor";
import { Program, BN, AnchorProvider, Wallet } from "@coral-xyz/anchor";
import { NftProgram } from "../target/types/nft_program";
import idlJson from "../target/idl/nft_program.json";
import {
  Connection,
  Keypair,
  PublicKey,
  SystemProgram,
  SYSVAR_RENT_PUBKEY,
} from "@solana/web3.js";
import {
  TOKEN_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID,
  getAssociatedTokenAddress,
  MINT_SIZE,
  createInitializeMintInstruction,
} from "@solana/spl-token";
import * as fs from "fs";

// Metaplex Token Metadata Program ID
const METADATA_PROGRAM_ID = new PublicKey(
  "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
);

// Configure the connection to devnet
const connection = new Connection("https://api.devnet.solana.com", "confirmed");

// Load user wallet keypair
const walletKeypairFile = fs.readFileSync("./wallet-keypair.json", "utf-8");
const walletKeypair = Keypair.fromSecretKey(
  Buffer.from(JSON.parse(walletKeypairFile))
);
const payer = walletKeypair; // Assuming the loaded keypair is the payer and authority
const authority = walletKeypair;
console.log(
  `Using wallet public key (payer/authority): ${payer.publicKey.toString()}`
);

// Initialize the provider
const anchorProviderInstance: anchor.AnchorProvider = new anchor.AnchorProvider(
  connection,
  new anchor.Wallet(walletKeypair),
  { commitment: "confirmed" }
);
anchor.setProvider(anchorProviderInstance);

// Program ID from your lib.rs
const PROGRAM_ID = new PublicKey(
  "GBRUTbNjxd7L8pSw14FEfsGPKkVz8rRhKyWiFFh4xkVC"
);

// Program
const program = anchor.workspace.NftProgram as Program<NftProgram>;

// Helper function to find PDA for metadata account
const getMetadataPDA = (mint: PublicKey): PublicKey => {
  const [pda] = PublicKey.findProgramAddressSync(
    [Buffer.from("metadata"), METADATA_PROGRAM_ID.toBuffer(), mint.toBuffer()],
    METADATA_PROGRAM_ID
  );
  return pda;
};

// Helper function to find PDA for master edition account
const getMasterEditionPDA = (mint: PublicKey): PublicKey => {
  const [pda] = PublicKey.findProgramAddressSync(
    [
      Buffer.from("metadata"),
      METADATA_PROGRAM_ID.toBuffer(),
      mint.toBuffer(),
      Buffer.from("edition"),
    ],
    METADATA_PROGRAM_ID
  );
  return pda;
};

async function callCreateCollection(
  idCollection: BN,
  name: string,
  symbol: string,
  uri: string
) {
  console.log(`\nCreating new collection with ID: ${idCollection.toString()}`);
  console.log(`Name: ${name}, Symbol: ${symbol}, URI: ${uri}`);

  const [collectionMintPDA] = PublicKey.findProgramAddressSync(
    [Buffer.from("mint"), idCollection.toArrayLike(Buffer, "le", 8)],
    program.programId
  );
  console.log(`Derived Collection Mint PDA: ${collectionMintPDA.toString()}`);

  const tokenAccount = await getAssociatedTokenAddress(
    collectionMintPDA,
    payer.publicKey
  );
  console.log(`Derived Token Account for Payer: ${tokenAccount.toString()}`);

  const nftMetadata = getMetadataPDA(collectionMintPDA);
  const masterEditionAccount = getMasterEditionPDA(collectionMintPDA);

  try {
    const tx = await program.methods
      .createCollection(idCollection, name, symbol, uri)
      .accounts({
        authority: authority.publicKey,
        payer: payer.publicKey,
        // @ts-ignore
        mint: collectionMintPDA,
        tokenAccount: tokenAccount,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        rent: SYSVAR_RENT_PUBKEY,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        metadataProgram: METADATA_PROGRAM_ID,
        masterEditionAccount: masterEditionAccount,
        nftMetadata: nftMetadata,
      })
      .signers([payer]) // authority is also payer here, so payer signs
      .rpc();
    console.log(`Create Collection transaction signature: ${tx}`);
    console.log(`Collection Mint Address: ${collectionMintPDA.toString()}`);
    return collectionMintPDA;
  } catch (error) {
    console.error("Error creating collection:", error);
    throw error;
  }
}

async function callMintToCollection(
  collectionMintAddress: PublicKey,
  idNft: BN,
  name: string,
  symbol: string,
  uri: string
) {
  console.log(
    `\nMinting NFT (ID: ${idNft.toString()}) into collection: ${collectionMintAddress.toString()}`
  );
  console.log(`Name: ${name}, Symbol: ${symbol}, URI: ${uri}`);

  const [nftMintPDA] = PublicKey.findProgramAddressSync(
    [
      Buffer.from("item_mint"),
      collectionMintAddress.toBuffer(),
      idNft.toArrayLike(Buffer, "le", 8),
    ],
    program.programId
  );
  console.log(`Derived NFT Mint PDA: ${nftMintPDA.toString()}`);

  const tokenAccount = await getAssociatedTokenAddress(
    nftMintPDA,
    payer.publicKey
  );
  console.log(`Derived Token Account for NFT: ${tokenAccount.toString()}`);

  const nftMetadata = getMetadataPDA(nftMintPDA);
  const masterEditionAccount = getMasterEditionPDA(nftMintPDA);

  try {
    const tx = await program.methods
      .mintToCollection(idNft, name, symbol, uri, 0.0, new BN(0))
      .accounts({
        authority: authority.publicKey,
        payer: payer.publicKey,
        // @ts-ignore
        mint: nftMintPDA,
        tokenAccount: tokenAccount,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        rent: SYSVAR_RENT_PUBKEY,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        metadataProgram: METADATA_PROGRAM_ID,
        masterEditionAccount: masterEditionAccount,
        nftMetadata: nftMetadata,
        collection: collectionMintAddress,
      })
      .signers([payer])
      .rpc();
    console.log(`Mint to Collection transaction signature: ${tx}`);
    console.log(`Minted NFT Address: ${nftMintPDA.toString()}`);
    return nftMintPDA;
  } catch (error) {
    console.error("Error minting to collection:", error);
    throw error;
  }
}

async function callCreateSingleNft(
  id: BN,
  name: string,
  symbol: string,
  uri: string,
  price: number,
  cant: BN
) {
  console.log(`\nCreating single NFT with ID: ${id.toString()}`);
  console.log(
    `Name: ${name}, Symbol: ${symbol}, URI: ${uri}, Price: ${price}, Cant: ${cant.toString()}`
  );

  const [nftMintPDA] = PublicKey.findProgramAddressSync(
    [Buffer.from("mint"), id.toArrayLike(Buffer, "le", 8)],
    program.programId
  );
  console.log(`Derived NFT Mint PDA: ${nftMintPDA.toString()}`);

  const tokenAccount = await getAssociatedTokenAddress(
    nftMintPDA,
    payer.publicKey
  );
  console.log(`Derived Token Account for NFT: ${tokenAccount.toString()}`);

  const nftMetadata = getMetadataPDA(nftMintPDA);
  const masterEditionAccount = getMasterEditionPDA(nftMintPDA);

  try {
    const tx = await program.methods
      .createSingleNft(id, name, symbol, uri, price, cant)
      .accounts({
        authority: authority.publicKey,
        payer: payer.publicKey,
        // @ts-ignore
        mint: nftMintPDA,
        tokenAccount: tokenAccount,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        rent: SYSVAR_RENT_PUBKEY,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        metadataProgram: METADATA_PROGRAM_ID,
        masterEditionAccount: masterEditionAccount,
        nftMetadata: nftMetadata,
      })
      .signers([payer])
      .rpc();
    console.log(`Create Single NFT transaction signature: ${tx}`);
    console.log(`Minted NFT Address: ${nftMintPDA.toString()}`);
    return nftMintPDA;
  } catch (error) {
    console.error("Error creating single NFT:", error);
    throw error;
  }
}

async function main() {
  try {
    console.log("Starting main script execution...");
    // --- 1. Create a new Collection ---
    const idCollection = new BN(Math.floor(Math.random() * 1000000) + 1);
    const collectionName = "FLEXXTOKEN";
    const collectionSymbol = "FLEXX";
    const collectionUri =
      "https://rose-causal-albatross-891.mypinata.cloud/ipfs/QmTcL1QD4jghCBstdTyc41hQahNYRSpigbYizXRgzJ17Xe";

    // const createdCollectionMintAddress = await callCreateCollection(
    //   idCollection,
    //   collectionName,
    //   collectionSymbol,
    //   collectionUri
    // );

    // if (createdCollectionMintAddress) {
    //   console.log(
    //     `Successfully created collection: ${createdCollectionMintAddress.toString()}`
    //   );
    // const createdCollectionMintAddress = new PublicKey(
    //   "72RPDJH6CKF2yPiiRkkvUf6Ya5nM6aVirMQj1gPa4DBW"
    // );
    // //   // --- 2. Mint an NFT into that Collection ---
    // const idNftInCollection = new BN(Math.floor(Math.random() * 1000000) + 1);
    // const nftNameInCollection = "SAYAPA";
    // const nftSymbolInCollection = "SPA";
    // const nftUriInCollection =
    //   "https://rose-causal-albatross-891.mypinata.cloud/ipfs/QmRyeAFuBBewezkZhucubKLxTG8UNHwvAX6YCWsc47uy5Y";

    // await callMintToCollection(
    //   createdCollectionMintAddress,
    //   idNftInCollection,
    //   nftNameInCollection,
    //   nftSymbolInCollection,
    //   nftUriInCollection
    // );

    //   const idNftInCollection2 = new BN(
    //     Math.floor(Math.random() * 1000000) + 2
    //   );
    //   const nftNameInCollection2 = "My Second NFT in Collection";
    //   const nftSymbolInCollection2 = "NFTC2";
    //   const nftUriInCollection2 =
    //     "https://arweave.net/your-second-nft-in-collection-metadata-uri";

    //   await callMintToCollection(
    //     createdCollectionMintAddress,
    //     idNftInCollection2,
    //     nftNameInCollection2,
    //     nftSymbolInCollection2,
    //     nftUriInCollection2
    //   );
    // }

    // // --- 3. Create a Single (Standalone) NFT ---
    const idSingleNft = new BN(Math.floor(Math.random() * 1000000) + 3);
    const singleNftName = "DEVDEAD";
    const singleNftSymbol = "DDT";
    const singleNftUri =
      "https://rose-causal-albatross-891.mypinata.cloud/ipfs/QmSxT8wTiXcpamyzjqjGocqerZeDEgvPSz8DZpa6QqP8hm";
    const singleNftPrice = 1.5;
    const singleNftCant = new BN(1);

    await callCreateSingleNft(
      idSingleNft,
      singleNftName,
      singleNftSymbol,
      singleNftUri,
      singleNftPrice,
      singleNftCant
    );
    // }
  } catch (error) {
    console.error("Error in main execution:", error);
  }
}

// main().then(() => console.log("Script finished."));

console.log("Script finished.");
