use std::ptr::NonNull;

/// Wrapper around facebook::jsi::Value providing a safe Rust API
pub struct JSValue {
    inner: NonNull<crate::sys::ffi::JSIValue>,
}

impl JSValue {
    pub unsafe fn from_raw(ptr: *mut crate::sys::ffi::JSIValue) -> Self {
        Self {
            inner: NonNull::new_unchecked(ptr),
        }
    }

    fn as_ref(&self) -> &crate::sys::ffi::JSIValue {
        unsafe { self.inner.as_ref() }
    }

    /// Access the inner NonNull pointer for advanced usage
    #[cfg(feature = "sys")]
    pub fn inner(&self) -> NonNull<crate::sys::ffi::JSIValue> {
        self.inner
    }

    pub fn is_undefined(&self) -> bool {
        self.as_ref().isUndefined()
    }

    pub fn is_null(&self) -> bool {
        self.as_ref().isNull()
    }

    pub fn is_bool(&self) -> bool {
        self.as_ref().isBool()
    }

    pub fn is_number(&self) -> bool {
        self.as_ref().isNumber()
    }

    pub fn is_string(&self) -> bool {
        self.as_ref().isString()
    }

    pub fn is_object(&self) -> bool {
        self.as_ref().isObject()
    }
}
