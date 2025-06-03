const axios = require("axios");

// Mock NFT minting webhook payload based on real logs
const mockNftMintPayload = [
  {
    transaction: {
      signatures: ["4gp5oCZbsBKJy4iSYzXxCcvbbSikFdRVH8qKWmrLWmAVMPB8b8A3yj9CpYZzNL75TFSs2EcwGYDY9cVXwUTr8cJL"],
      message: {
        accountKeys: [
          "SenderWalletAddress111111111111111111111111",
          "ReceiverWalletAddress11111111111111111111",
          "Equiqs1Z5Q4F1gBuciqo6yrvqNERzwp5v9Fskhq2A5WB", // Your program ID 
          "11111111111111111111111111111111", // System Program
          "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA", // Token Program
          "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s", // Metaplex
          "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL", // Associated Token Program
        ],
      },
    },
    meta: {
      logMessages: [
        "Program ComputeBudget111111111111111111111111111111 invoke [1]",
        "Program ComputeBudget111111111111111111111111111111 success",
        "Program ComputeBudget111111111111111111111111111111 invoke [1]",
        "Program ComputeBudget111111111111111111111111111111 success",
        "Program Equiqs1Z5Q4F1gBuciqo6yrvqNERzwp5v9Fskhq2A5WB invoke [1]",
        "Program log: Instruction: MintAndVerifyToCollection",
        "Program 11111111111111111111111111111111 invoke [2]",
        "Program 11111111111111111111111111111111 success",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [2]",
        "Program log: Instruction: InitializeMint2",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 2828 of 383962 compute units",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success",
        "Program ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL invoke [2]",
        "Program log: Create",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [3]",
        "Program log: Instruction: GetAccountDataSize",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 1622 of 369163 compute units",
        "Program return: TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA pQAAAAAAAAA=",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success",
        "Program 11111111111111111111111111111111 invoke [3]",
        "Program 11111111111111111111111111111111 success",
        "Program log: Initialize the associated token account",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [3]",
        "Program log: Instruction: InitializeImmutableOwner",
        "Program log: Please upgrade to SPL Token 2022 for immutable owner support",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 1405 of 362523 compute units",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [3]",
        "Program log: Instruction: InitializeAccount3",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 4241 of 358641 compute units",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success",
        "Program ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL consumed 20443 of 374560 compute units",
        "Program ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL success",
        "Program 11111111111111111111111111111111 invoke [2]",
        "Program 11111111111111111111111111111111 success",
        "Program log: Minting and verifying NFT in collection with unique seeds",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [2]",
        "Program log: Instruction: MintTo",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 4538 of 319626 compute units",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success",
        "Program log: Run create metadata accounts v3",
        "Program metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s invoke [2]",
        "Program log: IX: Create Metadata Accounts v3",
        "Program 11111111111111111111111111111111 invoke [3]",
        "Program 11111111111111111111111111111111 success",
        "Program log: Allocate space for the account",
        "Program 11111111111111111111111111111111 invoke [3]",
        "Program 11111111111111111111111111111111 success",
        "Program log: Assign the account to the owning program",
        "Program 11111111111111111111111111111111 invoke [3]",
        "Program 11111111111111111111111111111111 success",
        "Program metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s consumed 45192 of 310300 compute units",
        "Program metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s success",
        "Program metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s invoke [2]",
        "Program log: V3 Create Master Edition",
        "Program log: Transfer 1030080 lamports to the new account",
        "Program 11111111111111111111111111111111 invoke [3]",
        "Program 11111111111111111111111111111111 success",
        "Program log: Allocate space for the account",
        "Program 11111111111111111111111111111111 invoke [3]",
        "Program 11111111111111111111111111111111 success",
        "Program log: Assign the account to the owning program",
        "Program 11111111111111111111111111111111 invoke [3]",
        "Program 11111111111111111111111111111111 success",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [3]",
        "Program log: Instruction: SetAuthority",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 3090 of 214866 compute units",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [3]",
        "Program log: Instruction: SetAuthority",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 3250 of 208387 compute units",
        "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success",
        "Program metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s consumed 54945 of 259319 compute units",
        "Program metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s success",
        "Program metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s invoke [2]",
        "Program log: IX: Verify Collection",
        "Program metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s consumed 29410 of 200244 compute units",
        "Program metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s success",
        "Program log: NFT minted and verified in collection successfully",
        "Program data: xkym57FJxlzdEpwilwEAAHUugAEf1h4FHZ7n7oOJ4YYw/LyP7Ay0ywablAXsxvTvdTBsJS8L/fm02XZGDc4b+iA4yg4MzASYaOSOl59SicB1MGwlLwv9+bTZdkYNzhv6IDjKDgzMBJho5I6Xn1KJwAcAAABuZXcgTkZUAwAAAE5GVF0AAABodHRwczovL2dhdGV3YXkucGluYXRhLmNsb3VkL2lwZnMvYmFma3JlaWM0Y2w1NTVicDV0aDNucmI2c3hiYXppdHlmeGNrMjdmYmFodXdvYXJwdG4ydzRucTV6bDRIT+/3ElMOUd21mOq8oN18vFJUufgmZbJzcnNbPDoaeBgAYAkBAAAAdTBsJS8L/fm02XZGDc4b+iA4yg4MzASYaOSOl59SicABZA==",
        "Program data: dCziBJCDtfdIT+/3ElMOUd21mOq8oN18vFJUufgmZbJzcnNbPDoaeAQAAAAAAAAAIgM6aAAAAAA=",
        "Program Equiqs1Z5Q4F1gBuciqo6yrvqNERzwp5v9Fskhq2A5WB consumed 232059 of 399700 compute units",
        "Program Equiqs1Z5Q4F1gBuciqo6yrvqNERzwp5v9Fskhq2A5WB success"
      ],
    },
    blockTime: Math.floor(Date.now() / 1000),
  },
];

async function testNftMintWebhook() {
  try {
    console.log("Testing NFT mint webhook simulation...");

    // Test webhook with NFT mint payload
    const webhookResponse = await axios.post(
      "http://localhost:3000/webhook",
      mockNftMintPayload,
      {
        headers: {
          "Content-Type": "application/json",
        },
      }
    );

    console.log("✅ NFT Mint webhook test passed:", webhookResponse.data);
  } catch (error) {
    console.error("❌ NFT Mint test failed:", error.message);
    if (error.response) {
      console.error("Response data:", error.response.data);
    }
  }
}

// Run the test
testNftMintWebhook(); 