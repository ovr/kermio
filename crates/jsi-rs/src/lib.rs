// Low-level FFI bindings - conditionally exposed via 'sys' feature
#[cfg(not(feature = "sys"))]
mod sys;
#[cfg(feature = "sys")]
pub mod sys;

// Public API modules
mod array;
mod bigint;
mod object;
mod propnameid;
mod runtime;
mod string;
mod value;

// Re-export public types
pub use array::JSArray;
pub use bigint::JSBigInt;
pub use object::JSObject;
pub use propnameid::JSPropNameID;
pub use runtime::JSRuntime;
pub use string::JSString;
pub use value::JSValue;
