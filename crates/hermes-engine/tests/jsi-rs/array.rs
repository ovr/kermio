mod tests {
    use hermes_engine::jsi::{JSArray, JSValue};
    use hermes_engine::{Result, Runtime, RuntimeConfig};

    #[test]
    fn test_jsarray_new() -> Result<()> {
        let mut runtime = Runtime::new(RuntimeConfig::default())?;
        let mut jsi_runtime = runtime.jsi();

        let array = JSArray::new(&mut jsi_runtime, 5);

        assert!(!array.inner().is_null());
        Ok(())
    }

    #[test]
    fn test_jsarray_get_set() -> Result<()> {
        let mut runtime = Runtime::new(RuntimeConfig::default())?;
        let mut jsi_runtime = runtime.jsi();

        let array = JSArray::new(&mut jsi_runtime, 3);

        let val1 = JSValue::number(42.0);
        let val2 = JSValue::bool(true);
        let val3 = JSValue::null();

        array.set(&mut jsi_runtime, 0, &val1)?;
        array.set(&mut jsi_runtime, 1, &val2)?;
        array.set(&mut jsi_runtime, 2, &val3)?;

        let retrieved1 = array.get(&mut jsi_runtime, 0);
        let retrieved2 = array.get(&mut jsi_runtime, 1);
        let retrieved3 = array.get(&mut jsi_runtime, 2);

        assert!(retrieved1.is_number());
        assert_eq!(retrieved1.as_number(), 42.0);
        assert!(retrieved2.is_bool());
        assert_eq!(retrieved2.as_bool(), true);
        assert!(retrieved3.is_null());

        Ok(())
    }

    #[test]
    fn test_jsarray_length() -> Result<()> {
        let mut runtime = Runtime::new(RuntimeConfig::default())?;
        let mut jsi_runtime = runtime.jsi();

        let array = JSArray::new(&mut jsi_runtime, 5);

        assert_eq!(array.len(&mut jsi_runtime), 5);
        assert!(!array.is_empty(&mut jsi_runtime));

        let empty_array = JSArray::new(&mut jsi_runtime, 0);
        assert!(empty_array.is_empty(&mut jsi_runtime));

        Ok(())
    }

    #[test]
    fn test_jsarray_out_of_bounds() -> Result<()> {
        let mut runtime = Runtime::new(RuntimeConfig::default())?;
        let mut jsi_runtime = runtime.jsi();

        let array = JSArray::new(&mut jsi_runtime, 3);

        let value = array.get(&mut jsi_runtime, 10);
        assert!(value.is_undefined());

        let val = JSValue::number(42.0);
        let result = array.set(&mut jsi_runtime, 10, &val);
        assert!(result.is_err());

        Ok(())
    }
}
