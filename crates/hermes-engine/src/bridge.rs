// Rust/C++ bridge for Hermes Engine using cxx

#[cxx::bridge]
pub mod ffi {
    // Shared types that can cross the FFI boundary

    // Opaque C++ types
    unsafe extern "C++" {
        include!("hermes-engine/src/hermes_bridge.h");

        // Hermes Runtime handle
        type HermesRuntime;

        // Create a new Hermes runtime
        fn create_hermes_runtime() -> UniquePtr<HermesRuntime>;

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

        // Get bytecode version
        fn get_bytecode_version() -> u32;

        // Evaluate bytecode
        fn eval_bytecode(runtime: Pin<&mut HermesRuntime>, bytecode: &[u8]) -> Result<()>;

        // Get the underlying JSI runtime pointer
        unsafe fn get_jsi_runtime(runtime: Pin<&mut HermesRuntime>) -> *mut u8;

        // Free a JSI value pointer
        unsafe fn free_jsi_value(value: *mut u8);
    }
}
