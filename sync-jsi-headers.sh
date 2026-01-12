#!/bin/bash
# Sync JSI headers from hermes-vendor submodule to jsi-rs crate

set -e

# Initialize submodules if needed
git submodule update --init --recursive

# Source and destination paths
SRC="crates/hermes-engine/hermes-vendor/API/jsi/jsi"
DEST="crates/jsi-rs/include/jsi"

# Create destination directory
mkdir -p "$DEST"

# Copy all header files
cp "$SRC"/*.h "$DEST/"

echo "JSI headers synced to $DEST"
