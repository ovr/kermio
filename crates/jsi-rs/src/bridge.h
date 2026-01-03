#pragma once

// Factory functions wrapping JSI constructors for CXX bridge.
// CXX doesn't support C++ constructors: https://github.com/dtolnay/cxx/issues/280

#include "jsi/jsi.h"
#include "rust/cxx.h"

namespace jsi_rs {

inline std::unique_ptr<facebook::jsi::Object> create_object(facebook::jsi::Runtime& runtime) {
    return std::make_unique<facebook::jsi::Object>(runtime);
}

inline std::unique_ptr<facebook::jsi::Array> create_array(facebook::jsi::Runtime& runtime, size_t length) {
    return std::make_unique<facebook::jsi::Array>(runtime, length);
}

inline std::unique_ptr<facebook::jsi::String> create_string_from_utf8(facebook::jsi::Runtime& runtime, rust::Str data) {
    std::string str(data.data(), data.size());
    return std::make_unique<facebook::jsi::String>(facebook::jsi::String::createFromUtf8(runtime, str));
}

inline std::unique_ptr<facebook::jsi::PropNameID> create_propnameid_from_utf8(facebook::jsi::Runtime& runtime, rust::Str data) {
    return std::make_unique<facebook::jsi::PropNameID>(
        facebook::jsi::PropNameID::forUtf8(
            runtime,
            reinterpret_cast<const uint8_t*>(data.data()),
            data.size()
        )
    );
}

inline std::unique_ptr<facebook::jsi::BigInt> create_bigint_from_i64(facebook::jsi::Runtime& runtime, int64_t value) {
    return std::make_unique<facebook::jsi::BigInt>(facebook::jsi::BigInt::fromInt64(runtime, value));
}

inline std::unique_ptr<facebook::jsi::BigInt> create_bigint_from_u64(facebook::jsi::Runtime& runtime, uint64_t value) {
    return std::make_unique<facebook::jsi::BigInt>(facebook::jsi::BigInt::fromUint64(runtime, value));
}

} // namespace jsi_rs
