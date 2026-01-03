#[cfg(feature = "unsafe")]
mod tests {
    use hermes_engine::jsi::JSBigInt;
    use hermes_engine::Runtime;

    #[test]
    fn test_jsbigint_from_i64() {
        let mut runtime = Runtime::new().expect("Failed to create runtime");
        let mut jsi_runtime = runtime.jsi();

        let bigint = JSBigInt::from_i64(&mut jsi_runtime, -9223372036854775808);

        assert!(!bigint.inner().is_null());
    }

    #[test]
    fn test_jsbigint_from_u64() {
        let mut runtime = Runtime::new().expect("Failed to create runtime");
        let mut jsi_runtime = runtime.jsi();

        let bigint = JSBigInt::from_u64(&mut jsi_runtime, 18446744073709551615);

        assert!(!bigint.inner().is_null());
    }
}
