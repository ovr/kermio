mod tests {
    use hermes_engine::jsi::JSPropNameID;
    use hermes_engine::{Result, Runtime, RuntimeConfig};

    #[test]
    fn test_jspropnameid_new() -> Result<()> {
        let mut runtime = Runtime::new(RuntimeConfig::default())?;
        let mut jsi_runtime = runtime.jsi();

        let propname = JSPropNameID::new(&mut jsi_runtime, "myProperty");

        assert!(!propname.inner().is_null());
        Ok(())
    }
}
