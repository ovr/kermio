#pragma once

// Bridge functions for creating JSI objects from Rust.
//
// This file contains inline C++ wrapper functions to work around CXX limitations.
// CXX doesn't support calling C++ constructors directly from Rust, so we provide
// factory functions that wrap constructor calls.
//
// Related issue: https://github.com/dtolnay/cxx/issues/280

#include "jsi/jsi.h"

namespace jsi_rs {

// Create a new empty Object (like {} in JS)
// Returns a std::unique_ptr for automatic memory management
inline std::unique_ptr<facebook::jsi::Object> create_object(facebook::jsi::Runtime& runtime) {
    return std::make_unique<facebook::jsi::Object>(runtime);
}

} // namespace jsi_rs
