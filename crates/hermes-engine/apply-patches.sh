#!/bin/bash
# Script to apply patches to hermes-vendor source code
# Run this after updating the hermes-vendor submodule

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
HERMES_SRC="$SCRIPT_DIR/hermes-vendor"
PATCHES_DIR="$SCRIPT_DIR/patches"

if [ ! -d "$HERMES_SRC" ]; then
    echo "ERROR: hermes-vendor directory not found at $HERMES_SRC"
    echo "Please initialize the git submodule first:"
    echo "  git submodule update --init --recursive"
    exit 1
fi

if [ ! -d "$PATCHES_DIR" ]; then
    echo "ERROR: patches directory not found at $PATCHES_DIR"
    exit 1
fi

echo "Applying patches to hermes-vendor..."

# Apply all .patch files
for patch_file in "$PATCHES_DIR"/*.patch; do
    if [ -f "$patch_file" ]; then
        patch_name=$(basename "$patch_file")
        echo "Applying $patch_name..."

        patch -p1 -d "$HERMES_SRC" -i "$patch_file" --force --no-backup-if-mismatch
    fi
done

echo "All patches applied successfully!"
