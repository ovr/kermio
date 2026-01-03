#[cfg(feature = "unsafe")]
mod tests {
    use hermes_engine::jsi::JSArray;
    use hermes_engine::Runtime;

    #[test]
    fn test_jsarray_new() {
        let mut runtime = Runtime::new().expect("Failed to create runtime");
        let mut jsi_runtime = runtime.jsi();

        let array = JSArray::new(&mut jsi_runtime, 5);

        assert!(!array.inner().is_null());
    }
}
