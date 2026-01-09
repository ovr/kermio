# Hermes Engine Crate Guide

This crate provides low-level Rust FFI bindings to Meta's Hermes JavaScript engine.

## About the Crate

The hermes-engine crate includes the complete Hermes source code (hermes-vendor directory) bundled with the published crate. This means:

- **No external dependencies**: Everything needed to build is included
- **No git required**: Works without git installation
- **Offline builds**: Can be built without internet access
- **Large download**: The crate is ~100MB+ due to bundled source

## Building the Crate

The first build compiles the entire Hermes engine from source, which can take several minutes:

```bash
cargo build -p hermes-engine
# For release build with optimizations:
cargo build -p hermes-engine --release
```

Subsequent builds are much faster as Cargo caches the compiled artifacts.

## Hermes Compiler Tools

After building, the Hermes compiler and runtime tools are available in:

```
target/release/build/hermes-rs-<hash>/out/build/bin/
```

Key tools:
- **`hermesc`** - Hermes compiler (compiles JS/TS to bytecode)
- **`hermes`** - Hermes runtime (executes bytecode)
- **`shermes`** - Hermes shell with bytecode compilation
- **`hermes-jsi`** - Hermes with JSI interface

## Compiling to Bytecode

### Compile TypeScript

Hermes supports TypeScript syntax when built with `HERMES_PARSE_TS=ON` (enabled by default):

```bash
# Find the hermesc binary
HERMESC=$(find target/release/build -name hermesc -type f | head -1)

# Compile TypeScript to bytecode
$HERMESC -parse-ts -emit-binary -out output.hbc input.ts
```

### Compile JavaScript

```bash
$HERMESC -emit-binary -out output.hbc input.js
```

### Useful Compiler Flags

- `-parse-ts` - Enable TypeScript parsing
- `-parse-flow` - Enable Flow parsing
- `-emit-binary` - Output binary bytecode (.hbc file)
- `-dump-bytecode` - Dump bytecode as text (for debugging)
- `-O` - Enable expensive optimizations
- `-O0` - No optimizations
- `-output-source-map` - Generate source map

Example with optimizations:
```bash
$HERMESC -parse-ts -emit-binary -O -out optimized.hbc input.ts
```

## Running Bytecode

Use the `hermes` runtime to execute compiled bytecode:

```bash
# Find the hermes binary
HERMES=$(find target/release/build -name hermes -type f -executable | head -1)

# Run bytecode file
$HERMES program.hbc
```

## Build Output Locations

The Hermes build process generates static libraries in:
```
target/release/build/hermes-rs-<hash>/out/build/lib/
```

Available static libraries:
- `libhermesvm_a.a` - Core VM
- `libjsi.a` - JSI interface
- `libhermesPublic.a` - Public API
- Various subsystem libraries (IR, Parser, BCGen, etc.)

## CMake Build Configuration

The crate's `build.rs` configures Hermes with these settings:
- TypeScript parsing: Enabled (`HERMES_PARSE_TS=ON`)
- Flow parsing: Enabled (`HERMES_PARSE_FLOW=ON`)
- Debugger: Disabled
- Intl: Disabled
- Build type: Release (optimized)

## Testing

Run hermes-engine tests:
```bash
cargo test -p hermes-engine
```

Run jsi-rs integration tests (located in this crate):
```bash
cargo test -p hermes-engine --test jsi_rs
```

Note: jsi-rs is a low-level interface layer without an engine implementation, so all jsi-rs tests are located in `tests/jsi-rs/` directory and use hermes-engine's Runtime as the concrete JSI implementation.

## Example Workflow

Complete example of compiling and running a TypeScript file:

```bash
# 1. Build hermes-rs (if not already built)
cargo build -p hermes-rs --release

# 2. Find the tools
HERMESC=$(find target/release/build -name hermesc -type f | head -1)
HERMES=$(find target/release/build -name hermes -type f -executable | head -1)

# 3. Compile TypeScript to bytecode
$HERMESC -parse-ts -emit-binary -out myapp.hbc myapp.ts

# 4. Run the bytecode
$HERMES myapp.hbc
```

## Integration Notes

- The hermes-engine crate links statically with all Hermes libraries
- Hermes source code is bundled with the crate (hermes-vendor directory)
- Build artifacts from hermes-vendor are gitignored but included in published crates
- The crate exports FFI bindings for use in Rust code

## Development

When developing this crate from the git repository, hermes-vendor is a git submodule:

```bash
# Initialize the submodule after cloning
git submodule update --init --recursive
```

When published to crates.io, the hermes-vendor directory is bundled with the crate, so downstream users don't need to manage submodules.

### C++ Bridge Implementation

The C++ bridge code (wrapper.h) is implemented as a header-only file. **Do NOT create separate .cpp files** for the bridge implementation - all functions should be inline in the header file. This simplifies the build process and avoids unnecessary complexity in the cxx bridge setup.
