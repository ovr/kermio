use crate::{JSRuntime, JSString};

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

    pub fn as_string_opt(
        &self,
        runtime: &mut JSRuntime,
        radix: i32,
    ) -> Result<JSString, cxx::Exception> {
        let inner = crate::sys::ffi::bigint_to_string(runtime.pin_mut(), &self.inner, radix)?;
        Ok(JSString { inner })
    }

    pub fn as_string(&self, runtime: &mut JSRuntime) -> Result<JSString, cxx::Exception> {
        self.as_string_opt(runtime, 10)
    }

    pub fn to_string(&self, runtime: &mut JSRuntime) -> Result<String, cxx::Exception> {
        let js_string = self.as_string(runtime)?;
        Ok(js_string.value(runtime))
    }

    /// Access the inner UniquePtr for advanced usage
    #[cfg(feature = "unsafe")]
    pub fn inner(&self) -> &cxx::UniquePtr<crate::sys::ffi::JSIBigInt> {
        &self.inner
    }
}
