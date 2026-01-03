use std::ptr::NonNull;

/// JSValue - a wrapper around facebook::jsi::Value
/// This provides a safe Rust API over the raw JSI types
pub struct JSValue {
    // Store a non-null pointer to the FFI type
    // The actual value is stored in C++ and we just hold a reference
    inner: NonNull<crate::sys::ffi::JSIValue>,
}

impl JSValue {
    /// Create a JSValue from a raw pointer
    ///
    /// # Safety
    /// The pointer must be valid and point to a properly initialized JSIValue
    pub(crate) unsafe fn from_raw(ptr: *mut crate::sys::ffi::JSIValue) -> Self {
        Self {
            inner: NonNull::new_unchecked(ptr),
        }
    }

    /// Get a raw pointer to the inner JSIValue
    pub(crate) fn as_ptr(&self) -> *const crate::sys::ffi::JSIValue {
        self.inner.as_ptr()
    }

    /// Get a mutable raw pointer to the inner JSIValue
    pub(crate) fn as_mut_ptr(&mut self) -> *mut crate::sys::ffi::JSIValue {
        self.inner.as_ptr()
    }

    /// Get a reference to the inner JSIValue
    pub(crate) fn as_ref(&self) -> &crate::sys::ffi::JSIValue {
        unsafe { self.inner.as_ref() }
    }

    /// Get a mutable reference to the inner JSIValue
    pub(crate) fn as_mut(&mut self) -> &mut crate::sys::ffi::JSIValue {
        unsafe { self.inner.as_mut() }
    }

    /// Check if the value is undefined
    pub fn is_undefined(&self) -> bool {
        self.as_ref().isUndefined()
    }

    /// Check if the value is null
    pub fn is_null(&self) -> bool {
        self.as_ref().isNull()
    }

    /// Check if the value is a boolean
    pub fn is_bool(&self) -> bool {
        self.as_ref().isBool()
    }

    /// Check if the value is a number
    pub fn is_number(&self) -> bool {
        self.as_ref().isNumber()
    }

    /// Check if the value is a string
    pub fn is_string(&self) -> bool {
        self.as_ref().isString()
    }

    /// Check if the value is an object
    pub fn is_object(&self) -> bool {
        self.as_ref().isObject()
    }
}
