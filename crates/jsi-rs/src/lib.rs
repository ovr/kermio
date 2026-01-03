// Low-level FFI bindings - conditionally exposed via 'sys' feature
#[cfg(not(feature = "sys"))]
mod sys;
#[cfg(feature = "sys")]
pub mod sys;

// Public API modules
mod array;
mod value;

// Re-export public types
pub use array::JSArray;
pub use sys::ffi::{PropNameID, Runtime};
pub use value::JSValue;
