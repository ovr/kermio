mod tests {
    use hermes_engine::jsi::JSBigInt;
    use hermes_engine::{Result, Runtime, RuntimeConfig};

    #[test]
    fn test_jsbigint_from_i64() -> Result<()> {
        let mut runtime = Runtime::new(RuntimeConfig::default())?;
        let mut jsi_runtime = runtime.jsi();

        let bigint = JSBigInt::from_i64(&mut jsi_runtime, -9223372036854775808);

        assert!(!bigint.inner().is_null());
        Ok(())
    }

    #[test]
    fn test_jsbigint_from_u64() -> Result<()> {
        let mut runtime = Runtime::new(RuntimeConfig::default())?;
        let mut jsi_runtime = runtime.jsi();

        let bigint = JSBigInt::from_u64(&mut jsi_runtime, 18446744073709551615);

        assert!(!bigint.inner().is_null());
        Ok(())
    }
}
