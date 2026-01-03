#[cfg(feature = "unsafe")]
mod tests {
    use hermes_engine::jsi::JSString;
    use hermes_engine::Runtime;

    #[test]
    fn test_jsstring_new() {
        let mut runtime = Runtime::new().expect("Failed to create runtime");
        let mut jsi_runtime = runtime.jsi();

        let string = JSString::new(&mut jsi_runtime, "Hello, World!");

        assert!(!string.inner().is_null());
    }
}
