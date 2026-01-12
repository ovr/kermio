use crate::JSRuntime;

/// Wrapper around facebook::jsi::String providing a safe Rust API
pub struct JSString {
    pub(crate) inner: cxx::UniquePtr<crate::sys::ffi::JSIString>,
}

impl JSString {
    /// Create a new JavaScript string from UTF-8 data
    pub fn new(runtime: &mut JSRuntime<'_>, data: &str) -> Self {
        let ptr = crate::sys::ffi::create_string_from_utf8(runtime.pin_mut(), data);
        Self { inner: ptr }
    }

    pub fn value(&self, runtime: &mut JSRuntime<'_>) -> String {
        crate::sys::ffi::string_to_utf8(runtime.pin_mut(), &self.inner)
    }

    /// Access the inner UniquePtr for advanced usage
    #[cfg(feature = "unsafe")]
    pub fn inner(&self) -> &cxx::UniquePtr<crate::sys::ffi::JSIString> {
        &self.inner
    }
}
