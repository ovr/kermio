mod tests {
    use hermes_engine::jsi::JSArray;
    use hermes_engine::{Result, Runtime, RuntimeConfig};

    #[test]
    fn test_jsarray_new() -> Result<()> {
        let mut runtime = Runtime::new(RuntimeConfig::default())?;
        let mut jsi_runtime = runtime.jsi();

        let array = JSArray::new(&mut jsi_runtime, 5);

        assert!(!array.inner().is_null());
        Ok(())
    }
}
