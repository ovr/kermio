use crate::sys::ffi;

/// Represents a JSI Runtime instance
///
/// This is a wrapper around the C++ facebook::jsi::Runtime class.
/// The Runtime provides the main execution context for JavaScript.
pub struct Runtime {
    // We store a raw pointer since we don't own the Runtime
    // It's typically owned by the Hermes engine
    ptr: *mut ffi::JSIRuntime,
}

impl Runtime {
    /// Create a Runtime wrapper from a raw pointer
    ///
    /// # Safety
    /// The caller must ensure:
    /// - The pointer is valid and points to a live Runtime
    /// - The Runtime outlives this wrapper
    /// - No other mutable references exist to the Runtime
    pub unsafe fn from_raw(ptr: *mut ffi::JSIRuntime) -> Self {
        Self { ptr }
    }

    /// Get the raw pointer to the underlying Runtime
    pub fn as_ptr(&self) -> *mut ffi::JSIRuntime {
        self.ptr
    }

    /// Get a reference to the underlying Runtime
    ///
    /// # Safety
    /// The caller must ensure the Runtime is still valid
    pub unsafe fn as_ref(&self) -> &ffi::JSIRuntime {
        &*self.ptr
    }

    /// Get a mutable reference to the underlying Runtime
    ///
    /// # Safety
    /// The caller must ensure:
    /// - The Runtime is still valid
    /// - No other references exist to the Runtime
    pub unsafe fn as_mut(&mut self) -> &mut ffi::JSIRuntime {
        &mut *self.ptr
    }
}

// Runtime is Send but not Sync
// JavaScript runtimes are typically single-threaded
unsafe impl Send for Runtime {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_runtime_creation() {
        // This test just ensures the type compiles
        // Actual testing would require a real JSI runtime
    }
}
