const axios = require('axios');

// Mock Helius webhook payload for testing
const mockWebhookPayload = [
  {
    "transaction": {
      "signatures": ["5wHu1qwD1Mz4xRJfVD2c7QkJGT3nJ8K2mF7vL9pY6rE3"],
      "message": {
        "accountKeys": [
          "QaQX5WUroY6mHE8RPXXiQUnU73YFRVwKGkSaFcFj6yw", // Your program ID
          "11111111111111111111111111111111",
          "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        ]
      }
    },
    "meta": {
      "logMessages": [
        "Program QaQX5WUroY6mHE8RPXXiQUnU73YFRVwKGkSaFcFj6yw invoke [1]",
        "Program log: This is a test log message",
        "Program data: dGVzdCBldmVudCBkYXRh", // Base64 encoded test data
        "Program QaQX5WUroY6mHE8RPXXiQUnU73YFRVwKGkSaFcFj6yw consumed 50000 compute units",
        "Program QaQX5WUroY6mHE8RPXXiQUnU73YFRVwKGkSaFcFj6yw success"
      ]
    },
    "blockTime": Math.floor(Date.now() / 1000)
  }
];

async function testWebhook() {
  try {
    console.log('Testing webhook endpoint...');
    
    // Test health check first
    const healthResponse = await axios.get('https://72a7-115-186-117-195.ngrok-free.app/health');
    console.log('✅ Health check passed:', healthResponse.data);
    
    // Test webhook
    const webhookResponse = await axios.post('https://72a7-115-186-117-195.ngrok-free.app/webhook', mockWebhookPayload, {
      headers: {
        'Content-Type': 'application/json'
      }
    });
    
    console.log('✅ Webhook test passed:', webhookResponse.data);
    
  } catch (error) {
    console.error('❌ Test failed:', error.message);
    if (error.response) {
      console.error('Response data:', error.response.data);
    }
  }
}

// Run the test
testWebhook(); 