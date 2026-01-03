use crate::JSRuntime;

/// Wrapper around facebook::jsi::String providing a safe Rust API
pub struct JSString {
    inner: cxx::UniquePtr<crate::sys::ffi::JSIString>,
}

impl JSString {
    /// Create a new JavaScript string from UTF-8 data
    pub fn new(runtime: &mut JSRuntime, data: &str) -> Self {
        let ptr = crate::sys::ffi::create_string_from_utf8(runtime.pin_mut(), data);
        Self { inner: ptr }
    }

    /// Access the inner UniquePtr for advanced usage
    #[cfg(feature = "sys")]
    pub fn inner(&self) -> &cxx::UniquePtr<crate::sys::ffi::JSIString> {
        &self.inner
    }
}
