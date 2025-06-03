# NFT Marketplace Webhook Server

This webhook server captures events from your Solana NFT Marketplace program and stores them in Supabase PostgreSQL database.

## Features

- 🎯 Filters transactions to only process your program's events
- 📊 Stores events in structured format in Supabase
- 🛡️ Robust error handling and logging
- ⚡ Efficient event decoding using Anchor's BorshCoder
- 🔍 Health check endpoint for monitoring

## Events Captured

Your program emits the following events that will be captured:

- `nftMinted` - When an NFT is minted
- `nftListed` - When an NFT is listed for sale
- `nftPurchased` - When an NFT is purchased
- `collectionMinted` - When a collection is created
- `collectionNftMinted` - When an NFT is minted to a collection
- `offerMade` - When someone makes an offer
- `offerAccepted` - When an offer is accepted
- `offerCanceled` - When an offer is canceled
- `listingCanceled` - When a listing is canceled
- `listingPriceUpdated` - When listing price is updated
- And more...

## Setup Instructions

### 1. Database Setup

First, create the events table in your Supabase database:

```sql
-- Run this in your Supabase SQL editor
-- (Content of supabase-schema.sql)
```

### 2. Install Dependencies

```bash
cd tests
npm install
```

### 3. Environment Setup

Create a `.env` file (optional):

```env
PORT=3000
SUPABASE_URL=https://gfctlllkdwmdjbneddwy.supabase.co
SUPABASE_ANON_KEY=your_anon_key_here
```

### 4. Start the Server

```bash
# Development mode with auto-restart
npm run dev

# Production mode
npm start
```

### 5. Configure Helius Webhook

1. Go to your [Helius Dashboard](https://dashboard.helius.xyz/)
2. Navigate to "Webhooks"
3. Create a new webhook with these settings:
   - **Webhook URL**: `https://your-domain.com/webhook` (or use ngrok for testing)
   - **Webhook Type**: "Enhanced"
   - **Account Addresses**: Add your program ID: `Equiqs1Z5Q4F1gBuciqo6yrvqNERzwp5v9Fskhq2A5WB`
   - **Transaction Types**: Select "Any" or specific types you want to monitor

### 6. Test the Setup

```bash
# In one terminal, start the server
npm start

# In another terminal, run the test
npm test
```

## Deployment Options

### Option 1: Railway (Recommended)

1. Install Railway CLI: `npm install -g @railway/cli`
2. Login: `railway login`
3. Deploy: `railway up`
4. Set environment variables in Railway dashboard

### Option 2: Heroku

1. Install Heroku CLI
2. Login: `heroku login`
3. Create app: `heroku create your-webhook-app`
4. Deploy: `git push heroku main`

### Option 3: DigitalOcean App Platform

1. Connect your GitHub repository
2. Configure build settings
3. Add environment variables
4. Deploy

### For Local Testing (using ngrok)

```bash
# Install ngrok
npm install -g ngrok

# Start your webhook server
npm start

# In another terminal, expose local server
ngrok http 3000

# Use the ngrok URL (e.g., https://abc123.ngrok.io/webhook) in Helius
```

## Monitoring and Debugging

### Check Health Status

```bash
curl http://localhost:3000/health
```

### View Logs

The server provides detailed logging:

- ✅ Successful event processing
- ⚠️ Warnings for non-event logs
- ❌ Errors with full context

### Query Events in Supabase

```sql
-- View recent events
SELECT * FROM events ORDER BY created_at DESC LIMIT 10;

-- Count events by type
SELECT event_type, COUNT(*) FROM events GROUP BY event_type;

-- View events for a specific transaction
SELECT * FROM events WHERE transaction_hash = 'your_signature_here';
```

## Database Schema

The `events` table structure:

```sql
- id: BIGSERIAL PRIMARY KEY
- event_type: VARCHAR(100) -- Event name (e.g., 'nftPurchased')
- transaction_hash: VARCHAR(100) -- Solana transaction signature
- block_time: TIMESTAMP -- When transaction was processed
- data: JSONB -- Event data in JSON format
- program_id: VARCHAR(50) -- Your program ID
- created_at: TIMESTAMP -- When record was inserted
- updated_at: TIMESTAMP -- When record was last updated
```

## Troubleshooting

### Common Issues

1. **Events not being decoded**: Check if your `idl.json` is up to date
2. **Database connection errors**: Verify Supabase credentials
3. **Webhook not receiving data**: Ensure Helius webhook is configured correctly
4. **Duplicate events**: The database has unique constraints to prevent duplicates

### Debug Mode

Enable verbose logging by modifying the webhook code to log raw payloads:

```javascript
// Add this at the start of the webhook handler
console.log("Raw webhook payload:", JSON.stringify(req.body, null, 2));
```

## API Endpoints

- `GET /health` - Health check
- `POST /webhook` - Webhook endpoint for Helius

## Security Considerations

- Use environment variables for sensitive data
- Implement webhook signature verification (if needed)
- Use HTTPS in production
- Consider rate limiting for production deployments

## Support

If you encounter issues:

1. Check the server logs
2. Verify your Supabase table schema
3. Test with the provided test script
4. Ensure your program is emitting events correctly
