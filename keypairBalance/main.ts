// const fs = require("fs");
// const path = require("path");
// const {
//   Connection,
//   Keypair,
//   LAMPORTS_PER_SOL,
//   PublicKey,
//   SystemProgram,
//   sendAndConfirmTransaction,
//   Transaction,
// } = require("@solana/web3.js");

// // CONFIG
// const KEYPAIR_DIR = path.join(__dirname, "keypairs"); // Folder containing keypairs
// const DESTINATION_ADDRESS = new PublicKey(
//   "8tTXVRjeCMwfuTkZA3PgJwU8Fb5ddjTSAoAiXpNcnRB5"
// ); // Replace this
// const CLUSTER_URL = "https://api.devnet.solana.com"; // or devnet/testnet
// const connection = new Connection(CLUSTER_URL, "confirmed");

// // Read all keypair files
// const loadKeypairs = () => {
//   const files = fs.readdirSync(KEYPAIR_DIR);
//   return files.map((file) => {
//     const filePath = path.join(KEYPAIR_DIR, file);
//     const secretKey = Uint8Array.from(
//       JSON.parse(fs.readFileSync(filePath, "utf8"))
//     );
//     return Keypair.fromSecretKey(secretKey);
//   });
// };

// // Send SOL to destination address
// const sendBalance = async (fromKeypair, balance) => {
//   // Leave ~5000 lamports for fee
//   const amountToSend = balance - 5000;
//   if (amountToSend <= 0)
//     return console.log(
//       `🛑 Not enough balance to send from ${fromKeypair.publicKey}`
//     );

//   // Create a proper Transaction object
//   const transaction = new Transaction().add(
//     SystemProgram.transfer({
//       fromPubkey: fromKeypair.publicKey,
//       toPubkey: DESTINATION_ADDRESS,
//       lamports: amountToSend,
//     })
//   );

//   // Get the latest blockhash
//   const { blockhash } = await connection.getLatestBlockhash();
//   transaction.recentBlockhash = blockhash;
//   transaction.feePayer = fromKeypair.publicKey;

//   // Now send the properly constructed Transaction
//   const tx = await sendAndConfirmTransaction(connection, transaction, [
//     fromKeypair,
//   ]);

//   console.log(
//     `✅ Sent ${amountToSend / LAMPORTS_PER_SOL} SOL from ${
//       fromKeypair.publicKey
//     } in tx: ${tx}`
//   );
// };

// // Main logic
// (async () => {
//   const keypairs = loadKeypairs();
//   console.log(`🔑 Loaded ${keypairs.length} keypairs`);

//   for (const kp of keypairs) {
//     const balance = await connection.getBalance(kp.publicKey);
//     console.log(`💰 ${kp.publicKey}: ${balance / LAMPORTS_PER_SOL} SOL`);

//     if (balance > 0.001 * LAMPORTS_PER_SOL) {
//       await sendBalance(kp, balance);
//     } else {
//       console.log(`⚠️ Skipped ${kp.publicKey}, balance too low.`);
//     }
//   }
// })();
