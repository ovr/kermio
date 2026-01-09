use crate::JSRuntime;

/// Wrapper around facebook::jsi::BigInt providing a safe Rust API
pub struct JSBigInt {
    pub(crate) inner: cxx::UniquePtr<crate::sys::ffi::JSIBigInt>,
}

impl JSBigInt {
    /// Create a new BigInt from a signed 64-bit integer
    pub fn from_i64(runtime: &mut JSRuntime, value: i64) -> Self {
        let ptr = crate::sys::ffi::create_bigint_from_i64(runtime.pin_mut(), value);
        Self { inner: ptr }
    }

    /// Create a new BigInt from an unsigned 64-bit integer
    pub fn from_u64(runtime: &mut JSRuntime, value: u64) -> Self {
        let ptr = crate::sys::ffi::create_bigint_from_u64(runtime.pin_mut(), value);
        Self { inner: ptr }
    }

    /// Access the inner UniquePtr for advanced usage
    #[cfg(feature = "unsafe")]
    pub fn inner(&self) -> &cxx::UniquePtr<crate::sys::ffi::JSIBigInt> {
        &self.inner
    }
}
