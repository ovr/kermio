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

inline std::unique_ptr<facebook::jsi::Value> create_value_undefined() {
    return std::make_unique<facebook::jsi::Value>();
}

inline std::unique_ptr<facebook::jsi::Value> create_value_null() {
    return std::make_unique<facebook::jsi::Value>(nullptr);
}

inline std::unique_ptr<facebook::jsi::Value> create_value_bool(bool value) {
    return std::make_unique<facebook::jsi::Value>(value);
}

inline std::unique_ptr<facebook::jsi::Value> create_value_number(double value) {
    return std::make_unique<facebook::jsi::Value>(value);
}

inline rust::String string_to_utf8(facebook::jsi::Runtime& runtime, const std::unique_ptr<facebook::jsi::String>& str) {
    return rust::String(str->utf8(runtime));
}

inline std::unique_ptr<facebook::jsi::Object> value_as_object(facebook::jsi::Runtime& runtime, const std::unique_ptr<facebook::jsi::Value>& value) {
    facebook::jsi::Object obj = value->asObject(runtime);
    return std::make_unique<facebook::jsi::Object>(std::move(obj));
}

inline std::unique_ptr<facebook::jsi::Function> object_as_function(facebook::jsi::Runtime& runtime, const std::unique_ptr<facebook::jsi::Object>& obj) {
    facebook::jsi::Function func = obj->asFunction(runtime);
    return std::make_unique<facebook::jsi::Function>(std::move(func));
}

inline std::unique_ptr<facebook::jsi::Value> function_call(
    facebook::jsi::Runtime& runtime,
    const std::unique_ptr<facebook::jsi::Function>& func,
    size_t argc) {
    std::vector<facebook::jsi::Value> args;

    // Disambiguate the call by using a function pointer to the non-template overload
    facebook::jsi::Value (facebook::jsi::Function::*call_ptr)(
        facebook::jsi::Runtime&,
        const facebook::jsi::Value*,
        size_t) const = &facebook::jsi::Function::call;

    return std::make_unique<facebook::jsi::Value>(
        (func.get()->*call_ptr)(runtime, args.data(), argc));
}

inline std::unique_ptr<facebook::jsi::Value> function_call_with_this(
    facebook::jsi::Runtime& runtime,
    const std::unique_ptr<facebook::jsi::Function>& func,
    const facebook::jsi::Object& this_obj,
    size_t argc) {
    std::vector<facebook::jsi::Value> args;

    // Disambiguate the call by using a function pointer to the non-template overload
    facebook::jsi::Value (facebook::jsi::Function::*call_ptr)(
        facebook::jsi::Runtime&,
        const facebook::jsi::Object&,
        const facebook::jsi::Value*,
        size_t) const = &facebook::jsi::Function::callWithThis;

    return std::make_unique<facebook::jsi::Value>(
        (func.get()->*call_ptr)(runtime, this_obj, args.data(), argc));
}

inline std::unique_ptr<facebook::jsi::Value> function_call_as_constructor(
    facebook::jsi::Runtime& runtime,
    const std::unique_ptr<facebook::jsi::Function>& func,
    size_t argc) {
    std::vector<facebook::jsi::Value> args;

    // Disambiguate the call by using a function pointer to the non-template overload
    facebook::jsi::Value (facebook::jsi::Function::*call_ptr)(
        facebook::jsi::Runtime&,
        const facebook::jsi::Value*,
        size_t) const = &facebook::jsi::Function::callAsConstructor;

    return std::make_unique<facebook::jsi::Value>(
        (func.get()->*call_ptr)(runtime, args.data(), argc));
}

} // namespace jsi_rs
