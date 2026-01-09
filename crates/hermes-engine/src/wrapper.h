#ifndef HERMES_ENGINE_WRAPPER_H
#define HERMES_ENGINE_WRAPPER_H

#include "rust/cxx.h"
#include <hermes/hermes.h>
#include <hermes-vendor/API/hermes/CompileJS.h>
#include <hermes/Public/RuntimeConfig.h>
#include <jsi/jsi.h>
#include <memory>
#include <string>
#include <stdexcept>

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

// Compile JavaScript to bytecode
inline rust::Vec<uint8_t> compile_js_to_bytecode(
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

    // Convert std::string to rust::Vec<uint8_t>
    rust::Vec<uint8_t> result;
    for (char c : bytecode) {
        result.push_back(static_cast<uint8_t>(c));
    }
    return result;
}

// Check if data is Hermes bytecode
inline bool is_hermes_bytecode(rust::Slice<const uint8_t> data) {
    if (data.size() < 8) {
        return false;
    }

    // Hermes bytecode starts with a magic number
    const uint8_t* bytes = data.data();
    // Check for HBC magic: 0xC61FC6D0 (little endian: 0xD0 0xC6 0x1F 0xC6)
    return bytes[0] == 0xC6 && bytes[1] == 0x1F &&
           bytes[2] == 0xC6 && bytes[3] == 0xD0;
}

// Get bytecode version
inline uint32_t get_bytecode_version() {
    // Hermes bytecode version
    // This should be obtained from Hermes API, but for now return a known version
    return 96; // Hermes 0.12.0+ uses version 96
}

// Evaluate bytecode
inline void eval_bytecode(
    facebook::hermes::HermesRuntime& runtime,
    rust::Slice<const uint8_t> bytecode) {

    if (bytecode.empty()) {
        throw std::runtime_error("Invalid bytecode buffer");
    }

    try {
        // Convert bytecode to string buffer
        std::string bytecode_str(
            reinterpret_cast<const char*>(bytecode.data()),
            bytecode.size());

        runtime.evaluateJavaScript(
            std::make_unique<facebook::jsi::StringBuffer>(bytecode_str),
            "bundle");
    } catch (const facebook::jsi::JSError& e) {
        std::string error_msg = "JSError: " + e.getMessage();
        throw std::runtime_error(error_msg);
    } catch (const std::exception& e) {
        std::string error_msg = "Error: " + std::string(e.what());
        throw std::runtime_error(error_msg);
    }
}

// Get the underlying JSI runtime pointer
inline uint8_t* get_jsi_runtime(facebook::hermes::HermesRuntime& runtime) {
    return reinterpret_cast<uint8_t*>(&runtime);
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

#endif // HERMES_ENGINE_WRAPPER_H