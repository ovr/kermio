#ifndef HERMES_ENGINE_BRIDGE_H
#define HERMES_ENGINE_BRIDGE_H

#include "rust/cxx.h"
#include <memory>

// Forward declarations
namespace facebook {
namespace jsi {
class Runtime;
}
namespace hermes {
class HermesRuntime;
}
}

// Wrapper struct for Hermes runtime
struct HermesRuntime {
    std::shared_ptr<facebook::jsi::Runtime> runtime;
};

// Create a new Hermes runtime with default configuration
std::unique_ptr<HermesRuntime> create_hermes_runtime();

// Evaluate JavaScript source code
void eval_js(
    HermesRuntime& runtime,
    rust::Str source,
    rust::Str source_url,
    uint8_t** result_out);

// Compile JavaScript to bytecode
rust::Vec<uint8_t> compile_js_to_bytecode(
    rust::Str source,
    rust::Str source_url,
    bool optimize);

// Check if data is Hermes bytecode
bool is_hermes_bytecode(rust::Slice<const uint8_t> data);

// Get bytecode version
uint32_t get_bytecode_version();

// Evaluate bytecode
void eval_bytecode(
    HermesRuntime& runtime,
    rust::Slice<const uint8_t> bytecode);

// Get the underlying JSI runtime pointer
uint8_t* get_jsi_runtime(HermesRuntime& runtime);

// Free a JSI value pointer
void free_jsi_value(uint8_t* value);

#endif // HERMES_ENGINE_BRIDGE_H
