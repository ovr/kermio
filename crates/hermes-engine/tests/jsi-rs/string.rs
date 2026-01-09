mod tests {
    use hermes_engine::jsi::JSString;
    use hermes_engine::{Result, Runtime, RuntimeConfig};

    #[test]
    fn test_jsstring_new() -> Result<()> {
        let mut runtime = Runtime::new(RuntimeConfig::default())?;
        let mut jsi_runtime = runtime.jsi();

        let string = JSString::new(&mut jsi_runtime, "Hello, World!");

        assert!(!string.inner().is_null());
        Ok(())
    }

    #[test]
    fn test_jsstring_value() -> Result<()> {
        let mut runtime = Runtime::new(RuntimeConfig::default())?;
        let mut jsi_runtime = runtime.jsi();

        let string = JSString::new(&mut jsi_runtime, "Hello, World!");
        let value = string.value(&mut jsi_runtime);

        assert_eq!(value, "Hello, World!");
        Ok(())
    }
}
