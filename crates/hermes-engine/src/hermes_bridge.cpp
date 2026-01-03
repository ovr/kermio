#include "hermes_bridge.h"
#include <hermes/hermes.h>
#include <hermes-vendor/API/hermes/CompileJS.h>
#include <jsi/jsi.h>
#include <memory>
#include <string>
#include <stdexcept>

std::unique_ptr<HermesRuntime> create_hermes_runtime() {
    auto runtime = facebook::hermes::makeHermesRuntime();
    auto wrapper = std::make_unique<HermesRuntime>();
    wrapper->runtime = std::move(runtime);
    return wrapper;
}

void eval_js(
    HermesRuntime& runtime,
    rust::Str source,
    rust::Str source_url,
    uint8_t** result_out) {

    if (!runtime.runtime) {
        throw std::runtime_error("Invalid runtime handle");
    }

    try {
        std::string source_str(source.data(), source.size());
        std::string url_str(source_url.data(), source_url.size());

        auto result = runtime.runtime->evaluateJavaScript(
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

rust::Vec<uint8_t> compile_js_to_bytecode(
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

bool is_hermes_bytecode(rust::Slice<const uint8_t> data) {
    // Get the Hermes root API
    auto hermesAPI = facebook::hermes::makeHermesRuntime();

    // Check if it's Hermes bytecode using the JSI interface
    // For now, return a simple check - this can be improved
    if (data.size() < 8) {
        return false;
    }

    // Hermes bytecode starts with a magic number
    // This is a simplified check
    const uint8_t* bytes = data.data();
    // Check for HBC magic: 0xC61FC6D0 (little endian: 0xD0 0xC6 0x1F 0xC6)
    return bytes[0] == 0xC6 && bytes[1] == 0x1F &&
           bytes[2] == 0xC6 && bytes[3] == 0xD0;
}

uint32_t get_bytecode_version() {
    // Hermes bytecode version
    // This should be obtained from Hermes API, but for now return a known version
    return 96; // Hermes 0.12.0+ uses version 96
}

void eval_bytecode(
    HermesRuntime& runtime,
    rust::Slice<const uint8_t> bytecode) {

    if (!runtime.runtime) {
        throw std::runtime_error("Invalid runtime handle");
    }

    if (bytecode.empty()) {
        throw std::runtime_error("Invalid bytecode buffer");
    }

    try {
        // Convert bytecode to string buffer
        std::string bytecode_str(
            reinterpret_cast<const char*>(bytecode.data()),
            bytecode.size());

        runtime.runtime->evaluateJavaScript(
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

uint8_t* get_jsi_runtime(HermesRuntime& runtime) {
    if (!runtime.runtime) {
        return nullptr;
    }
    return reinterpret_cast<uint8_t*>(runtime.runtime.get());
}

void free_jsi_value(uint8_t* value) {
    if (value) {
        delete reinterpret_cast<facebook::jsi::Value*>(value);
    }
}
