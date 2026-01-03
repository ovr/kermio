use std::ptr::NonNull;

/// JSArray - a wrapper around facebook::jsi::Array
/// This provides a safe Rust API over the raw JSI types
pub struct JSArray {
    // Store a non-null pointer to the FFI type
    // The actual array is stored in C++ and we just hold a reference
    inner: NonNull<crate::sys::ffi::JSIArray>,
}

impl JSArray {
    /// Create a JSArray from a raw pointer
    ///
    /// # Safety
    /// The pointer must be valid and point to a properly initialized JSIArray
    pub(crate) unsafe fn from_raw(ptr: *mut crate::sys::ffi::JSIArray) -> Self {
        Self {
            inner: NonNull::new_unchecked(ptr),
        }
    }

    /// Get a raw pointer to the inner JSIArray
    pub(crate) fn as_ptr(&self) -> *const crate::sys::ffi::JSIArray {
        self.inner.as_ptr()
    }

    /// Get a mutable raw pointer to the inner JSIArray
    pub(crate) fn as_mut_ptr(&mut self) -> *mut crate::sys::ffi::JSIArray {
        self.inner.as_ptr()
    }

    /// Get a reference to the inner JSIArray
    pub(crate) fn as_ref(&self) -> &crate::sys::ffi::JSIArray {
        unsafe { self.inner.as_ref() }
    }

    /// Get a mutable reference to the inner JSIArray
    pub(crate) fn as_mut(&mut self) -> &mut crate::sys::ffi::JSIArray {
        unsafe { self.inner.as_mut() }
    }
}
