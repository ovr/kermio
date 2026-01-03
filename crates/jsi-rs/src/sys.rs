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
    }
}
