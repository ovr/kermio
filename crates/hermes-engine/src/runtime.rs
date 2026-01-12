use cxx::{SharedPtr, UniquePtr};

use crate::bridge::ffi;
use crate::config::RuntimeConfig;
use crate::error::Result;
use crate::jsi::{self, JSValue};

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

/// Pre-compiled Hermes bytecode ready for execution.
///
/// Created via `Runtime::compile_to_bytecode()`. Can be executed using
/// `Runtime::eval_bytecode()`. Supports serialization via `as_bytes()`
/// and deserialization via `from_bytes()` for caching compiled bytecode.
pub struct CompiledBytecode {
    handle: UniquePtr<ffi::CompiledBytecode>,
}

impl CompiledBytecode {
    pub fn as_bytes(&self) -> &[u8] {
        let ptr = ffi::compiled_bytecode_data(&self.handle);
        let len = ffi::compiled_bytecode_size(&self.handle);
        unsafe { std::slice::from_raw_parts(ptr, len) }
    }

    pub fn from_bytes(data: &[u8]) -> Self {
        Self {
            handle: ffi::create_compiled_bytecode(data),
        }
    }

    pub fn len(&self) -> usize {
        ffi::compiled_bytecode_size(&self.handle)
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// A Hermes JavaScript runtime instance.
pub struct Runtime {
    handle: UniquePtr<ffi::HermesRuntime>,
}

impl Runtime {
    /// Create a new Hermes runtime with the specified configuration.
    pub fn new(config: RuntimeConfig) -> Result<Self> {
        let handle = ffi::create_hermes_runtime(config.as_ref());
        Ok(Self { handle })
    }

    /// Evaluate JavaScript code.
    pub fn eval(&mut self, source: &str, source_url: Option<&str>) -> Result<()> {
        self.eval_with_result(source, source_url).map(|_| ())
    }

    /// Evaluate JavaScript code and return the result.
    pub fn eval_with_result(&mut self, source: &str, source_url: Option<&str>) -> Result<JSValue> {
        let url = source_url.unwrap_or("eval");

        let value_ptr = ffi::eval_js(self.handle.pin_mut(), source, url)?;

        // SAFETY: Since JSIValue and jsi_rs::sys::ffi::JSIValue are the same type (both facebook::jsi::Value),
        // we can safely transmute the UniquePtr
        let js_value = unsafe {
            let raw_ptr = cxx::UniquePtr::into_raw(value_ptr);
            JSValue::from_raw(raw_ptr as *mut crate::jsi::sys::ffi::JSIValue)
        };

        Ok(js_value)
    }

    /// Get access to the underlying JSI Runtime
    ///
    /// # Example
    /// ```no_run
    /// # use hermes_engine::{Runtime, RuntimeConfig};
    /// let mut runtime = Runtime::new(RuntimeConfig::default())?;
    /// let mut jsi_runtime = runtime.jsi();
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn jsi(&mut self) -> jsi_rs::JSRuntime<'_> {
        unsafe {
            let mut jsi_ref = ffi::get_jsi_runtime(self.handle.pin_mut());
            let ptr = jsi_ref.as_mut().get_unchecked_mut();
            jsi_rs::JSRuntime::from_raw(ptr as *mut _ as *mut jsi_rs::sys::ffi::JSIRuntime)
        }
    }

    pub fn create_undefined() -> jsi::JSValue {
        jsi::JSRuntime::create_undefined()
    }

    pub fn create_null() -> jsi::JSValue {
        jsi::JSRuntime::create_null()
    }

    pub fn create_bool(value: bool) -> jsi::JSValue {
        jsi::JSRuntime::create_bool(value)
    }

    pub fn create_number(value: f64) -> jsi::JSValue {
        jsi::JSRuntime::create_number(value)
    }

    pub fn create_string(&mut self, data: &str) -> jsi::JSString {
        self.jsi().create_string(data)
    }

    pub fn create_object(&mut self) -> jsi::JSObject {
        self.jsi().create_object()
    }

    pub fn create_array_empty(&mut self) -> jsi::JSArray {
        self.jsi().create_array_empty()
    }

    pub fn create_array(&mut self, length: usize) -> jsi::JSArray {
        self.jsi().create_array(length)
    }

    pub fn create_prop_name_id(&mut self, name: &str) -> jsi::JSPropNameID {
        self.jsi().create_prop_name_id(name)
    }

    pub fn create_bigint<T: jsi::IntoJSIBigInt>(&mut self, value: T) -> jsi::JSBigInt {
        self.jsi().create_bigint(value)
    }

    /// Compile JavaScript source to Hermes bytecode.
    pub fn compile_to_bytecode(source: &str, source_url: Option<&str>) -> Result<CompiledBytecode> {
        let url = source_url.unwrap_or("bundle");
        let handle = ffi::compile_js_to_bytecode(source, url, true)?;
        Ok(CompiledBytecode { handle })
    }

    /// Evaluate pre-compiled Hermes bytecode.
    pub fn eval_bytecode(&mut self, bytecode: &CompiledBytecode) -> Result<()> {
        Ok(ffi::eval_bytecode(self.handle.pin_mut(), &bytecode.handle)?)
    }

    /// Prepare JavaScript code for optimized repeated execution.
    ///
    /// This parses and optimizes the JavaScript code, returning a `PreparedJavaScript`
    /// object that can be executed multiple times efficiently. This is useful when you
    /// need to run the same code repeatedly, as the preparation cost is amortized across
    /// all executions.
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
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn prepare_javascript(
        &mut self,
        source: &str,
        source_url: Option<&str>,
    ) -> Result<PreparedJavaScript> {
        let url = source_url.unwrap_or("prepared");
        let handle = ffi::prepare_javascript(self.handle.pin_mut(), source, url)?;

        Ok(PreparedJavaScript { handle })
    }

    /// Evaluate prepared JavaScript code and return the result.
    ///
    /// Executes JavaScript code that was previously prepared using `prepare_javascript()`.
    /// This is more efficient than calling `eval_with_result()` repeatedly with the same
    /// source code, as the parsing and optimization has already been done.
    ///
    /// # Example
    /// ```no_run
    /// # use hermes_engine::{Runtime, RuntimeConfig};
    /// let mut runtime = Runtime::new(RuntimeConfig::default())?;
    /// let prepared = runtime.prepare_javascript("'hello' + ' world'", None)?;
    ///
    /// let result = runtime.evaluate_prepared_javascript(&prepared)?;
    /// assert!(result.is_string());
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn evaluate_prepared_javascript(
        &mut self,
        prepared: &PreparedJavaScript,
    ) -> Result<JSValue> {
        let value_ptr = ffi::evaluate_prepared_javascript(self.handle.pin_mut(), &prepared.handle)?;

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
