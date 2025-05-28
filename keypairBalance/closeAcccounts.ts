const fs = require("fs");
const path = require("path");
const {
  Connection,
  Keypair,
  PublicKey,
  sendAndConfirmTransaction,
  Transaction,
  BpfLoader,
  BPF_LOADER_PROGRAM_ID,
  SystemProgram,
  LAMPORTS_PER_SOL,
} = require("@solana/web3.js");

// CONFIG
const KEYPAIR_DIR = path.join(__dirname, "keypairs"); // Folder containing keypairs
const RECIPIENT_ADDRESS = new PublicKey(
  "8tTXVRjeCMwfuTkZA3PgJwU8Fb5ddjTSAoAiXpNcnRB5"
); // Where to send recovered rent
const CLUSTER_URL = "https://api.devnet.solana.com"; // Change as needed
const connection = new Connection(CLUSTER_URL, "confirmed");

// BPF Loader Program IDs (different versions)
const BPF_LOADER_PROGRAM_IDS = [
  new PublicKey("BPFLoader2111111111111111111111111111111111"),
  new PublicKey("BPFLoader1111111111111111111111111111111111"),
  new PublicKey("BPFLoaderUpgradeab1e11111111111111111111111"),
];

// Read all keypair files
const loadKeypairs = () => {
  console.log(`Loading keypairs from ${KEYPAIR_DIR}...`);
  const files = fs.readdirSync(KEYPAIR_DIR);
  return files
    .map((file) => {
      const filePath = path.join(KEYPAIR_DIR, file);
      try {
        const secretKey = Uint8Array.from(
          JSON.parse(fs.readFileSync(filePath, "utf8"))
        );
        const keypair = Keypair.fromSecretKey(secretKey);
        console.log(
          `Loaded keypair: ${keypair.publicKey.toString()} from ${file}`
        );
        return { keypair, fileName: file };
      } catch (error) {
        console.error(`Error loading keypair from ${file}:`, error);
        return null;
      }
    })
    .filter(Boolean);
};

// Check if an account is a program
const isProgramAccount = async (publicKey) => {
  try {
    const accountInfo = await connection.getAccountInfo(publicKey);

    if (!accountInfo) {
      console.log(`Account ${publicKey.toString()} does not exist`);
      return { isProgram: false, accountInfo: null };
    }

    // Check if the account is owned by one of the BPF Loaders
    const isOwnedByLoader = BPF_LOADER_PROGRAM_IDS.some((loader) =>
      accountInfo.owner.equals(loader)
    );

    if (isOwnedByLoader) {
      console.log(`✅ Found program account: ${publicKey.toString()}`);
      console.log(
        `💰 Contains ${accountInfo.lamports / LAMPORTS_PER_SOL} SOL in rent`
      );
      return { isProgram: true, accountInfo };
    }

    return { isProgram: false, accountInfo };
  } catch (error) {
    console.error(`Error checking account ${publicKey.toString()}:`, error);
    return { isProgram: false, accountInfo: null };
  }
};

// Create transaction to close program
const createCloseTransaction = (
  programId,
  recipientAddress,
  upgradeAuthority
) => {
  // Create the close account instruction
  const transaction = new Transaction().add({
    keys: [
      { pubkey: programId, isSigner: false, isWritable: true },
      { pubkey: recipientAddress, isSigner: false, isWritable: true },
      { pubkey: upgradeAuthority.publicKey, isSigner: true, isWritable: false },
    ],
    programId: BPF_LOADER_PROGRAM_ID,
    data: Buffer.from([3, 0, 0, 0]), // Close program instruction (0x03)
  });

  return transaction;
};

// Attempt to close a program account
const closeProgram = async (programKeypair, recipientAddress) => {
  try {
    const programId = programKeypair.publicKey;
    console.log(`\n🔍 Attempting to close program: ${programId.toString()}`);

    // Check if this is actually a program account
    const { isProgram, accountInfo } = await isProgramAccount(programId);

    if (!isProgram) {
      console.log(`⚠️ This keypair is not a program account. Skipping.`);
      return false;
    }

    const rentAmount = accountInfo.lamports / LAMPORTS_PER_SOL;
    console.log(`💰 Program account has ${rentAmount} SOL in rent`);

    // Create transaction to close program
    const transaction = createCloseTransaction(
      programId,
      recipientAddress,
      programKeypair
    );

    // Get the latest blockhash
    const { blockhash } = await connection.getLatestBlockhash();
    transaction.recentBlockhash = blockhash;
    transaction.feePayer = programKeypair.publicKey;

    // Sign and send the transaction
    console.log(`📝 Sending transaction to close program...`);
    const signature = await sendAndConfirmTransaction(connection, transaction, [
      programKeypair,
    ]);

    console.log(`✅ Program closed successfully!`);
    console.log(`📝 Transaction signature: ${signature}`);
    console.log(
      `💰 Rent of ${rentAmount} SOL returned to: ${recipientAddress.toString()}`
    );

    // Verify the account has been closed
    const accountAfter = await connection.getAccountInfo(programId);
    if (!accountAfter) {
      console.log(`✅ Confirmed: Program account has been closed.`);
      return true;
    } else {
      console.log(
        `⚠️ Warning: Program account still exists. Close operation may have failed.`
      );
      return false;
    }
  } catch (error) {
    console.error("❌ Error closing program:", error);
    console.error(error.stack);
    return false;
  }
};

// Main function
const recoverProgramRent = async () => {
  // Load all keypairs
  const keypairs = loadKeypairs();
  console.log(`🔑 Loaded ${keypairs.length} keypairs`);

  // Attempt to close each keypair as a program
  let successCount = 0;
  let attemptCount = 0;

  for (const { keypair, fileName } of keypairs) {
    console.log(
      `\nProcessing keypair: ${keypair.publicKey.toString()} (${fileName})`
    );

    // Check account info first to avoid unnecessary attempts
    const { isProgram, accountInfo } = await isProgramAccount(
      keypair.publicKey
    );

    if (isProgram) {
      attemptCount++;
      const success = await closeProgram(keypair, RECIPIENT_ADDRESS);
      if (success) {
        successCount++;
      }
    } else if (accountInfo) {
      const balance = accountInfo.lamports / LAMPORTS_PER_SOL;
      console.log(`💰 Regular account with balance: ${balance} SOL`);
    }
  }

  console.log(`\n📊 Summary:`);
  console.log(`🔍 Found ${attemptCount} program accounts`);
  console.log(`✅ Successfully closed ${successCount} program accounts`);

  if (successCount > 0) {
    console.log(`💰 Rent has been sent to ${RECIPIENT_ADDRESS.toString()}`);
  }
};

// Run the script
recoverProgramRent();
