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
    }

    // Helper functions for creating JSI objects
    unsafe extern "C++" {
        include!("jsi-rs/src/bridge.h");

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
    }
}
