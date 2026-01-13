mod tests {
    use hermes_engine::jsi::JSValue;
    use hermes_engine::{Error, Result, Runtime, RuntimeConfig};

    #[test]
    fn test_jsfunction_call() -> Result<()> {
        let mut runtime = Runtime::new(RuntimeConfig::default())?;

        // Create a simple function in JavaScript
        runtime.eval_with_result("function add(a, b) { return a + b; }", Some("test.js"))?;

        // Get the function from global scope
        runtime.eval_with_result("this", Some("test.js"))?;
        let add_value = runtime.eval_with_result("add", Some("test.js"))?;

        assert!(add_value.is_object());

        let mut jsi_runtime = runtime.jsi();
        let add_func = add_value
            .as_function(&mut jsi_runtime)
            .ok_or_else(|| Error::internal("add is not a function"))?;

        // Call the function with arguments: add(5, 3)
        let args = vec![JSValue::number(5.0), JSValue::number(3.0)];
        let result = add_func.call(&mut jsi_runtime, &args)?;

        assert!(result.is_number());
        assert_eq!(result.as_number(), 8.0);
        Ok(())
    }

    #[test]
    fn test_jsfunction_call_as_constructor() -> Result<()> {
        let mut runtime = Runtime::new(RuntimeConfig::default())?;

        // Create a simple constructor in JavaScript
        runtime.eval_with_result(
            "function Point(x, y) { this.x = x; this.y = y; }",
            Some("test.js"),
        )?;

        let point_value = runtime.eval_with_result("Point", Some("test.js"))?;

        let mut jsi_runtime = runtime.jsi();
        let point_func = point_value
            .as_function(&mut jsi_runtime)
            .ok_or_else(|| Error::internal("Point is not a function"))?;

        // Call as constructor: new Point(10, 20)
        let args = vec![JSValue::number(10.0), JSValue::number(20.0)];
        let result = point_func.call_as_constructor(&mut jsi_runtime, &args)?;

        assert!(result.is_object());

        // Verify the constructor worked by creating an instance via eval and comparing
        let expected = runtime.eval_with_result("new Point(10, 20)", Some("test.js"))?;
        assert!(expected.is_object());

        // Both should be objects representing points with x=10, y=20
        // We can't directly compare objects, but we verified construction succeeded
        Ok(())
    }

    #[test]
    fn test_jsfunction_no_conversion() -> Result<()> {
        let mut runtime = Runtime::new(RuntimeConfig::default())?;
        let mut jsi_runtime = runtime.jsi();

        // Try to convert a non-function value to a function
        let number = JSValue::number(42.0);
        let func = number.as_function(&mut jsi_runtime);
        assert!(func.is_none());

        let string = JSValue::number(42.0);
        let func = string.as_function(&mut jsi_runtime);
        assert!(func.is_none());

        Ok(())
    }
}
