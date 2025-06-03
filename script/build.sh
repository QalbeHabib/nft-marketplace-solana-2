#!/bin/bash

# Build script that ensures the program keypair is used

# Set paths
PROGRAM_ID="Equiqs1Z5Q4F1gBuciqo6yrvqNERzwp5v9Fskhq2A5WB"
KEYPAIR_PATH="./Equiqs1Z5Q4F1gBuciqo6yrvqNERzwp5v9Fskhq2A5WB.json"
TARGET_DIR="target/deploy"

# Run the prebuild script
./script/prebuild.sh
if [ $? -ne 0 ]; then
  echo "Prebuild script failed. Aborting build."
  exit 1
fi

# Clean everything thoroughly
echo "Performing thorough cleanup..."
anchor clean
cargo clean
rm -rf target

# Ensure the target directory exists
mkdir -p $TARGET_DIR

# Copy the program keypair to the target directory before build
echo "Copying program keypair to: $TARGET_DIR"
cp $KEYPAIR_PATH $TARGET_DIR/nft_marketplace-keypair.json

# Build the program
echo "Building program with keypair: $KEYPAIR_PATH"
cargo build
anchor build

# Verify the program ID in the built binary
BINARY_PROGRAM_ID=$(solana-keygen pubkey $KEYPAIR_PATH)
if [ "$BINARY_PROGRAM_ID" != "$PROGRAM_ID" ]; then
  echo "Warning: Program ID mismatch in built binary"
  echo "Expected: $PROGRAM_ID"
  echo "Got: $BINARY_PROGRAM_ID"
  exit 1
fi

echo "Build completed successfully with program ID: $PROGRAM_ID" 