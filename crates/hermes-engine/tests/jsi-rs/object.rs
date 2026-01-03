#[cfg(feature = "unsafe")]
mod tests {
    use hermes_engine::jsi::JSObject;
    use hermes_engine::Runtime;

    #[test]
    fn test_jsobject_new() {
        let mut runtime = Runtime::new().expect("Failed to create runtime");
        let mut jsi_runtime = runtime.jsi();

        let obj = JSObject::new(&mut jsi_runtime);

        assert!(!obj.inner().is_null());
    }
}
