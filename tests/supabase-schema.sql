-- Create the events table in Supabase
CREATE TABLE IF NOT EXISTS events (
    id BIGSERIAL PRIMARY KEY,
    event_type VARCHAR(100) NOT NULL,
    transaction_hash VARCHAR(100) NOT NULL,
    block_time TIMESTAMP WITH TIME ZONE NOT NULL,
    data JSONB NOT NULL,
    program_id VARCHAR(50) NOT NULL DEFAULT 'Equiqs1Z5Q4F1gBuciqo6yrvqNERzwp5v9Fskhq2A5WB',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Create indexes for better query performance
CREATE INDEX IF NOT EXISTS idx_events_event_type ON events(event_type);
CREATE INDEX IF NOT EXISTS idx_events_transaction_hash ON events(transaction_hash);
CREATE INDEX IF NOT EXISTS idx_events_block_time ON events(block_time);
CREATE INDEX IF NOT EXISTS idx_events_program_id ON events(program_id);
CREATE INDEX IF NOT EXISTS idx_events_created_at ON events(created_at);

-- Create a composite index for common queries
CREATE INDEX IF NOT EXISTS idx_events_type_time ON events(event_type, block_time);

-- Create a unique constraint to prevent duplicate events
CREATE UNIQUE INDEX IF NOT EXISTS idx_events_unique ON events(transaction_hash, event_type, data);

-- Add comments for documentation
COMMENT ON TABLE events IS 'Stores Solana program events from the NFT Marketplace';
COMMENT ON COLUMN events.event_type IS 'Type of event (e.g., nftListed, nftPurchased, etc.)';
COMMENT ON COLUMN events.transaction_hash IS 'Solana transaction signature';
COMMENT ON COLUMN events.block_time IS 'When the transaction was processed on Solana';
COMMENT ON COLUMN events.data IS 'Event data in JSON format';
COMMENT ON COLUMN events.program_id IS 'Solana program ID that emitted the event';

-- Create a function to update the updated_at timestamp
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Create a trigger to automatically update the updated_at column
CREATE TRIGGER update_events_updated_at BEFORE UPDATE ON events
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column(); 