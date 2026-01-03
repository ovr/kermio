#ifndef HERMES_RS_WRAPPER_H
#define HERMES_RS_WRAPPER_H

#include <stddef.h>
#include <stdint.h>
#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif

// Opaque handle to Hermes runtime
typedef struct HermesRuntimeHandle HermesRuntimeHandle;

// Create a new Hermes runtime with default configuration
HermesRuntimeHandle* hermes_runtime_create(void);

// Destroy a Hermes runtime
void hermes_runtime_destroy(HermesRuntimeHandle* runtime);

// Evaluate JavaScript source code
// Returns NULL on success, or an error message on failure (caller must free)
// If result_out is not NULL, it will be set to a string representation of the result (caller must free)
char* hermes_runtime_eval_js(
    HermesRuntimeHandle* runtime,
    const char* source,
    size_t source_len,
    const char* source_url,
    char** result_out);

// Check if bytecode is valid Hermes bytecode
bool hermes_is_hermes_bytecode(const uint8_t* data, size_t len);

// Get the Hermes bytecode version
uint32_t hermes_get_bytecode_version(void);

// Get the underlying JSI runtime from a Hermes runtime
void* hermes_runtime_get_jsi(HermesRuntimeHandle* runtime);

// Compile JavaScript source to Hermes bytecode
// Returns NULL on success with bytecode_out set to allocated buffer (caller must free)
// Returns error message on failure (caller must free)
char* hermes_compile_js(
    const char* source,
    size_t source_len,
    const char* source_url,
    uint8_t** bytecode_out,
    size_t* bytecode_len_out);

// Free bytecode buffer allocated by hermes_compile_js
void hermes_free_bytecode(uint8_t* bytecode, size_t len);

// Evaluate pre-compiled Hermes bytecode
// Returns NULL on success, or an error message on failure (caller must free)
char* hermes_runtime_eval_bytecode(
    HermesRuntimeHandle* runtime,
    const uint8_t* bytecode,
    size_t bytecode_len);

#ifdef __cplusplus
}
#endif

#endif // HERMES_RS_WRAPPER_H
