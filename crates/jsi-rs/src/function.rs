use crate::sys::ffi;
use crate::value::JSValue;
use crate::JSRuntime;

/// Wrapper around facebook::jsi::Function providing a safe Rust API
pub struct JSFunction {
    pub(crate) inner: cxx::UniquePtr<ffi::JSIFunction>,
}

impl JSFunction {
    /// Call the function with the given arguments
    ///
    /// # Arguments
    /// * `runtime` - The JS runtime
    /// * `args` - Array of arguments to pass to the function (currently not supported)
    ///
    /// # Returns
    /// The result of the function call
    pub fn call(&self, runtime: &mut JSRuntime, _args: &[JSValue]) -> Result<JSValue, String> {
        let result = ffi::function_call(runtime.pin_mut(), &self.inner, 0);

        Ok(JSValue { inner: result })
    }

    /// Call the function with explicit 'this' object
    ///
    /// # Arguments
    /// * `runtime` - The JS runtime
    /// * `this_obj` - The 'this' object to use for the call
    /// * `args` - Array of arguments to pass to the function (currently not supported)
    ///
    /// # Returns
    /// The result of the function call
    pub fn call_with_this(
        &self,
        runtime: &mut JSRuntime,
        this_obj: &crate::JSObject,
        _args: &[JSValue],
    ) -> Result<JSValue, String> {
        let result = ffi::function_call_with_this(
            runtime.pin_mut(),
            &self.inner,
            this_obj.inner.as_ref().expect("JSObject inner is null"),
            0,
        );

        Ok(JSValue { inner: result })
    }

    /// Call the function as a constructor (using 'new')
    ///
    /// # Arguments
    /// * `runtime` - The JS runtime
    /// * `args` - Array of arguments to pass to the constructor (currently not supported)
    ///
    /// # Returns
    /// The newly constructed object
    pub fn call_as_constructor(
        &self,
        runtime: &mut JSRuntime,
        _args: &[JSValue],
    ) -> Result<JSValue, String> {
        let result = ffi::function_call_as_constructor(runtime.pin_mut(), &self.inner, 0);

        Ok(JSValue { inner: result })
    }

    /// Access the inner UniquePtr for advanced usage
    #[cfg(feature = "unsafe")]
    pub fn inner(&self) -> &cxx::UniquePtr<ffi::JSIFunction> {
        &self.inner
    }
}
