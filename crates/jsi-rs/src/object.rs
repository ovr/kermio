use std::ptr::NonNull;

/// JSObject - a wrapper around facebook::jsi::Object
/// This provides a safe Rust API over the raw JSI types
pub struct JSObject {
    // Store a non-null pointer to the FFI type
    // The actual object is stored in C++ and we just hold a reference
    inner: NonNull<crate::sys::ffi::JSIObject>,
}

impl JSObject {
    /// Create a JSObject from a raw pointer
    ///
    /// # Safety
    /// The pointer must be valid and point to a properly initialized JSIObject
    pub(crate) unsafe fn from_raw(ptr: *mut crate::sys::ffi::JSIObject) -> Self {
        Self {
            inner: NonNull::new_unchecked(ptr),
        }
    }

    /// Get a raw pointer to the inner JSIObject
    pub(crate) fn as_ptr(&self) -> *const crate::sys::ffi::JSIObject {
        self.inner.as_ptr()
    }

    /// Get a mutable raw pointer to the inner JSIObject
    pub(crate) fn as_mut_ptr(&mut self) -> *mut crate::sys::ffi::JSIObject {
        self.inner.as_ptr()
    }

    /// Get a reference to the inner JSIObject
    pub(crate) fn as_ref(&self) -> &crate::sys::ffi::JSIObject {
        unsafe { self.inner.as_ref() }
    }

    /// Get a mutable reference to the inner JSIObject
    pub(crate) fn as_mut(&mut self) -> &mut crate::sys::ffi::JSIObject {
        unsafe { self.inner.as_mut() }
    }
}
