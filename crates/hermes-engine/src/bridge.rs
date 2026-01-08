// Rust/C++ bridge for Hermes Engine using cxx

#[cxx::bridge]
pub mod ffi {
    // Shared types that can cross the FFI boundary

    // Opaque C++ types
    unsafe extern "C++" {
        include!("hermes-engine/src/wrapper.h");

        // Runtime configuration - maps to hermes::vm::RuntimeConfig
        #[namespace = "hermes::vm"]
        type RuntimeConfig;

        // Hermes Runtime handle - maps to facebook::hermes::HermesRuntime
        #[namespace = "facebook::hermes"]
        type HermesRuntime;

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

        // Evaluate JavaScript source code
        // Returns error string on failure, empty string on success
        // result_out will contain a pointer to jsi::Value if provided
        unsafe fn eval_js(
            runtime: Pin<&mut HermesRuntime>,
            source: &str,
            source_url: &str,
            result_out: *mut *mut u8,
        ) -> Result<()>;

        // Compile JavaScript to bytecode
        fn compile_js_to_bytecode(
            source: &str,
            source_url: &str,
            optimize: bool,
        ) -> Result<Vec<u8>>;

        // Check if data is Hermes bytecode
        fn is_hermes_bytecode(data: &[u8]) -> bool;

        // Evaluate bytecode
        fn eval_bytecode(runtime: Pin<&mut HermesRuntime>, bytecode: &[u8]) -> Result<()>;

        // Get the underlying JSI runtime pointer
        unsafe fn get_jsi_runtime(runtime: Pin<&mut HermesRuntime>) -> *mut u8;
    }
}
