mod tests {
    use hermes_engine::jsi::JSRuntime;
    use hermes_engine::{Result, Runtime, RuntimeConfig};

    #[test]
    fn test_jsruntime_create_undefined() -> Result<()> {
        let value = JSRuntime::create_undefined();

        assert!(value.is_undefined());
        Ok(())
    }

    #[test]
    fn test_jsruntime_create_null() -> Result<()> {
        let value = JSRuntime::create_null();

        assert!(value.is_null());
        Ok(())
    }

    #[test]
    fn test_jsruntime_create_bool() -> Result<()> {
        let value_true = JSRuntime::create_bool(true);
        let value_false = JSRuntime::create_bool(false);

        assert!(value_true.is_bool());
        assert!(value_false.is_bool());
        Ok(())
    }

    #[test]
    fn test_jsruntime_create_number() -> Result<()> {
        let value = JSRuntime::create_number(42.5);

        assert!(value.is_number());
        Ok(())
    }

    #[test]
    fn test_jsruntime_create_string() -> Result<()> {
        let mut runtime = Runtime::new(RuntimeConfig::default())?;
        let mut jsi_runtime = runtime.jsi();

        let string = jsi_runtime.create_string("Hello, World!");

        assert!(!string.inner().is_null());
        Ok(())
    }

    #[test]
    fn test_jsruntime_create_object() -> Result<()> {
        let mut runtime = Runtime::new(RuntimeConfig::default())?;
        let mut jsi_runtime = runtime.jsi();

        let obj = jsi_runtime.create_object();

        assert!(!obj.inner().is_null());
        Ok(())
    }

    #[test]
    fn test_jsruntime_create_array_empty() -> Result<()> {
        let mut runtime = Runtime::new(RuntimeConfig::default())?;
        let mut jsi_runtime = runtime.jsi();

        let array = jsi_runtime.create_array_empty();

        assert!(!array.inner().is_null());
        Ok(())
    }

    #[test]
    fn test_jsruntime_create_array() -> Result<()> {
        let mut runtime = Runtime::new(RuntimeConfig::default())?;
        let mut jsi_runtime = runtime.jsi();

        let array = jsi_runtime.create_array(5);

        assert!(!array.inner().is_null());
        Ok(())
    }

    #[test]
    fn test_jsruntime_create_prop_name_id() -> Result<()> {
        let mut runtime = Runtime::new(RuntimeConfig::default())?;
        let mut jsi_runtime = runtime.jsi();

        let prop_id = jsi_runtime.create_prop_name_id("foo");

        assert!(!prop_id.inner().is_null());
        Ok(())
    }

    #[test]
    fn test_jsruntime_create_bigint_i64() -> Result<()> {
        let mut runtime = Runtime::new(RuntimeConfig::default())?;
        let mut jsi_runtime = runtime.jsi();

        let bigint = jsi_runtime.create_bigint::<i64>(-9223372036854775808);

        assert!(!bigint.inner().is_null());
        Ok(())
    }

    #[test]
    fn test_jsruntime_create_bigint_u64() -> Result<()> {
        let mut runtime = Runtime::new(RuntimeConfig::default())?;
        let mut jsi_runtime = runtime.jsi();

        let bigint = jsi_runtime.create_bigint::<u64>(18446744073709551615);

        assert!(!bigint.inner().is_null());
        Ok(())
    }
}
