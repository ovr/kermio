#[cxx::bridge]
pub mod ffi {
    // Opaque C++ types from facebook::jsi namespace
    unsafe extern "C++" {
        include!("jsi/jsi.h");

        // JSI Runtime - the main execution context
        #[namespace = "facebook::jsi"]
        #[cxx_name = "Runtime"]
        type JSIRuntime;

        // JSI Value types - using JSI prefix for internal FFI types
        #[namespace = "facebook::jsi"]
        #[cxx_name = "Value"]
        type JSIValue;

        // JSI Value member methods
        #[namespace = "facebook::jsi"]
        fn isUndefined(self: &JSIValue) -> bool;

        #[namespace = "facebook::jsi"]
        fn isNull(self: &JSIValue) -> bool;

        #[namespace = "facebook::jsi"]
        fn isBool(self: &JSIValue) -> bool;

        #[namespace = "facebook::jsi"]
        fn isNumber(self: &JSIValue) -> bool;

        #[namespace = "facebook::jsi"]
        fn isString(self: &JSIValue) -> bool;

        #[namespace = "facebook::jsi"]
        fn isObject(self: &JSIValue) -> bool;

        #[namespace = "facebook::jsi"]
        fn isBigInt(self: &JSIValue) -> bool;

        #[namespace = "facebook::jsi"]
        #[cxx_name = "String"]
        type JSIString;

        #[namespace = "facebook::jsi"]
        #[cxx_name = "Object"]
        type JSIObject;

        #[namespace = "facebook::jsi"]
        #[cxx_name = "Array"]
        type JSIArray;

        #[namespace = "facebook::jsi"]
        #[cxx_name = "Function"]
        type JSIFunction;

        #[namespace = "facebook::jsi"]
        #[cxx_name = "PropNameID"]
        type JSIPropNameID;

        #[namespace = "facebook::jsi"]
        #[cxx_name = "BigInt"]
        type JSIBigInt;

        // Helper functions for type conversions
        #[namespace = "jsi_rs"]
        fn value_as_object(
            runtime: Pin<&mut JSIRuntime>,
            value: &UniquePtr<JSIValue>,
        ) -> UniquePtr<JSIObject>;

        #[namespace = "jsi_rs"]
        fn object_as_function(
            runtime: Pin<&mut JSIRuntime>,
            obj: &UniquePtr<JSIObject>,
        ) -> UniquePtr<JSIFunction>;

        #[namespace = "jsi_rs"]
        fn value_as_bool(value: &UniquePtr<JSIValue>) -> bool;

        #[namespace = "jsi_rs"]
        fn value_as_number(value: &UniquePtr<JSIValue>) -> f64;

        #[namespace = "jsi_rs"]
        fn value_as_string(
            runtime: Pin<&mut JSIRuntime>,
            value: &UniquePtr<JSIValue>,
        ) -> UniquePtr<JSIString>;

        #[namespace = "jsi_rs"]
        fn value_as_bigint(
            runtime: Pin<&mut JSIRuntime>,
            value: &UniquePtr<JSIValue>,
        ) -> UniquePtr<JSIBigInt>;
    }

    // Helper functions for creating JSI objects
    unsafe extern "C++" {
        include!("jsi-rs/src/bridge.h");

        #[namespace = "jsi_rs"]
        type ValueVec;

        #[namespace = "jsi_rs"]
        fn create_object(runtime: Pin<&mut JSIRuntime>) -> UniquePtr<JSIObject>;

        #[namespace = "jsi_rs"]
        fn create_array(runtime: Pin<&mut JSIRuntime>, length: usize) -> UniquePtr<JSIArray>;

        #[namespace = "jsi_rs"]
        fn create_string_from_utf8(
            runtime: Pin<&mut JSIRuntime>,
            data: &str,
        ) -> UniquePtr<JSIString>;

        #[namespace = "jsi_rs"]
        fn create_propnameid_from_utf8(
            runtime: Pin<&mut JSIRuntime>,
            data: &str,
        ) -> UniquePtr<JSIPropNameID>;

        #[namespace = "jsi_rs"]
        fn create_bigint_from_i64(
            runtime: Pin<&mut JSIRuntime>,
            value: i64,
        ) -> UniquePtr<JSIBigInt>;

        #[namespace = "jsi_rs"]
        fn create_bigint_from_u64(
            runtime: Pin<&mut JSIRuntime>,
            value: u64,
        ) -> UniquePtr<JSIBigInt>;

        #[namespace = "jsi_rs"]
        fn create_value_undefined() -> UniquePtr<JSIValue>;

        #[namespace = "jsi_rs"]
        fn create_value_null() -> UniquePtr<JSIValue>;

        #[namespace = "jsi_rs"]
        fn create_value_bool(value: bool) -> UniquePtr<JSIValue>;

        #[namespace = "jsi_rs"]
        fn create_value_number(value: f64) -> UniquePtr<JSIValue>;

        #[namespace = "jsi_rs"]
        fn string_to_utf8(runtime: Pin<&mut JSIRuntime>, str: &UniquePtr<JSIString>) -> String;

        #[namespace = "jsi_rs"]
        fn bigint_to_string(
            runtime: Pin<&mut JSIRuntime>,
            bigint: &UniquePtr<JSIBigInt>,
            radix: i32,
        ) -> Result<UniquePtr<JSIString>>;

        #[namespace = "jsi_rs"]
        fn value_vec_create() -> UniquePtr<ValueVec>;

        #[namespace = "jsi_rs"]
        fn value_vec_push(
            vec: Pin<&mut ValueVec>,
            runtime: Pin<&mut JSIRuntime>,
            value: &UniquePtr<JSIValue>,
        );

        #[namespace = "jsi_rs"]
        fn function_call(
            runtime: Pin<&mut JSIRuntime>,
            func: &UniquePtr<JSIFunction>,
            args: &ValueVec,
        ) -> UniquePtr<JSIValue>;

        #[namespace = "jsi_rs"]
        fn function_call_with_this(
            runtime: Pin<&mut JSIRuntime>,
            func: &UniquePtr<JSIFunction>,
            this_obj: &JSIObject,
            args: &ValueVec,
        ) -> UniquePtr<JSIValue>;

        #[namespace = "jsi_rs"]
        fn function_call_as_constructor(
            runtime: Pin<&mut JSIRuntime>,
            func: &UniquePtr<JSIFunction>,
            args: &ValueVec,
        ) -> UniquePtr<JSIValue>;
    }
}
