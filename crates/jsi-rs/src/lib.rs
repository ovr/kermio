mod error;

// Low-level FFI bindings - conditionally exposed via 'unsafe' feature
#[cfg(not(feature = "unsafe"))]
mod sys;
#[cfg(feature = "unsafe")]
pub mod sys;

// Public API modules
mod array;
mod bigint;
mod function;
mod object;
mod propnameid;
mod runtime;
mod string;
mod value;

// Re-export public types
pub use array::JSArray;
pub use bigint::JSBigInt;
pub use error::{Error, Result};
pub use function::JSFunction;
pub use object::JSObject;
pub use propnameid::JSPropNameID;
pub use runtime::{IntoJSIBigInt, JSRuntime};
pub use string::JSString;
pub use value::JSValue;
