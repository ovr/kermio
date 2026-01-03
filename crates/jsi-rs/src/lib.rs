// Low-level FFI bindings - conditionally exposed via 'sys' feature
#[cfg(not(feature = "sys"))]
mod sys;
#[cfg(feature = "sys")]
pub mod sys;

// Public API modules
mod array;
mod object;
mod runtime;
mod value;

// Re-export public types
pub use array::JSArray;
pub use object::JSObject;
pub use runtime::JSRuntime;
pub use value::JSValue;
