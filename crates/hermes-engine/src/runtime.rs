use std::ffi::{CStr, CString};
use std::ptr;

use crate::{
    hermes_compile_js, hermes_free_bytecode, hermes_is_hermes_bytecode, hermes_runtime_create,
    hermes_runtime_destroy, hermes_runtime_eval_bytecode, hermes_runtime_eval_js,
    hermes_runtime_get_jsi, HermesRuntimeHandle,
};

/// A Hermes JavaScript runtime instance.
pub struct Runtime {
    handle: *mut HermesRuntimeHandle,
}

impl Runtime {
    /// Create a new Hermes runtime with default configuration.
    pub fn new() -> Result<Self, String> {
        let handle = unsafe { hermes_runtime_create() };
        if handle.is_null() {
            return Err("Failed to create Hermes runtime".to_string());
        }
        Ok(Self { handle })
    }

    /// Evaluate JavaScript code.
    ///
    /// # Arguments
    /// * `source` - The JavaScript source code to evaluate
    /// * `source_url` - Optional URL/name for the source (used in error messages)
    ///
    /// # Returns
    /// * `Ok(())` on success
    /// * `Err(String)` with error message on failure
    pub fn eval(&mut self, source: &str, source_url: Option<&str>) -> Result<(), String> {
        self.eval_with_result(source, source_url).map(|_| ())
    }

    /// Evaluate JavaScript code and return the result.
    ///
    /// # Arguments
    /// * `source` - The JavaScript source code to evaluate
    /// * `source_url` - Optional URL/name for the source (used in error messages)
    ///
    /// # Returns
    /// * `Ok(String)` with the result value as a string
    /// * `Err(String)` with error message on failure
    pub fn eval_with_result(
        &mut self,
        source: &str,
        source_url: Option<&str>,
    ) -> Result<String, String> {
        let source_cstr = CString::new(source).map_err(|e| format!("Invalid source: {}", e))?;
        let url_cstr = source_url.map(|url| CString::new(url).ok()).flatten();

        let url_ptr = url_cstr.as_ref().map(|s| s.as_ptr()).unwrap_or(ptr::null());

        let mut result_ptr: *mut std::os::raw::c_char = ptr::null_mut();
        let error_ptr = unsafe {
            hermes_runtime_eval_js(
                self.handle,
                source_cstr.as_ptr(),
                source.len(),
                url_ptr,
                &mut result_ptr,
            )
        };

        if error_ptr.is_null() {
            // Success - get the result
            let result = if result_ptr.is_null() {
                "undefined".to_string()
            } else {
                unsafe {
                    let result_str = CStr::from_ptr(result_ptr).to_string_lossy().into_owned();
                    libc::free(result_ptr as *mut libc::c_void);
                    result_str
                }
            };
            Ok(result)
        } else {
            let error_msg = unsafe {
                let msg = CStr::from_ptr(error_ptr).to_string_lossy().into_owned();
                libc::free(error_ptr as *mut libc::c_void);
                msg
            };
            Err(error_msg)
        }
    }

    /// Check if the given bytecode is valid Hermes bytecode.
    pub fn is_hermes_bytecode(data: &[u8]) -> bool {
        unsafe { hermes_is_hermes_bytecode(data.as_ptr(), data.len()) }
    }

    /// Get the underlying JSI runtime pointer
    pub fn jsi_runtime(&mut self) -> *mut std::os::raw::c_void {
        unsafe { hermes_runtime_get_jsi(self.handle) }
    }

    /// Compile JavaScript source to Hermes bytecode.
    ///
    /// # Arguments
    /// * `source` - JavaScript source code
    /// * `source_url` - Optional URL/name for error messages
    ///
    /// # Returns
    /// * `Ok(Vec<u8>)` with bytecode on success
    /// * `Err(String)` with error message on failure
    pub fn compile_to_bytecode(source: &str, source_url: Option<&str>) -> Result<Vec<u8>, String> {
        let source_cstr = CString::new(source).map_err(|e| format!("Invalid source: {}", e))?;
        let url_cstr = source_url.map(|url| CString::new(url).ok()).flatten();

        let url_ptr = url_cstr.as_ref().map(|s| s.as_ptr()).unwrap_or(ptr::null());

        let mut bytecode_ptr: *mut u8 = ptr::null_mut();
        let mut bytecode_len: usize = 0;

        let error_ptr = unsafe {
            hermes_compile_js(
                source_cstr.as_ptr(),
                source.len(),
                url_ptr,
                &mut bytecode_ptr,
                &mut bytecode_len,
            )
        };

        if error_ptr.is_null() {
            // Success - copy bytecode into a Vec
            let bytecode = if bytecode_ptr.is_null() || bytecode_len == 0 {
                Vec::new()
            } else {
                unsafe {
                    let slice = std::slice::from_raw_parts(bytecode_ptr, bytecode_len);
                    let result = slice.to_vec();
                    hermes_free_bytecode(bytecode_ptr, bytecode_len);
                    result
                }
            };
            Ok(bytecode)
        } else {
            let error_msg = unsafe {
                let msg = CStr::from_ptr(error_ptr).to_string_lossy().into_owned();
                libc::free(error_ptr as *mut libc::c_void);
                msg
            };
            Err(error_msg)
        }
    }

    /// Evaluate pre-compiled Hermes bytecode.
    ///
    /// # Arguments
    /// * `bytecode` - Hermes bytecode buffer
    ///
    /// # Returns
    /// * `Ok(())` on success
    /// * `Err(String)` with error message on failure
    pub fn eval_bytecode(&mut self, bytecode: &[u8]) -> Result<(), String> {
        let error_ptr =
            unsafe { hermes_runtime_eval_bytecode(self.handle, bytecode.as_ptr(), bytecode.len()) };

        if error_ptr.is_null() {
            Ok(())
        } else {
            let error_msg = unsafe {
                let msg = CStr::from_ptr(error_ptr).to_string_lossy().into_owned();
                libc::free(error_ptr as *mut libc::c_void);
                msg
            };
            Err(error_msg)
        }
    }
}

impl Drop for Runtime {
    fn drop(&mut self) {
        unsafe {
            hermes_runtime_destroy(self.handle);
        }
    }
}

unsafe impl Send for Runtime {}

impl Default for Runtime {
    fn default() -> Self {
        Self::new().expect("Failed to create Hermes runtime")
    }
}
