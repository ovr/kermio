// Rust/C++ bridge for Hermes Engine using cxx

#[cxx::bridge]
pub mod ffi {
    // Opaque C++ types
    unsafe extern "C++" {
        include!("hermes-engine/src/wrapper.h");

        // Runtime configuration - maps to hermes::vm::RuntimeConfig
        #[namespace = "hermes::vm"]
        type RuntimeConfig;

        // Hermes Runtime handle - maps to facebook::hermes::HermesRuntime
        #[namespace = "facebook::hermes"]
        type HermesRuntime;

        // PreparedJavaScript - maps to facebook::jsi::PreparedJavaScript
        #[namespace = "facebook::jsi"]
        type PreparedJavaScript;

        // JSI Value - maps to facebook::jsi::Value
        // We use the same type name as jsi-rs for consistency
        #[namespace = "facebook::jsi"]
        #[cxx_name = "Value"]
        type JSIValue;

        // JSI Runtime - maps to facebook::jsi::Runtime
        #[namespace = "facebook::jsi"]
        #[cxx_name = "Runtime"]
        type JSIRuntime;

        // Create RuntimeConfig with all settings
        fn create_runtime_config(
            init_heap_size: u32,
            max_heap_size: u32,
            enable_eval: bool,
            enable_jit: bool,
            enable_es6_proxy: bool,
            enable_es6_block_scoping: bool,
            enable_intl: bool,
            enable_microtask_queue: bool,
            enable_generator: bool,
            enable_hermes_internal: bool,
            enable_sample_profiling: bool,
            native_stack_gap: u32,
            max_num_registers: u32,
        ) -> UniquePtr<RuntimeConfig>;

        // Create a new Hermes runtime
        fn create_hermes_runtime(config: &RuntimeConfig) -> UniquePtr<HermesRuntime>;

        // Evaluate JavaScript source code and return the result
        fn eval_js(
            runtime: Pin<&mut HermesRuntime>,
            source: &str,
            source_url: &str,
        ) -> Result<UniquePtr<JSIValue>>;

        // Compile JavaScript to bytecode
        fn compile_js_to_bytecode(
            source: &str,
            source_url: &str,
            optimize: bool,
        ) -> Result<Vec<u8>>;

        // Evaluate bytecode
        fn eval_bytecode(runtime: Pin<&mut HermesRuntime>, bytecode: &[u8]) -> Result<()>;

        // Get the underlying JSI runtime (upcast HermesRuntime to jsi::Runtime base class)
        fn get_jsi_runtime(runtime: Pin<&mut HermesRuntime>) -> Pin<&mut JSIRuntime>;

        // Prepare JavaScript for optimized execution
        fn prepare_javascript(
            runtime: Pin<&mut HermesRuntime>,
            source: &str,
            source_url: &str,
        ) -> Result<SharedPtr<PreparedJavaScript>>;

        // Evaluate prepared JavaScript
        fn evaluate_prepared_javascript(
            runtime: Pin<&mut HermesRuntime>,
            prepared: &SharedPtr<PreparedJavaScript>,
        ) -> Result<UniquePtr<JSIValue>>;
    }
}
