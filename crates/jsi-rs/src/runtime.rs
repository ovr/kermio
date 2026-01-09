use std::pin::Pin;

use crate::sys::ffi;

pub trait IntoJSIBigInt {
    fn create_jsi_bigint(self, runtime: &mut JSRuntime) -> crate::JSBigInt;
}

impl IntoJSIBigInt for i64 {
    fn create_jsi_bigint(self, runtime: &mut JSRuntime) -> crate::JSBigInt {
        crate::JSBigInt::from_i64(runtime, self)
    }
}

impl IntoJSIBigInt for u64 {
    fn create_jsi_bigint(self, runtime: &mut JSRuntime) -> crate::JSBigInt {
        crate::JSBigInt::from_u64(runtime, self)
    }
}

/// Wrapper around facebook::jsi::Runtime providing a safe Rust API
pub struct JSRuntime {
    pub(crate) ptr: *mut ffi::JSIRuntime,
}

impl JSRuntime {
    pub unsafe fn from_raw(ptr: *mut ffi::JSIRuntime) -> Self {
        Self { ptr }
    }

    pub(crate) fn pin_mut(&mut self) -> Pin<&mut ffi::JSIRuntime> {
        unsafe { Pin::new_unchecked(&mut *self.ptr) }
    }

    /// Access the inner raw pointer for advanced usage
    #[cfg(feature = "unsafe")]
    pub fn inner(&self) -> *mut ffi::JSIRuntime {
        self.ptr
    }

    pub fn create_undefined() -> crate::JSValue {
        crate::JSValue::undefined()
    }

    pub fn create_null() -> crate::JSValue {
        crate::JSValue::null()
    }

    pub fn create_bool(value: bool) -> crate::JSValue {
        crate::JSValue::bool(value)
    }

    pub fn create_number(value: f64) -> crate::JSValue {
        crate::JSValue::number(value)
    }

    pub fn create_string(&mut self, data: &str) -> crate::JSString {
        crate::JSString::new(self, data)
    }

    pub fn create_object(&mut self) -> crate::JSObject {
        crate::JSObject::new(self)
    }

    pub fn create_array_empty(&mut self) -> crate::JSArray {
        crate::JSArray::new(self, 0)
    }

    pub fn create_array(&mut self, length: usize) -> crate::JSArray {
        crate::JSArray::new(self, length)
    }

    pub fn create_prop_name_id(&mut self, name: &str) -> crate::JSPropNameID {
        crate::JSPropNameID::new(self, name)
    }

    pub fn create_bigint<T: IntoJSIBigInt>(&mut self, value: T) -> crate::JSBigInt {
        value.create_jsi_bigint(self)
    }
}

// JSRuntime is Send but not Sync
// JavaScript runtimes are typically single-threaded
unsafe impl Send for JSRuntime {}

#[cfg(test)]
mod tests {}
