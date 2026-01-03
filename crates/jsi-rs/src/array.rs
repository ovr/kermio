use std::pin::Pin;

use crate::JSRuntime;

/// Wrapper around facebook::jsi::Array providing a safe Rust API
pub struct JSArray {
    inner: cxx::UniquePtr<crate::sys::ffi::JSIArray>,
}

impl JSArray {
    /// Create a new JavaScript array with the specified length
    pub fn new(runtime: &mut JSRuntime, length: usize) -> Self {
        let ptr = unsafe {
            let runtime_ref = runtime.as_mut();
            crate::sys::ffi::create_array(Pin::new_unchecked(runtime_ref), length)
        };
        Self { inner: ptr }
    }

    /// Access the inner UniquePtr for advanced usage
    #[cfg(feature = "sys")]
    pub fn inner(&self) -> &cxx::UniquePtr<crate::sys::ffi::JSIArray> {
        &self.inner
    }
}
