#[cfg(feature = "unsafe")]
mod tests {
    use hermes_engine::jsi::JSPropNameID;
    use hermes_engine::Runtime;

    #[test]
    fn test_jspropnameid_new() {
        let mut runtime = Runtime::new().expect("Failed to create runtime");
        let mut jsi_runtime = runtime.jsi();

        let propname = JSPropNameID::new(&mut jsi_runtime, "myProperty");

        assert!(!propname.inner().is_null());
    }
}
