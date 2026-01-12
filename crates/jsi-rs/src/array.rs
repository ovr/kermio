use crate::JSRuntime;

/// Wrapper around facebook::jsi::Array providing a safe Rust API
pub struct JSArray {
    pub(crate) inner: cxx::UniquePtr<crate::sys::ffi::JSIArray>,
}

impl JSArray {
    /// Create a new JavaScript array with the specified length
    pub fn new(runtime: &mut JSRuntime, length: usize) -> Self {
        let ptr = crate::sys::ffi::create_array(runtime.pin_mut(), length);
        Self { inner: ptr }
    }

    pub fn get(&self, runtime: &mut JSRuntime, index: usize) -> crate::JSValue {
        if index >= self.len(runtime) {
            return crate::JSValue::undefined();
        }

        let value =
            crate::sys::ffi::array_get_value_at_index(runtime.pin_mut(), &self.inner, index);
        crate::JSValue { inner: value }
    }

    pub fn set(
        &self,
        runtime: &mut JSRuntime,
        index: usize,
        value: &crate::JSValue,
    ) -> Result<(), String> {
        if index >= self.len(runtime) {
            return Err(format!(
                "Index {} out of bounds for array of length {}",
                index,
                self.len(runtime)
            ));
        }

        crate::sys::ffi::array_set_value_at_index(
            runtime.pin_mut(),
            &self.inner,
            index,
            value.inner(),
        );
        Ok(())
    }

    pub fn len(&self, runtime: &mut JSRuntime) -> usize {
        crate::sys::ffi::array_size(runtime.pin_mut(), &self.inner)
    }

    pub fn is_empty(&self, runtime: &mut JSRuntime) -> bool {
        self.len(runtime) == 0
    }

    /// Access the inner UniquePtr for advanced usage
    #[cfg(feature = "unsafe")]
    pub fn inner(&self) -> &cxx::UniquePtr<crate::sys::ffi::JSIArray> {
        &self.inner
    }
}
