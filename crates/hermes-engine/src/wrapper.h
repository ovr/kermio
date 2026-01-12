#ifndef HERMES_ENGINE_WRAPPER_H
#define HERMES_ENGINE_WRAPPER_H

#include "rust/cxx.h"
#include <hermes/hermes.h>
#include <hermes/CompileJS.h>
#include <hermes/Public/RuntimeConfig.h>
#include <jsi/jsi.h>
#include <memory>
#include <string>
#include <stdexcept>

// Non-owning buffer for zero-copy bytecode evaluation
class BorrowedBuffer : public facebook::jsi::Buffer {
public:
    BorrowedBuffer(const uint8_t* data, size_t size) : data_(data), size_(size) {}
    size_t size() const override { return size_; }
    const uint8_t* data() const override { return data_; }
private:
    const uint8_t* data_;
    size_t size_;
};

// Wrapper for compiled Hermes bytecode
struct CompiledBytecode {
    std::string data;
    
    explicit CompiledBytecode(std::string bytecode) : data(std::move(bytecode)) {}
};

// Create RuntimeConfig with custom settings
inline std::unique_ptr<::hermes::vm::RuntimeConfig> create_runtime_config(
    uint32_t init_heap_size,
    uint32_t max_heap_size,
    bool enable_eval,
    bool enable_jit,
    bool enable_es6_proxy,
    bool enable_es6_block_scoping,
    bool enable_intl,
    bool enable_microtask_queue,
    bool enable_generator,
    bool enable_hermes_internal,
    bool enable_sample_profiling,
    uint32_t native_stack_gap,
    uint32_t max_num_registers) {

    ::hermes::vm::RuntimeConfig::Builder builder;
    ::hermes::vm::GCConfig::Builder gcBuilder;

    if (init_heap_size > 0 || max_heap_size > 0) {
        if (init_heap_size > 0) {
            gcBuilder.withInitHeapSize(init_heap_size);
        }
        if (max_heap_size > 0) {
            gcBuilder.withMaxHeapSize(max_heap_size);
        }
        builder.withGCConfig(gcBuilder.build());
    }

    builder.withEnableEval(enable_eval)
           .withEnableJIT(enable_jit)
           .withES6Proxy(enable_es6_proxy)
           .withES6BlockScoping(enable_es6_block_scoping)
           .withIntl(enable_intl)
           .withMicrotaskQueue(enable_microtask_queue)
           .withEnableGenerator(enable_generator)
           .withEnableHermesInternal(enable_hermes_internal)
           .withEnableSampleProfiling(enable_sample_profiling);

    if (native_stack_gap > 0) {
        builder.withNativeStackGap(native_stack_gap);
    }

    if (max_num_registers > 0) {
        builder.withMaxNumRegisters(max_num_registers);
    }

    return std::make_unique<::hermes::vm::RuntimeConfig>(builder.build());
}

// Create a new Hermes runtime with provided configuration
inline std::unique_ptr<facebook::hermes::HermesRuntime> create_hermes_runtime(
    const ::hermes::vm::RuntimeConfig& config
) {
    return facebook::hermes::makeHermesRuntime(config);
}

// Evaluate JavaScript source code and return the result
inline std::unique_ptr<facebook::jsi::Value> eval_js(
    facebook::hermes::HermesRuntime& runtime,
    rust::Str source,
    rust::Str source_url) {

    try {
        std::string source_str(source.data(), source.size());
        std::string url_str(source_url.data(), source_url.size());

        auto result = runtime.evaluateJavaScript(
            std::make_unique<facebook::jsi::StringBuffer>(source_str),
            url_str);

        // Return the jsi::Value wrapped in a unique_ptr
        return std::make_unique<facebook::jsi::Value>(std::move(result));
    } catch (const facebook::jsi::JSError& e) {
        std::string error_msg = "JSError: " + e.getMessage();
        throw std::runtime_error(error_msg);
    } catch (const std::exception& e) {
        std::string error_msg = "Error: " + std::string(e.what());
        throw std::runtime_error(error_msg);
    }
}

// Compile JavaScript to bytecode - returns CompiledBytecode wrapper
inline std::unique_ptr<CompiledBytecode> compile_js_to_bytecode(
    rust::Str source,
    rust::Str source_url,
    bool optimize) {

    std::string source_str(source.data(), source.size());
    std::string url_str(source_url.data(), source_url.size());
    std::string bytecode;

    bool success = ::hermes::compileJS(
        source_str,
        url_str,
        bytecode,
        optimize);

    if (!success) {
        throw std::runtime_error("Failed to compile JavaScript to bytecode");
    }

    return std::make_unique<CompiledBytecode>(std::move(bytecode));
}

