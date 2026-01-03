# Development Guide

This guide is for developers working on the hermes-engine crate.

## Initial Setup

This crate uses the Hermes source code as a git submodule. Follow these steps when setting up your development environment:

### 1. Initialize the git submodule

```bash
git submodule update --init --recursive
```

This will checkout the Hermes source code into the `hermes-vendor/` directory.

### 2. Apply patches to the Hermes source

The Hermes source needs patches to optimize the build for embedding:

```bash
./apply-patches.sh
```

This script applies patches that:
- Exclude test suites and examples from the build
- Remove unnecessary Android tooling dependencies
- Optimize the CMake configuration for embedding

### 3. Build the crate

```bash
cargo build
```

## Re-patching after submodule updates

If you update the `hermes-vendor` submodule (e.g., `git submodule update --remote`), you'll need to reapply the patches:

```bash
./apply-patches.sh
```

## Understanding the patches

The patches are stored in the `patches/` directory:

- **01-api-cmake.patch** - Excludes `hermes_sandbox` from the API build
- **02-root-cmake.patch** - Excludes Android intl test directory from the root build

These patches reduce build time and exclude components not needed for embedding Hermes in Rust applications.

## Publishing

When publishing this crate to crates.io:

1. Ensure the `hermes-vendor` directory has patches applied
2. The `hermes-vendor` directory is bundled with the published crate (see `Cargo.toml` `exclude` directives)
3. The build script does not modify the source directory, only writes to `OUT_DIR`

## Troubleshooting

### Build fails with "hermes-vendor not found"

Initialize the submodule:
```bash
git submodule update --init --recursive
```

### Build fails with CMake errors

Make sure you have the required build tools:
- macOS: Xcode Command Line Tools (`xcode-select --install`)
- Linux: `build-essential`, `cmake`, `ninja-build`

### Patches don't apply

If the patches fail to apply, the Hermes source may have changed. You may need to update the patch files to match the new Hermes version.
