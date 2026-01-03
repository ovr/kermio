#include "wrapper.h"
#include <hermes/hermes.h>
#include <jsi/jsi.h>
#include <hermes-vendor/API/hermes/CompileJS.h>
#include <cstring>
#include <cstdio>
#include <memory>

using namespace facebook;

struct HermesRuntimeHandle {
    std::shared_ptr<jsi::Runtime> runtime;
};

HermesRuntimeHandle* hermes_runtime_create(void) {
    try {
        auto runtime = facebook::hermes::makeHermesRuntime();
        auto* handle = new HermesRuntimeHandle();
        handle->runtime = std::move(runtime);
        return handle;
    } catch (const std::exception& e) {
        return nullptr;
    }
}

void hermes_runtime_destroy(HermesRuntimeHandle* runtime) {
    if (runtime) {
        delete runtime;
    }
}

char* hermes_runtime_eval_js(
    HermesRuntimeHandle* handle,
    const char* source,
    size_t source_len,
    const char* source_url,
    void** result_out) {

    if (!handle || !handle->runtime) {
        char* error = (char*)malloc(32);
        strcpy(error, "Invalid runtime handle");
        return error;
    }

    try {
        auto& runtime = *handle->runtime;
        std::string source_str(source, source_len);
        std::string url_str(source_url ? source_url : "eval");

        auto result = runtime.evaluateJavaScript(
            std::make_unique<jsi::StringBuffer>(source_str),
            url_str);

        // Return the jsi::Value as a pointer if requested
        if (result_out) {
            // Allocate a new jsi::Value and move the result into it
            *result_out = new jsi::Value(std::move(result));
        }

        // Success - return NULL for error
        return nullptr;
    } catch (const jsi::JSError& e) {
        std::string error_msg = "JSError: " + e.getMessage();
        char* error = (char*)malloc(error_msg.size() + 1);
        strcpy(error, error_msg.c_str());
        return error;
    } catch (const std::exception& e) {
        std::string error_msg = "Error: " + std::string(e.what());
        char* error = (char*)malloc(error_msg.size() + 1);
        strcpy(error, error_msg.c_str());
        return error;
    }
}

bool hermes_is_hermes_bytecode(const uint8_t* data, size_t len) {
    // TODO: Implement bytecode checking
    // This requires linking against additional Hermes libraries
    return false;
}

uint32_t hermes_get_bytecode_version(void) {
    // TODO: Implement version retrieval
    // Return a placeholder version for now
    return 89;
}

void* hermes_runtime_get_jsi(HermesRuntimeHandle* handle) {
    if (!handle || !handle->runtime) {
        return nullptr;
    }
    return handle->runtime.get();
}

char* hermes_compile_js(
    const char* source,
    size_t source_len,
    const char* source_url,
    uint8_t** bytecode_out,
    size_t* bytecode_len_out) {

    if (!source || !bytecode_out || !bytecode_len_out) {
        char* error = (char*)malloc(40);
        strcpy(error, "Invalid parameters to hermes_compile_js");
        return error;
    }

    try {
        std::string source_str(source, source_len);
        std::string url_str(source_url ? source_url : "bundle");
        std::string bytecode;

        bool success = ::hermes::compileJS(
            source_str,
            url_str,
            bytecode,
            true  // optimize
        );

        if (!success) {
            char* error = (char*)malloc(60);
            strcpy(error, "Failed to compile JavaScript to bytecode");
            return error;
        }

        *bytecode_len_out = bytecode.size();
        *bytecode_out = (uint8_t*)malloc(bytecode.size());
        memcpy(*bytecode_out, bytecode.data(), bytecode.size());

        return nullptr;

    } catch (const std::exception& e) {
        std::string error_msg = "Compilation error: " + std::string(e.what());
        char* error = (char*)malloc(error_msg.size() + 1);
        strcpy(error, error_msg.c_str());
        return error;
    }
}

void hermes_free_bytecode(uint8_t* bytecode, size_t len) {
    if (bytecode) {
        free((void*)bytecode);
    }
}

char* hermes_runtime_eval_bytecode(
    HermesRuntimeHandle* handle,
    const uint8_t* bytecode,
    size_t bytecode_len) {

    if (!handle || !handle->runtime) {
        char* error = (char*)malloc(32);
        strcpy(error, "Invalid runtime handle");
        return error;
    }

    if (!bytecode || bytecode_len == 0) {
        char* error = (char*)malloc(30);
        strcpy(error, "Invalid bytecode buffer");
        return error;
    }

    try {
        auto& runtime = *handle->runtime;

        // Use JSI's evaluateJavaScript which can handle bytecode
        // Convert bytecode buffer to a string
        std::string bytecode_str((char*)bytecode, bytecode_len);

        auto result = runtime.evaluateJavaScript(
            std::make_unique<jsi::StringBuffer>(bytecode_str),
            "bundle"
        );

        return nullptr;

    } catch (const jsi::JSError& e) {
        std::string error_msg = "JSError: " + e.getMessage();
        char* error = (char*)malloc(error_msg.size() + 1);
        strcpy(error, error_msg.c_str());
        return error;
    } catch (const std::exception& e) {
        std::string error_msg = "Error: " + std::string(e.what());
        char* error = (char*)malloc(error_msg.size() + 1);
        strcpy(error, error_msg.c_str());
        return error;
    }
}