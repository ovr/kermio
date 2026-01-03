#pragma once

// Factory functions wrapping JSI constructors for CXX bridge.
// CXX doesn't support C++ constructors: https://github.com/dtolnay/cxx/issues/280

#include "jsi/jsi.h"

namespace jsi_rs {

inline std::unique_ptr<facebook::jsi::Object> create_object(facebook::jsi::Runtime& runtime) {
    return std::make_unique<facebook::jsi::Object>(runtime);
}

inline std::unique_ptr<facebook::jsi::Array> create_array(facebook::jsi::Runtime& runtime, size_t length) {
    return std::make_unique<facebook::jsi::Array>(runtime, length);
}

} // namespace jsi_rs
