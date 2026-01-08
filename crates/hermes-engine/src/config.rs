use cxx::UniquePtr;

use crate::bridge::ffi;

/// Configuration for the Hermes JavaScript runtime
pub struct RuntimeConfig {
    handle: UniquePtr<ffi::RuntimeConfig>,
}

impl RuntimeConfig {
    /// Get reference to the underlying C++ RuntimeConfig
    pub(crate) fn as_ref(&self) -> &ffi::RuntimeConfig {
        &self.handle
    }
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        RuntimeConfigBuilder::new().build()
    }
}

/// Builder for RuntimeConfig with fluent API
pub struct RuntimeConfigBuilder {
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
}

impl RuntimeConfigBuilder {
    /// Create a new builder with default configuration
    pub fn new() -> Self {
        Self {
            init_heap_size: 0, // 0 means use default
            max_heap_size: 0,  // 0 means use default
            enable_eval: true,
            enable_jit: false,
            enable_es6_proxy: true,
            enable_es6_block_scoping: false,
            enable_intl: true,
            enable_microtask_queue: false,
            enable_generator: true,
            enable_hermes_internal: true,
            enable_sample_profiling: false,
            native_stack_gap: 0,  // 0 means use default
            max_num_registers: 0, // 0 means use default
        }
    }

    /// Set heap size configuration
    pub fn heap_size(mut self, init_heap_size: u32, max_heap_size: u32) -> Self {
        self.init_heap_size = init_heap_size;
        self.max_heap_size = max_heap_size;
        self
    }

    /// Enable or disable eval()
    pub fn enable_eval(mut self, enable: bool) -> Self {
        self.enable_eval = enable;
        self
    }

    /// Enable or disable JIT
    pub fn enable_jit(mut self, enable: bool) -> Self {
        self.enable_jit = enable;
        self
    }

    /// Enable or disable ES6 Proxy
    pub fn enable_es6_proxy(mut self, enable: bool) -> Self {
        self.enable_es6_proxy = enable;
        self
    }

    /// Enable or disable ES6 block scoping
    pub fn enable_es6_block_scoping(mut self, enable: bool) -> Self {
        self.enable_es6_block_scoping = enable;
        self
    }

    /// Enable or disable Intl APIs
    pub fn enable_intl(mut self, enable: bool) -> Self {
        self.enable_intl = enable;
        self
    }

    /// Enable or disable microtask queue
    pub fn enable_microtask_queue(mut self, enable: bool) -> Self {
        self.enable_microtask_queue = enable;
        self
    }

    /// Enable or disable generators
    pub fn enable_generator(mut self, enable: bool) -> Self {
        self.enable_generator = enable;
        self
    }

    /// Enable or disable HermesInternal
    pub fn enable_hermes_internal(mut self, enable: bool) -> Self {
        self.enable_hermes_internal = enable;
        self
    }

    /// Enable or disable sample profiling
    pub fn enable_sample_profiling(mut self, enable: bool) -> Self {
        self.enable_sample_profiling = enable;
        self
    }

    /// Set native stack gap
    pub fn native_stack_gap(mut self, gap: u32) -> Self {
        self.native_stack_gap = gap;
        self
    }

    /// Set maximum number of registers
    pub fn max_num_registers(mut self, num: u32) -> Self {
        self.max_num_registers = num;
        self
    }

    /// Build the RuntimeConfig
    pub fn build(self) -> RuntimeConfig {
        RuntimeConfig {
            handle: ffi::create_runtime_config(
                self.init_heap_size,
                self.max_heap_size,
                self.enable_eval,
                self.enable_jit,
                self.enable_es6_proxy,
                self.enable_es6_block_scoping,
                self.enable_intl,
                self.enable_microtask_queue,
                self.enable_generator,
                self.enable_hermes_internal,
                self.enable_sample_profiling,
                self.native_stack_gap,
                self.max_num_registers,
            ),
        }
    }
}

impl Default for RuntimeConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}
