mod tests {
    use hermes_engine::jsi::JSValue;
    use hermes_engine::{Runtime, RuntimeConfig};

    #[test]
    fn test_jsvalue_undefined() -> Result<(), Box<dyn std::error::Error>> {
        let _runtime = Runtime::new(RuntimeConfig::default())?;

        let value = JSValue::undefined();

        assert!(value.is_undefined());
        assert!(!value.is_null());
        Ok(())
    }

    #[test]
    fn test_jsvalue_null() -> Result<(), Box<dyn std::error::Error>> {
        let _runtime = Runtime::new(RuntimeConfig::default())?;

        let value = JSValue::null();

        assert!(value.is_null());
        assert!(!value.is_undefined());
        Ok(())
    }

    #[test]
    fn test_jsvalue_bool() -> Result<(), Box<dyn std::error::Error>> {
        let _runtime = Runtime::new(RuntimeConfig::default())?;

        let value_true = JSValue::bool(true);
        let value_false = JSValue::bool(false);

        assert!(value_true.is_bool());
        assert!(value_false.is_bool());
        Ok(())
    }

    #[test]
    fn test_jsvalue_number() -> Result<(), Box<dyn std::error::Error>> {
        let _runtime = Runtime::new(RuntimeConfig::default())?;

        let value = JSValue::number(42.5);

        assert!(value.is_number());
        assert!(!value.is_bool());
        Ok(())
    }

    #[test]
    fn test_jsvalue_as_bool() -> Result<(), Box<dyn std::error::Error>> {
        let _runtime = Runtime::new(RuntimeConfig::default())?;

        let value_true = JSValue::bool(true);
        let value_false = JSValue::bool(false);

        assert_eq!(value_true.as_bool(), true);
        assert_eq!(value_false.as_bool(), false);
        Ok(())
    }

    #[test]
    fn test_jsvalue_as_number() -> Result<(), Box<dyn std::error::Error>> {
        let _runtime = Runtime::new(RuntimeConfig::default())?;

        let value = JSValue::number(42.5);

        assert_eq!(value.as_number(), 42.5);
        Ok(())
    }

    #[test]
    fn test_jsvalue_as_string() -> Result<(), Box<dyn std::error::Error>> {
        let mut runtime = Runtime::new(RuntimeConfig::default())?;

        let value = runtime.eval_with_result("String('hello world')", None)?;

        assert!(value.is_string());

        let str_value = value.as_string(&mut runtime.jsi()).ok_or("Not a string")?;
        assert_eq!(str_value.value(&mut runtime.jsi()), "hello world");
        Ok(())
    }

    #[test]
    fn test_jsvalue_as_string_none() -> Result<(), Box<dyn std::error::Error>> {
        let mut runtime = Runtime::new(RuntimeConfig::default())?;

        let value = JSValue::number(42.0);

        assert!(value.is_number());
        assert!(value.as_string(&mut runtime.jsi()).is_none());
        Ok(())
    }

    #[test]
    fn test_jsvalue_as_bigint() -> Result<(), Box<dyn std::error::Error>> {
        let mut runtime = Runtime::new(RuntimeConfig::default())?;

        let value = runtime.eval_with_result("123n", None)?;

        assert!(value.is_bigint());

        let _bigint_value = value.as_bigint(&mut runtime.jsi()).ok_or("Not a bigint")?;
        Ok(())
    }

    #[test]
    fn test_jsvalue_as_bigint_none() -> Result<(), Box<dyn std::error::Error>> {
        let mut runtime = Runtime::new(RuntimeConfig::default())?;

        let value = JSValue::number(42.0);

        assert!(value.is_number());
        assert!(value.as_bigint(&mut runtime.jsi()).is_none());
        Ok(())
    }
}
