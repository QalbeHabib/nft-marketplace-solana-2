#!/bin/bash

# Full build script with thorough cleaning and keypair setup

# Set paths
PROGRAM_ID="Equiqs1Z5Q4F1gBuciqo6yrvqNERzwp5v9Fskhq2A5WB"
KEYPAIR_PATH="./Equiqs1Z5Q4F1gBuciqo6yrvqNERzwp5v9Fskhq2A5WB.json"
TARGET_DIR="target/deploy"

echo "Starting full build process with program ID: $PROGRAM_ID"
echo "Using keypair: $KEYPAIR_PATH"

# Complete cleaning
echo "Step 1: Performing thorough cleanup..."
anchor clean && cargo clean && rm -rf target

# Create target directory
echo "Step 2: Creating target directory..."
mkdir -p $TARGET_DIR

# Copy keypair to target location
echo "Step 3: Copying program keypair to $TARGET_DIR..."
cp $KEYPAIR_PATH $TARGET_DIR/nft_marketplace-keypair.json

# Build using cargo first, then anchor
echo "Step 4: Building with cargo and anchor..."
cargo build && anchor build

# Generate IDL and TypeScript definitions
echo "Step 5: Generating IDL and TypeScript types..."
mkdir -p target/idl
anchor idl parse -f programs/nft-marketplace-program/src/lib.rs -o target/idl/nft_program.json
anchor idl type target/idl/nft_program.json -o target/types/nft_program.ts

echo "Build process completed."
echo "Program ID: $PROGRAM_ID" 