// Get bytecode version
inline uint32_t get_bytecode_version() {
    // Hermes bytecode version
    // This should be obtained from Hermes API, but for now return a known version
    return 96; // Hermes 0.12.0+ uses version 96
}

// Evaluate bytecode - zero copy using BorrowedBuffer
inline void eval_bytecode(
    facebook::hermes::HermesRuntime& runtime,
    const CompiledBytecode& bytecode) {

    if (bytecode.data.empty()) {
        throw std::runtime_error("Invalid bytecode buffer");
    }

    try {
        auto buffer = std::make_shared<BorrowedBuffer>(
            reinterpret_cast<const uint8_t*>(bytecode.data.data()),
            bytecode.data.size());

        runtime.evaluateJavaScript(buffer, "bundle");
    } catch (const facebook::jsi::JSError& e) {
        std::string error_msg = "JSError: " + e.getMessage();
        throw std::runtime_error(error_msg);
    } catch (const std::exception& e) {
        std::string error_msg = "Error: " + std::string(e.what());
        throw std::runtime_error(error_msg);
    }
}

// Get the underlying JSI runtime (upcast HermesRuntime to jsi::Runtime base class)
inline facebook::jsi::Runtime& get_jsi_runtime(facebook::hermes::HermesRuntime& runtime) {
    // Upcast to jsi::Runtime base class
    return static_cast<facebook::jsi::Runtime&>(runtime);
}

// Prepare JavaScript for optimized execution
inline std::shared_ptr<facebook::jsi::PreparedJavaScript> prepare_javascript(
    facebook::hermes::HermesRuntime& runtime,
    rust::Str source,
    rust::Str source_url) {

    try {
        std::string source_str(source.data(), source.size());
        std::string url_str(source_url.data(), source_url.size());

        auto buffer = std::make_shared<facebook::jsi::StringBuffer>(source_str);
        auto prepared = runtime.prepareJavaScript(buffer, url_str);

        // CXX doesn't support const in SharedPtr, so we need to cast it away
        return std::const_pointer_cast<facebook::jsi::PreparedJavaScript>(prepared);
    } catch (const facebook::jsi::JSError& e) {
        std::string error_msg = "JSError: " + e.getMessage();
        throw std::runtime_error(error_msg);
    } catch (const std::exception& e) {
        std::string error_msg = "Error: " + std::string(e.what());
        throw std::runtime_error(error_msg);
    }
}

// Evaluate prepared JavaScript and return the result
inline std::unique_ptr<facebook::jsi::Value> evaluate_prepared_javascript(
    facebook::hermes::HermesRuntime& runtime,
    const std::shared_ptr<facebook::jsi::PreparedJavaScript>& prepared) {

    try {
        // The evaluatePreparedJavaScript expects const PreparedJavaScript, so cast it back
        auto const_prepared = std::const_pointer_cast<const facebook::jsi::PreparedJavaScript>(prepared);
        auto result = runtime.evaluatePreparedJavaScript(const_prepared);

        // Return the jsi::Value wrapped in a unique_ptr
        return std::make_unique<facebook::jsi::Value>(std::move(result));
    } catch (const facebook::jsi::JSError& e) {
        std::string error_msg = "JSError: " + e.getMessage();
        throw std::runtime_error(error_msg);
    } catch (const std::exception& e) {
        std::string error_msg = "Error: " + std::string(e.what());
        throw std::runtime_error(error_msg);
    }
}

// Create CompiledBytecode from raw bytes (for loading from disk)
inline std::unique_ptr<CompiledBytecode> create_compiled_bytecode(
    rust::Slice<const uint8_t> data) {

    std::string bytecode(reinterpret_cast<const char*>(data.data()), data.size());
    return std::make_unique<CompiledBytecode>(std::move(bytecode));
}

// Get pointer to bytecode data - zero copy
inline const uint8_t* compiled_bytecode_data(const CompiledBytecode& bytecode) {
    return reinterpret_cast<const uint8_t*>(bytecode.data.data());
}

// Get bytecode size
inline size_t compiled_bytecode_size(const CompiledBytecode& bytecode) {
    return bytecode.data.size();
}

#endif // HERMES_ENGINE_WRAPPER_H