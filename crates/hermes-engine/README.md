# hermes-engine

Rust bindings to the Hermes JavaScript engine.

## Development

For development setup, building, and contributing, see [DEVELOPMENT.md](DEVELOPMENT.md).

## Quick Start

This crate bundles the Hermes source code. For most users, simply add the dependency:

```toml
[dependencies]
hermes-engine = "0.0.1"
```

## Features

### `unsafe`

Enables low-level access to the JSI (JavaScript Interface) API for advanced use cases. This feature is opt-in and provides direct access to low-level JSI system types and the underlying JavaScript runtime.

**⚠️ Warning:** The JSI API requires careful handling and understanding of JavaScript engine internals. Use this feature only if you need direct access to low-level JSI functionality.

```toml
[dependencies]
hermes-engine = { version = "0.0.1", features = ["unsafe"] }
```

With this feature enabled, you can work with JSI types directly:

```rust
use hermes_engine::jsi;

// Access JSI types and their methods
let value: &jsi::JSValue = /* ... */;
if value.is_undefined() {
    // ...
}
```

## Patches

This crate applies patches to the Hermes source to optimize for embedding:

- Exclude test suites and examples from the build
- Remove unnecessary Android tooling dependencies
- Optimize the CMake configuration

These patches are already applied in the published crate. See [DEVELOPMENT.md](DEVELOPMENT.md) for details on working with patches during development.
