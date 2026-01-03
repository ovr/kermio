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
    char** result_out) {

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

        // Convert result to string if requested
        if (result_out) {
            std::string result_str;
            if (result.isUndefined()) {
                result_str = "undefined";
            } else if (result.isNull()) {
                result_str = "null";
            } else if (result.isString()) {
                result_str = result.asString(runtime).utf8(runtime);
            } else if (result.isNumber()) {
                result_str = std::to_string(result.asNumber());
            } else if (result.isBool()) {
                result_str = result.getBool() ? "true" : "false";
            } else if (result.isObject()) {
                auto obj = result.asObject(runtime);
                if (obj.isFunction(runtime)) {
                    result_str = "[Function]";
                } else if (obj.isArray(runtime)) {
                    // Simple array serialization
                    auto arr = obj.asArray(runtime);
                    size_t len = arr.size(runtime);
                    result_str = "[";
                    for (size_t i = 0; i < len && i < 100; i++) {
                        if (i > 0) result_str += ",";
                        auto elem = arr.getValueAtIndex(runtime, i);
                        if (elem.isString()) {
                            result_str += "\"" + elem.asString(runtime).utf8(runtime) + "\"";
                        } else if (elem.isNumber()) {
                            result_str += std::to_string(elem.asNumber());
                        } else if (elem.isBool()) {
                            result_str += elem.getBool() ? "true" : "false";
                        } else if (elem.isNull()) {
                            result_str += "null";
                        } else if (elem.isUndefined()) {
                            result_str += "undefined";
                        } else {
                            result_str += "...";
                        }
                    }
                    if (len > 100) result_str += ",...";
                    result_str += "]";
                } else {
                    // Try to use JSON.stringify for objects
                    try {
                        auto global = runtime.global();
                        auto json = global.getPropertyAsObject(runtime, "JSON");
                        auto stringify = json.getPropertyAsFunction(runtime, "stringify");
                        auto stringified = stringify.call(runtime, json, result);
                        result_str = stringified.asString(runtime).utf8(runtime);
                    } catch (...) {
                        result_str = "[Object]";
                    }
                }
            } else {
                result_str = "[Unknown]";
            }

            *result_out = (char*)malloc(result_str.size() + 1);
            strcpy(*result_out, result_str.c_str());
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