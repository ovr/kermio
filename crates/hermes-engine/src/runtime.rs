use cxx::{SharedPtr, UniquePtr};

use crate::bridge::ffi;
use crate::config::RuntimeConfig;
use crate::jsi::JSValue;

/// Prepared JavaScript code optimized for repeated execution.
///
/// Created via `Runtime::prepare_javascript()`, this represents JavaScript code
/// that has been parsed and optimized by the runtime. The prepared code can be
/// executed multiple times efficiently using `Runtime::evaluate_prepared_javascript()`.
///
/// The prepared form is runtime-specific and optimized for the runtime configuration
/// used when it was created. It can be shared across multiple runtime instances of
/// the same type for memory efficiency.
pub struct PreparedJavaScript {
    handle: SharedPtr<ffi::PreparedJavaScript>,
}

/// A Hermes JavaScript runtime instance.
pub struct Runtime {
    handle: UniquePtr<ffi::HermesRuntime>,
}

impl Runtime {
    /// Create a new Hermes runtime with the specified configuration.
    pub fn new(config: RuntimeConfig) -> Result<Self, String> {
        let handle = ffi::create_hermes_runtime(config.as_ref());
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

        let value_ptr =
            ffi::eval_js(self.handle.pin_mut(), source, url).map_err(|e| e.to_string())?;

        // SAFETY: Since JSIValue and jsi_rs::sys::ffi::JSIValue are the same type (both facebook::jsi::Value),
        // we can safely transmute the UniquePtr
        let js_value = unsafe {
            let raw_ptr = cxx::UniquePtr::into_raw(value_ptr);
            JSValue::from_raw(raw_ptr as *mut crate::jsi::sys::ffi::JSIValue)
        };

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
    pub fn jsi(&mut self) -> jsi_rs::JSRuntime {
        unsafe {
            jsi_rs::JSRuntime::from_raw(self.jsi_runtime() as *mut jsi_rs::sys::ffi::JSIRuntime)
        }
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

    /// Prepare JavaScript code for optimized repeated execution.
    ///
    /// This parses and optimizes the JavaScript code, returning a `PreparedJavaScript`
    /// object that can be executed multiple times efficiently. This is useful when you
    /// need to run the same code repeatedly, as the preparation cost is amortized across
    /// all executions.
    ///
    /// # Arguments
    /// * `source` - JavaScript source code to prepare
    /// * `source_url` - Optional URL/name for the source (used in error messages)
    ///
    /// # Returns
    /// * `Ok(PreparedJavaScript)` - Prepared JavaScript ready for execution
    /// * `Err(String)` - Error message if preparation fails
    ///
    /// # Example
    /// ```no_run
    /// # use hermes_engine::{Runtime, RuntimeConfig};
    /// let mut runtime = Runtime::new(RuntimeConfig::default())?;
    /// let prepared = runtime.prepare_javascript("2 + 2", Some("calc.js"))?;
    ///
    /// // Execute the prepared code multiple times
    /// let result1 = runtime.evaluate_prepared_javascript(&prepared)?;
    /// let result2 = runtime.evaluate_prepared_javascript(&prepared)?;
    /// # Ok::<(), String>(())
    /// ```
    pub fn prepare_javascript(
        &mut self,
        source: &str,
        source_url: Option<&str>,
    ) -> Result<PreparedJavaScript, String> {
        let url = source_url.unwrap_or("prepared");
        let handle = ffi::prepare_javascript(self.handle.pin_mut(), source, url)
            .map_err(|e| e.to_string())?;

        Ok(PreparedJavaScript { handle })
    }

    /// Evaluate prepared JavaScript code and return the result.
    ///
    /// Executes JavaScript code that was previously prepared using `prepare_javascript()`.
    /// This is more efficient than calling `eval_with_result()` repeatedly with the same
    /// source code, as the parsing and optimization has already been done.
    ///
    /// # Arguments
    /// * `prepared` - The prepared JavaScript to execute
    ///
    /// # Returns
    /// * `Ok(JSValue)` - The result of executing the prepared code
    /// * `Err(String)` - Error message if execution fails
    ///
    /// # Example
    /// ```no_run
    /// # use hermes_engine::{Runtime, RuntimeConfig};
    /// let mut runtime = Runtime::new(RuntimeConfig::default())?;
    /// let prepared = runtime.prepare_javascript("'hello' + ' world'", None)?;
    ///
    /// let result = runtime.evaluate_prepared_javascript(&prepared)?;
    /// assert!(result.is_string());
    /// # Ok::<(), String>(())
    /// ```
    pub fn evaluate_prepared_javascript(
        &mut self,
        prepared: &PreparedJavaScript,
    ) -> Result<JSValue, String> {
        let value_ptr = ffi::evaluate_prepared_javascript(self.handle.pin_mut(), &prepared.handle)
            .map_err(|e| e.to_string())?;

        // SAFETY: Since JSIValue and jsi_rs::sys::ffi::JSIValue are the same type (both facebook::jsi::Value),
        // we can safely transmute the UniquePtr
        let js_value = unsafe {
            let raw_ptr = cxx::UniquePtr::into_raw(value_ptr);
            JSValue::from_raw(raw_ptr as *mut crate::jsi::sys::ffi::JSIValue)
        };

        Ok(js_value)
    }
}

unsafe impl Send for Runtime {}
