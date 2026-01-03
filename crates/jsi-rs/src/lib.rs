// Low-level FFI bindings - hidden from public API
mod sys;

// Public API modules
mod array;
mod value;

// Re-export public types
pub use array::JSArray;
pub use sys::ffi::{PropNameID, Runtime};
pub use value::JSValue;
