use std::pin::Pin;

use crate::JSRuntime;

/// Wrapper around facebook::jsi::Object providing a safe Rust API
pub struct JSObject {
    inner: cxx::UniquePtr<crate::sys::ffi::JSIObject>,
}

impl JSObject {
    /// Create a new empty JavaScript object
    pub fn new(runtime: &mut JSRuntime) -> Self {
        let ptr = crate::sys::ffi::create_object(runtime.pin_mut());
        Self { inner: ptr }
    }

    /// Access the inner UniquePtr for advanced usage
    #[cfg(feature = "sys")]
    pub fn inner(&self) -> &cxx::UniquePtr<crate::sys::ffi::JSIObject> {
        &self.inner
    }
}
