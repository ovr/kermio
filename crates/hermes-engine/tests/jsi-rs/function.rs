mod tests {
    use hermes_engine::jsi::{JSFunction, JSValue};
    use hermes_engine::{Runtime, RuntimeConfig};

    #[test]
    fn test_jsfunction_call() -> Result<(), Box<dyn std::error::Error>> {
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
            .ok_or("add is not a function")?;

        // Call the function (args currently not supported)
        let args = vec![];
        let result = add_func.call(&mut jsi_runtime, &args)?;

        assert!(result.is_number());
        Ok(())
    }

    #[test]
    fn test_jsfunction_call_as_constructor() -> Result<(), Box<dyn std::error::Error>> {
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
            .ok_or("Point is not a function")?;

        // Call as constructor (args currently not supported)
        let args = vec![];
        let result = point_func.call_as_constructor(&mut jsi_runtime, &args)?;

        assert!(result.is_object());
        Ok(())
    }

    #[test]
    fn test_jsfunction_no_conversion() -> Result<(), Box<dyn std::error::Error>> {
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
