use cxx::UniquePtr;
use std::ptr;

use crate::bridge::ffi;
use crate::jsi::JSValue;

/// A Hermes JavaScript runtime instance.
pub struct Runtime {
    handle: UniquePtr<ffi::HermesRuntime>,
}

impl Runtime {
    /// Create a new Hermes runtime with default configuration.
    pub fn new() -> Result<Self, String> {
        let handle = ffi::create_hermes_runtime();
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
    /// * `Ok(JSValue)` with the result value
    /// * `Err(String)` with error message on failure
    pub fn eval_with_result(
        &mut self,
        source: &str,
        source_url: Option<&str>,
    ) -> Result<JSValue, String> {
        let url = source_url.unwrap_or("eval");
        let mut result_ptr: *mut u8 = ptr::null_mut();

        unsafe {
            ffi::eval_js(
                self.handle.pin_mut(),
                source,
                url,
                &mut result_ptr as *mut *mut u8,
            )
            .map_err(|e| e.to_string())?;
        }

        if result_ptr.is_null() {
            return Err("Evaluation succeeded but no result was returned".to_string());
        }

        let js_value =
            unsafe { JSValue::from_raw(result_ptr as *mut crate::jsi::sys::ffi::JSIValue) };

        Ok(js_value)
    }

    /// Check if the given bytecode is valid Hermes bytecode.
    pub fn is_hermes_bytecode(data: &[u8]) -> bool {
        ffi::is_hermes_bytecode(data)
    }

    /// Get the underlying JSI runtime pointer
    pub fn jsi_runtime(&mut self) -> *mut std::os::raw::c_void {
        unsafe { ffi::get_jsi_runtime(self.handle.pin_mut()) as *mut std::os::raw::c_void }
    }

    /// Get a reference to the underlying JSI Runtime
    ///
    /// This provides access to the low-level JSI API for advanced use cases.
    ///
    /// # Safety
    /// This method is only available when the `unsafe` feature is enabled.
    /// The caller must ensure proper synchronization when using the JSI API directly.
    #[cfg(feature = "unsafe")]
    pub fn jsi(&mut self) -> &mut crate::jsi::Runtime {
        unsafe { &mut *(self.jsi_runtime() as *mut crate::jsi::Runtime) }
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
        let url = source_url.unwrap_or("bundle");
        ffi::compile_js_to_bytecode(source, url, true).map_err(|e| e.to_string())
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
        ffi::eval_bytecode(self.handle.pin_mut(), bytecode).map_err(|e| e.to_string())
    }
}

unsafe impl Send for Runtime {}

impl Default for Runtime {
    fn default() -> Self {
        Self::new().expect("Failed to create Hermes runtime")
    }
}
