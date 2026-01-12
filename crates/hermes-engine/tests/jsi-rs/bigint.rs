mod tests {
    use hermes_engine::{Result, Runtime, RuntimeConfig};

    #[test]
    fn test_jsbigint_from_i64() -> Result<()> {
        let mut runtime = Runtime::new(RuntimeConfig::default())?;
        let mut jsi_runtime = runtime.jsi();

        let bigint = jsi_runtime.create_bigint::<i64>(-9223372036854775808);

        assert!(!bigint.inner().is_null());
        Ok(())
    }

    #[test]
    fn test_jsbigint_from_u64() -> Result<()> {
        let mut runtime = Runtime::new(RuntimeConfig::default())?;
        let mut jsi_runtime = runtime.jsi();

        let bigint = jsi_runtime.create_bigint::<u64>(18446744073709551615);

        assert!(!bigint.inner().is_null());
        Ok(())
    }

    #[test]
    fn test_jsbigint_as_string_default_radix() -> Result<()> {
        let mut runtime = Runtime::new(RuntimeConfig::default())?;
        let mut jsi_runtime = runtime.jsi();

        let bigint = jsi_runtime.create_bigint::<i64>(42);
        let result = bigint.as_string(&mut jsi_runtime)?;
        let value = result.value(&mut jsi_runtime);

        assert_eq!(value, "42");
        Ok(())
    }

    #[test]
    fn test_jsbigint_as_string_negative() -> Result<()> {
        let mut runtime = Runtime::new(RuntimeConfig::default())?;
        let mut jsi_runtime = runtime.jsi();

        let bigint = jsi_runtime.create_bigint::<i64>(-123456789123456789);
        let result = bigint.as_string(&mut jsi_runtime)?;
        let value = result.value(&mut jsi_runtime);

        assert_eq!(value, "-123456789123456789");
        Ok(())
    }

    #[test]
    fn test_jsbigint_as_string_radix_2() -> Result<()> {
        let mut runtime = Runtime::new(RuntimeConfig::default())?;
        let mut jsi_runtime = runtime.jsi();

        let bigint = jsi_runtime.create_bigint::<i64>(42);
        let result = bigint.as_string_opt(&mut jsi_runtime, 2)?;
        let value = result.value(&mut jsi_runtime);

        assert_eq!(value, "101010");
        Ok(())
    }

    #[test]
    fn test_jsbigint_as_string_radix_16() -> Result<()> {
        let mut runtime = Runtime::new(RuntimeConfig::default())?;
        let mut jsi_runtime = runtime.jsi();

        let bigint = jsi_runtime.create_bigint::<i64>(255);
        let result = bigint.as_string_opt(&mut jsi_runtime, 16)?;
        let value = result.value(&mut jsi_runtime);

        assert_eq!(value, "ff");
        Ok(())
    }

    #[test]
    fn test_jsbigint_as_string_radix_32() -> Result<()> {
        let mut runtime = Runtime::new(RuntimeConfig::default())?;
        let mut jsi_runtime = runtime.jsi();

        let bigint = jsi_runtime.create_bigint::<i64>(-123456789123456789);
        let result = bigint.as_string_opt(&mut jsi_runtime, 32)?;
        let value = result.value(&mut jsi_runtime);

        assert_eq!(value, "-3dkr9emd0nol");
        Ok(())
    }

    #[test]
    fn test_jsbigint_as_string_radix_36() -> Result<()> {
        let mut runtime = Runtime::new(RuntimeConfig::default())?;
        let mut jsi_runtime = runtime.jsi();

        let bigint = jsi_runtime.create_bigint::<i64>(1234567890);
        let result = bigint.as_string_opt(&mut jsi_runtime, 36)?;
        let value = result.value(&mut jsi_runtime);

        assert_eq!(value, "kf12oi");
        Ok(())
    }

    #[test]
    fn test_jsbigint_as_string_invalid_radix_too_small() -> Result<()> {
        let mut runtime = Runtime::new(RuntimeConfig::default())?;
        let mut jsi_runtime = runtime.jsi();

        let bigint = jsi_runtime.create_bigint::<i64>(42);
        let result = bigint.as_string_opt(&mut jsi_runtime, 1);

        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn test_jsbigint_as_string_invalid_radix_too_large() -> Result<()> {
        let mut runtime = Runtime::new(RuntimeConfig::default())?;
        let mut jsi_runtime = runtime.jsi();

        let bigint = jsi_runtime.create_bigint::<i64>(42);
        let result = bigint.as_string_opt(&mut jsi_runtime, 37);

        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn test_jsbigint_as_string_large_positive() -> Result<()> {
        let mut runtime = Runtime::new(RuntimeConfig::default())?;
        let mut jsi_runtime = runtime.jsi();

        let bigint = jsi_runtime.create_bigint::<u64>(18446744073709551615);
        let result = bigint.as_string(&mut jsi_runtime)?;
        let value = result.value(&mut jsi_runtime);

        assert_eq!(value, "18446744073709551615");
        Ok(())
    }

    #[test]
    fn test_jsbigint_to_string() -> Result<()> {
        let mut runtime = Runtime::new(RuntimeConfig::default())?;
        let mut jsi_runtime = runtime.jsi();

        let bigint = jsi_runtime.create_bigint::<i64>(-123456789);
        let value = bigint.to_string(&mut jsi_runtime)?;

        assert_eq!(value, "-123456789");
        Ok(())
    }
}
