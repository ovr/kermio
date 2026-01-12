use crate::JSRuntime;

/// Wrapper around facebook::jsi::PropNameID providing a safe Rust API
pub struct JSPropNameID {
    pub(crate) inner: cxx::UniquePtr<crate::sys::ffi::JSIPropNameID>,
}

impl JSPropNameID {
    /// Create a new PropNameID from UTF-8 data
    pub fn new(runtime: &mut JSRuntime, name: &str) -> Self {
        let ptr = crate::sys::ffi::create_propnameid_from_utf8(runtime.pin_mut(), name);
        Self { inner: ptr }
    }

    pub fn value(&self, runtime: &mut JSRuntime) -> String {
        crate::sys::ffi::propnameid_to_utf8(runtime.pin_mut(), &self.inner)
    }

    /// Access the inner UniquePtr for advanced usage
    #[cfg(feature = "unsafe")]
    pub fn inner(&self) -> &cxx::UniquePtr<crate::sys::ffi::JSIPropNameID> {
        &self.inner
    }
}
