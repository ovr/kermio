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

## Patches

This crate applies patches to the Hermes source to optimize for embedding:

- Exclude test suites and examples from the build
- Remove unnecessary Android tooling dependencies
- Optimize the CMake configuration

These patches are already applied in the published crate. See [DEVELOPMENT.md](DEVELOPMENT.md) for details on working with patches during development.
