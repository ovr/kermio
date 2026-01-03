#[cfg(feature = "unsafe")]
mod tests {
    use hermes_engine::jsi::JSValue;
    use hermes_engine::Runtime;

    #[test]
    fn test_jsvalue_undefined() {
        let mut _runtime = Runtime::new().expect("Failed to create runtime");

        let value = JSValue::undefined();

        assert!(value.is_undefined());
        assert!(!value.is_null());
    }

    #[test]
    fn test_jsvalue_null() {
        let mut _runtime = Runtime::new().expect("Failed to create runtime");

        let value = JSValue::null();

        assert!(value.is_null());
        assert!(!value.is_undefined());
    }

    #[test]
    fn test_jsvalue_bool() {
        let mut _runtime = Runtime::new().expect("Failed to create runtime");

        let value_true = JSValue::bool(true);
        let value_false = JSValue::bool(false);

        assert!(value_true.is_bool());
        assert!(value_false.is_bool());
    }

    #[test]
    fn test_jsvalue_number() {
        let mut _runtime = Runtime::new().expect("Failed to create runtime");

        let value = JSValue::number(42.5);

        assert!(value.is_number());
        assert!(!value.is_bool());
    }
}
