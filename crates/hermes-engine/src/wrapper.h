#ifndef HERMES_ENGINE_WRAPPER_H
#define HERMES_ENGINE_WRAPPER_H

#include "rust/cxx.h"
#include <hermes/hermes.h>
#include <hermes-vendor/API/hermes/CompileJS.h>
#include <jsi/jsi.h>
#include <memory>
#include <string>
#include <stdexcept>

// Create a new Hermes runtime with default configuration
inline std::unique_ptr<facebook::hermes::HermesRuntime> create_hermes_runtime() {
    return facebook::hermes::makeHermesRuntime();
}

// Evaluate JavaScript source code
inline void eval_js(
    facebook::hermes::HermesRuntime& runtime,
    rust::Str source,
    rust::Str source_url,
    uint8_t** result_out) {

    try {
        std::string source_str(source.data(), source.size());
        std::string url_str(source_url.data(), source_url.size());

        auto result = runtime.evaluateJavaScript(
            std::make_unique<facebook::jsi::StringBuffer>(source_str),
            url_str);

        // Return the jsi::Value as a pointer if requested
        if (result_out) {
            // Allocate a new jsi::Value and move the result into it
            *result_out = reinterpret_cast<uint8_t*>(new facebook::jsi::Value(std::move(result)));
        }
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

// Free a JSI value pointer
inline void free_jsi_value(uint8_t* value) {
    if (value) {
        delete reinterpret_cast<facebook::jsi::Value*>(value);
    }
}

#endif // HERMES_ENGINE_WRAPPER_H