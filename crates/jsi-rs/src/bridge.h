#pragma once

// Factory functions wrapping JSI constructors for CXX bridge.
// CXX doesn't support C++ constructors: https://github.com/dtolnay/cxx/issues/280

#include "jsi/jsi.h"
#include "rust/cxx.h"

namespace jsi_rs {

struct ValueVec {
    std::vector<facebook::jsi::Value> values;
};

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

inline bool value_as_bool(const std::unique_ptr<facebook::jsi::Value>& value) {
    return value->getBool();
}

inline double value_as_number(const std::unique_ptr<facebook::jsi::Value>& value) {
    return value->getNumber();
}

inline std::unique_ptr<facebook::jsi::String> value_as_string(facebook::jsi::Runtime& runtime, const std::unique_ptr<facebook::jsi::Value>& value) {
    facebook::jsi::String str = value->asString(runtime);
    return std::make_unique<facebook::jsi::String>(std::move(str));
}

inline std::unique_ptr<facebook::jsi::BigInt> value_as_bigint(facebook::jsi::Runtime& runtime, const std::unique_ptr<facebook::jsi::Value>& value) {
    facebook::jsi::BigInt bigint = value->asBigInt(runtime);
    return std::make_unique<facebook::jsi::BigInt>(std::move(bigint));
}

inline std::unique_ptr<facebook::jsi::Object> value_as_object(facebook::jsi::Runtime& runtime, const std::unique_ptr<facebook::jsi::Value>& value) {
    facebook::jsi::Object obj = value->asObject(runtime);
    return std::make_unique<facebook::jsi::Object>(std::move(obj));
}

inline std::unique_ptr<facebook::jsi::Function> object_as_function(facebook::jsi::Runtime& runtime, const std::unique_ptr<facebook::jsi::Object>& obj) {
    facebook::jsi::Function func = obj->asFunction(runtime);
    return std::make_unique<facebook::jsi::Function>(std::move(func));
}

inline std::unique_ptr<ValueVec> value_vec_create() {
    return std::make_unique<ValueVec>();
}

inline void value_vec_push(
    ValueVec& vec,
    facebook::jsi::Runtime& runtime,
    const std::unique_ptr<facebook::jsi::Value>& value) {
    vec.values.push_back(facebook::jsi::Value(runtime, *value));
}

inline std::unique_ptr<facebook::jsi::Value> function_call(
    facebook::jsi::Runtime& runtime,
    const std::unique_ptr<facebook::jsi::Function>& func,
    const ValueVec& args) {
    return std::make_unique<facebook::jsi::Value>(
        func->call(runtime, args.values.data(), args.values.size()));
}

inline std::unique_ptr<facebook::jsi::Value> function_call_with_this(
    facebook::jsi::Runtime& runtime,
    const std::unique_ptr<facebook::jsi::Function>& func,
    const facebook::jsi::Object& this_obj,
    const ValueVec& args) {
    return std::make_unique<facebook::jsi::Value>(
        func->callWithThis(runtime, this_obj, args.values.data(), args.values.size()));
}

inline std::unique_ptr<facebook::jsi::Value> function_call_as_constructor(
    facebook::jsi::Runtime& runtime,
    const std::unique_ptr<facebook::jsi::Function>& func,
    const ValueVec& args) {
    return std::make_unique<facebook::jsi::Value>(
        func->callAsConstructor(runtime, args.values.data(), args.values.size()));
}

} // namespace jsi_rs
