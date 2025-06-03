const axios = require("axios");

// Mock Helius webhook payload for testing
const mockWebhookPayload = [
  {
    transaction: {
      signatures: ["5wHu1qwD1Mz4xRJfVD2c7QkJGT3nJ8K2mF7vL9pY6rE3"],
      message: {
        accountKeys: [
          "Equiqs1Z5Q4F1gBuciqo6yrvqNERzwp5v9Fskhq2A5WB", // Your program ID
          "11111111111111111111111111111111",
          "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
        ],
      },
    },
    meta: {
      logMessages: [
        "Program Equiqs1Z5Q4F1gBuciqo6yrvqNERzwp5v9Fskhq2A5WB invoke [1]",
        "Program log: Instruction: MintNFT",
        "Program log: Creating NFT for owner: SomeOwnerAddress",
        "Program log: NFT created successfully",
        // Let's add a JSON formatted log that might be useful
        "Program log: {\"event_type\":\"nft_minted\",\"mint\":\"SomeMintAddress\",\"owner\":\"SomeOwnerAddress\"}",
        "Program data: eyJuYW1lIjoiTkZUTWludGVkIiwiZGF0YSI6eyJtaW50IjoiU29tZU1pbnRBZGRyZXNzIiwib3duZXIiOiJTb21lT3duZXJBZGRyZXNzIn19", // Base64 encoded sample event
        "Program Equiqs1Z5Q4F1gBuciqo6yrvqNERzwp5v9Fskhq2A5WB consumed 50000 compute units",
        "Program Equiqs1Z5Q4F1gBuciqo6yrvqNERzwp5v9Fskhq2A5WB success",
      ],
    },
    blockTime: Math.floor(Date.now() / 1000),
  },
];

async function testWebhook() {
  try {
    console.log("Testing webhook endpoint...");

    // Test health check first
    const healthResponse = await axios.get(
      "http://localhost:3000/health"
    );
    console.log("✅ Health check passed:", healthResponse.data);

    // Test webhook
    const webhookResponse = await axios.post(
      "http://localhost:3000/webhook",
      mockWebhookPayload,
      {
        headers: {
          "Content-Type": "application/json",
        },
      }
    );

    console.log("✅ Webhook test passed:", webhookResponse.data);
  } catch (error) {
    console.error("❌ Test failed:", error.message);
    if (error.response) {
      console.error("Response data:", error.response.data);
    }
  }
}

// Run the test
testWebhook();
