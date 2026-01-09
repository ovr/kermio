mod tests {
    use hermes_engine::jsi::JSObject;
    use hermes_engine::{Result, Runtime, RuntimeConfig};

    #[test]
    fn test_jsobject_new() -> Result<()> {
        let mut runtime = Runtime::new(RuntimeConfig::default())?;
        let mut jsi_runtime = runtime.jsi();

        let obj = JSObject::new(&mut jsi_runtime);

        assert!(!obj.inner().is_null());
        Ok(())
    }
}
