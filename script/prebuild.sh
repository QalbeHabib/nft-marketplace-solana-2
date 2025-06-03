#!/bin/bash

# Pre-build script to ensure the correct program keypair is used

# Set paths
PROGRAM_ID="Equiqs1Z5Q4F1gBuciqo6yrvqNERzwp5v9Fskhq2A5WB"
KEYPAIR_PATH="./Equiqs1Z5Q4F1gBuciqo6yrvqNERzwp5v9Fskhq2A5WB.json"
TARGET_DIR="target/deploy"
LIB_RS_PATH="programs/nft-marketplace-program/src/lib.rs"

# Create target directory if it doesn't exist
mkdir -p $TARGET_DIR

# Check if keypair exists, if not display error
if [ ! -f "$KEYPAIR_PATH" ]; then
  echo "Error: Program keypair not found at $KEYPAIR_PATH"
  exit 1
fi

# Verify that lib.rs has the correct program ID
if ! grep -q "declare_id!(\"$PROGRAM_ID\")" "$LIB_RS_PATH"; then
  echo "Warning: Program ID in lib.rs does not match expected ID"
  echo "Expected: $PROGRAM_ID"
  echo "Run 'anchor keys sync' to update the program ID in lib.rs"
fi

echo "Pre-build checks completed successfully."
echo "Using program ID: $PROGRAM_ID"
echo "Using keypair: $KEYPAIR_PATH" 