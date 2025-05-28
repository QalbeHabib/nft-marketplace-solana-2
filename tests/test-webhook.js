const axios = require("axios");

// Mock Helius webhook payload for testing
const mockWebhookPayload = [
  {
    transaction: {
      signatures: ["5wHu1qwD1Mz4xRJfVD2c7QkJGT3nJ8K2mF7vL9pY6rE3"],
      message: {
        accountKeys: [
          "48Afa15ypgAHQr7qNm2QqW8WL114Ynwer556CV9chARa", // Your program ID
          "11111111111111111111111111111111",
          "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
        ],
      },
    },
    meta: {
      logMessages: [
        "Program 48Afa15ypgAHQr7qNm2QqW8WL114Ynwer556CV9chARa invoke [1]",
        "Program log: This is a test log message",
        "Program data: dGVzdCBldmVudCBkYXRh", // Base64 encoded test data
        "Program 48Afa15ypgAHQr7qNm2QqW8WL114Ynwer556CV9chARa consumed 50000 compute units",
        "Program 48Afa15ypgAHQr7qNm2QqW8WL114Ynwer556CV9chARa success",
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
      "https://72a7-115-186-117-195.ngrok-free.app/health"
    );
    console.log("✅ Health check passed:", healthResponse.data);

    // Test webhook
    const webhookResponse = await axios.post(
      "https://72a7-115-186-117-195.ngrok-free.app/webhook",
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
