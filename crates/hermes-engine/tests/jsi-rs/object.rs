mod tests {
    use hermes_engine::jsi::{JSObject, JSValue};
    use hermes_engine::{Result, Runtime, RuntimeConfig};

    #[test]
    fn test_jsobject_new() -> Result<()> {
        let mut runtime = Runtime::new(RuntimeConfig::default())?;
        let mut jsi_runtime = runtime.jsi();

        let obj = JSObject::new(&mut jsi_runtime);

        assert!(!obj.inner().is_null());
        Ok(())
    }

    #[test]
    fn test_jsobject_get_set_property() -> Result<()> {
        let mut runtime = Runtime::new(RuntimeConfig::default())?;
        let mut jsi_runtime = runtime.jsi();

        let obj = JSObject::new(&mut jsi_runtime);

        // Set a number property
        let value = JSValue::number(42.0);
        obj.set(&mut jsi_runtime, "answer", &value);

        // Get the property back
        let result = obj.get(&mut jsi_runtime, "answer");
        assert!(result.is_number());
        assert_eq!(result.as_number(), 42.0);

        Ok(())
    }

    #[test]
    fn test_jsobject_get_undefined_property() -> Result<()> {
        let mut runtime = Runtime::new(RuntimeConfig::default())?;
        let mut jsi_runtime = runtime.jsi();

        let obj = JSObject::new(&mut jsi_runtime);

        // Get a non-existent property should return undefined
        let result = obj.get(&mut jsi_runtime, "nonexistent");
        assert!(result.is_undefined());

        Ok(())
    }

    #[test]
    fn test_jsobject_has_property() -> Result<()> {
        let mut runtime = Runtime::new(RuntimeConfig::default())?;
        let mut jsi_runtime = runtime.jsi();

        let obj = JSObject::new(&mut jsi_runtime);

        // Initially the property doesn't exist
        assert!(!obj.has(&mut jsi_runtime, "foo"));

        // Set the property
        let value = JSValue::bool(true);
        obj.set(&mut jsi_runtime, "foo", &value);

        // Now it exists
        assert!(obj.has(&mut jsi_runtime, "foo"));

        Ok(())
    }

    #[test]
    fn test_jsobject_delete_property() -> Result<()> {
        let mut runtime = Runtime::new(RuntimeConfig::default())?;
        let mut jsi_runtime = runtime.jsi();

        let obj = JSObject::new(&mut jsi_runtime);

        // Set a property
        let value = JSValue::number(123.0);
        obj.set(&mut jsi_runtime, "temp", &value);
        assert!(obj.has(&mut jsi_runtime, "temp"));

        // Delete it
        obj.delete(&mut jsi_runtime, "temp");

        // The property value should now be undefined
        let result = obj.get(&mut jsi_runtime, "temp");
        assert!(result.is_undefined());

        Ok(())
    }

    #[test]
    fn test_jsobject_get_property_names() -> Result<()> {
        let mut runtime = Runtime::new(RuntimeConfig::default())?;
        let mut jsi_runtime = runtime.jsi();

        let obj = JSObject::new(&mut jsi_runtime);

        // Set some properties
        obj.set(&mut jsi_runtime, "a", &JSValue::number(1.0));
        obj.set(&mut jsi_runtime, "b", &JSValue::number(2.0));
        obj.set(&mut jsi_runtime, "c", &JSValue::number(3.0));

        // Get property names
        let names = obj.get_property_names(&mut jsi_runtime);
        assert_eq!(names.len(&mut jsi_runtime), 3);

        Ok(())
    }

    #[test]
    fn test_jsobject_set_multiple_types() -> Result<()> {
        let mut runtime = Runtime::new(RuntimeConfig::default())?;
        let mut jsi_runtime = runtime.jsi();

        let obj = JSObject::new(&mut jsi_runtime);

        // Set various property types
        obj.set(&mut jsi_runtime, "num", &JSValue::number(3.14));
        obj.set(&mut jsi_runtime, "bool", &JSValue::bool(true));
        obj.set(&mut jsi_runtime, "nil", &JSValue::null());
        obj.set(&mut jsi_runtime, "undef", &JSValue::undefined());

        // Verify each type
        assert!(obj.get(&mut jsi_runtime, "num").is_number());
        assert!(obj.get(&mut jsi_runtime, "bool").is_bool());
        assert!(obj.get(&mut jsi_runtime, "nil").is_null());
        assert!(obj.get(&mut jsi_runtime, "undef").is_undefined());

        Ok(())
    }
}
