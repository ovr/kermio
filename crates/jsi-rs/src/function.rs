use crate::sys::ffi;
use crate::value::JSValue;
use crate::JSRuntime;

/// Wrapper around facebook::jsi::Function providing a safe Rust API
pub struct JSFunction {
    pub(crate) inner: cxx::UniquePtr<ffi::JSIFunction>,
}

impl JSFunction {
    /// Call the function with the given arguments
    pub fn call(&self, runtime: &mut JSRuntime, args: &[JSValue]) -> Result<JSValue, String> {
        let mut vec = ffi::value_vec_create();
        for arg in args {
            ffi::value_vec_push(vec.pin_mut(), runtime.pin_mut(), arg.inner());
        }

        let result = ffi::function_call(runtime.pin_mut(), &self.inner, &vec);

        Ok(JSValue { inner: result })
    }

    /// Call the function with explicit 'this' object
    pub fn call_with_this(
        &self,
        runtime: &mut JSRuntime,
        this_obj: &crate::JSObject,
        args: &[JSValue],
    ) -> Result<JSValue, String> {
        let mut vec = ffi::value_vec_create();
        for arg in args {
            ffi::value_vec_push(vec.pin_mut(), runtime.pin_mut(), arg.inner());
        }

        let result = ffi::function_call_with_this(
            runtime.pin_mut(),
            &self.inner,
            this_obj.inner.as_ref().expect("JSObject inner is null"),
            &vec,
        );

        Ok(JSValue { inner: result })
    }

    /// Call the function as a constructor (using 'new')
    pub fn call_as_constructor(
        &self,
        runtime: &mut JSRuntime,
        args: &[JSValue],
    ) -> Result<JSValue, String> {
        let mut vec = ffi::value_vec_create();
        for arg in args {
            ffi::value_vec_push(vec.pin_mut(), runtime.pin_mut(), arg.inner());
        }

        let result = ffi::function_call_as_constructor(runtime.pin_mut(), &self.inner, &vec);

        Ok(JSValue { inner: result })
    }

    /// Access the inner UniquePtr for advanced usage
    #[cfg(feature = "unsafe")]
    pub fn inner(&self) -> &cxx::UniquePtr<ffi::JSIFunction> {
        &self.inner
    }
}
