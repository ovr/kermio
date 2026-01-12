use crate::JSRuntime;

/// Wrapper around facebook::jsi::Object providing a safe Rust API
pub struct JSObject {
    pub(crate) inner: cxx::UniquePtr<crate::sys::ffi::JSIObject>,
}

impl JSObject {
    /// Create a new empty JavaScript object
    pub fn new(runtime: &mut JSRuntime<'_>) -> Self {
        let ptr = crate::sys::ffi::create_object(runtime.pin_mut());
        Self { inner: ptr }
    }

    /// Access the inner UniquePtr for advanced usage
    #[cfg(feature = "unsafe")]
    pub fn inner(&self) -> &cxx::UniquePtr<crate::sys::ffi::JSIObject> {
        &self.inner
    }
}
