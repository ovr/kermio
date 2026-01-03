# jsi-rs Implementation Notes

## Overview
This crate provides Rust bindings to Facebook's JSI (JavaScript Interface) using cxx for C++ interop.

## Architecture

### C++ Interop via CXX
- We use cxx::bridge to directly expose JSI C++ classes and methods
- No custom C++ wrapper code is needed - we bind directly to the JSI headers from hermes-vendor
- The JSI headers are already available in `../hermes-engine/hermes-vendor/API/jsi/`

### Key Design Decisions

1. **Direct JSI Bindings**: We expose JSI types directly through cxx rather than creating wrapper functions
2. **Type Naming**: C++ JSI types are prefixed with `JSI` in Rust (e.g., `Value` → `JSIValue`) to avoid naming conflicts
3. **Memory Management**: Using cxx's UniquePtr and SharedPtr for automatic memory management

### Files Structure

- `src/sys.rs`: Low-level FFI bindings using cxx::bridge
- `src/runtime.rs`: Safe Rust wrapper around Runtime
- `src/value.rs`: Safe Rust wrapper for JSValue
- `src/array.rs`: Safe Rust wrapper for JSArray
- `src/lib.rs`: Public API exports

### Building

The `build.rs` file:
- Uses cxx_build to generate the C++ binding code
- Includes JSI headers from hermes-vendor
- Requires C++17 support

## Current Implementation Status

### Exposed Types
- `JSIRuntime` - The main JavaScript execution context
- `JSIValue` - JavaScript value wrapper with type checking methods
- `JSIString` - JavaScript string type
- `JSIObject` - JavaScript object type
- `JSIArray` - JavaScript array type
- `JSIFunction` - JavaScript function type
- `JSIPropNameID` - Property name identifier

All JSI types are prefixed with `JSI` to maintain consistency and avoid naming conflicts.

### Runtime Access
The Runtime is exposed as an opaque type with a safe Rust wrapper. It can be accessed from hermes-engine using the `jsi()` method (requires `unsafe` feature):

```rust
let mut runtime = hermes_engine::Runtime::new()?;
let jsi_runtime = runtime.jsi(); // Returns jsi_rs::Runtime
```

## Implementation Challenges

### CXX Limitations
Since JSI uses complex C++ templates and inheritance, not all methods can be directly exposed through cxx. We currently:
1. Expose core types as opaque C++ types
2. Provide basic type checking methods on JSIValue
3. Use raw pointers for Runtime access (wrapped safely in Rust)

### Runtime Methods
Most Runtime methods require complex C++ bindings that cxx cannot directly handle. Future implementations may need:
- Custom C++ shim functions for complex methods
- SharedPtr/UniquePtr for proper memory management
- Pin<&mut> for methods requiring mutable access

## Future Work
- Add Runtime method bindings (evaluateJavaScript, global, createObject, etc.)
- Implement HostObject and HostFunction support
- Add proper error handling with Result types
- Create higher-level abstractions for common patterns
- Implement Value conversion methods (getString, getNumber, etc.)