// Re-export jsi-rs as the jsi module for JSI API access
pub use jsi_rs as jsi;

// CXX bridge module
mod bridge;

mod config;
pub use config::{RuntimeConfig, RuntimeConfigBuilder};

mod runtime;
pub use runtime::{PreparedJavaScript, Runtime};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::RuntimeConfig;

    #[test]
    fn test_eval_simple() {
        let mut runtime = Runtime::new(RuntimeConfig::default()).expect("Failed to create runtime");
        runtime.eval("2 + 2", None).expect("Failed to evaluate");
    }

    #[test]
    fn test_eval_error() {
        let mut runtime = Runtime::new(RuntimeConfig::default()).expect("Failed to create runtime");
        let result = runtime.eval("throw new Error('test error')", None);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("test error"));
    }

    #[test]
    fn test_eval_with_result_number() {
        let mut runtime = Runtime::new(RuntimeConfig::default()).expect("Failed to create runtime");
        let result = runtime
            .eval_with_result("2 + 2", None)
            .expect("Failed to evaluate");
        assert!(result.is_number());
    }

    #[test]
    fn test_eval_with_result_string() {
        let mut runtime = Runtime::new(RuntimeConfig::default()).expect("Failed to create runtime");
        let result = runtime
            .eval_with_result("'hello'", None)
            .expect("Failed to evaluate");
        assert!(result.is_string());
    }

    #[test]
    fn test_eval_with_result_bool() {
        let mut runtime = Runtime::new(RuntimeConfig::default()).expect("Failed to create runtime");
        let result = runtime
            .eval_with_result("true", None)
            .expect("Failed to evaluate");
        assert!(result.is_bool());
    }

    #[test]
    fn test_eval_with_result_null() {
        let mut runtime = Runtime::new(RuntimeConfig::default()).expect("Failed to create runtime");
        let result = runtime
            .eval_with_result("null", None)
            .expect("Failed to evaluate");
        assert!(result.is_null());
    }

    #[test]
    fn test_eval_with_result_undefined() {
        let mut runtime = Runtime::new(RuntimeConfig::default()).expect("Failed to create runtime");
        let result = runtime
            .eval_with_result("undefined", None)
            .expect("Failed to evaluate");
        assert!(result.is_undefined());
    }

    #[test]
    fn test_eval_with_result_object() {
        let mut runtime = Runtime::new(RuntimeConfig::default()).expect("Failed to create runtime");
        let result = runtime
            .eval_with_result("({foo: 'bar'})", None)
            .expect("Failed to evaluate");
        assert!(result.is_object());
    }

    #[test]
    fn test_runtime_config_builder() {
        let config = RuntimeConfigBuilder::new()
            .heap_size(64 << 20, 512 << 20) // 64MB init, 512MB max
            .enable_eval(false)
            .enable_jit(false)
            .enable_es6_proxy(true)
            .build();

        let mut runtime = Runtime::new(config).expect("Failed to create runtime");

        // Test that eval is disabled
        let result = runtime.eval("eval('1 + 1')", None);
        assert!(result.is_err());
    }

    #[test]
    fn test_runtime_config_disable_generators() {
        let config = RuntimeConfigBuilder::new().enable_generator(false).build();

        let mut runtime = Runtime::new(config).expect("Failed to create runtime");

        // Test that generators are disabled
        let result = runtime.eval("function* gen() { yield 1; }", None);
        assert!(result.is_err());
    }

    #[test]
    fn test_runtime_config_heap_settings() {
        let config = RuntimeConfigBuilder::new()
            .heap_size(16 << 20, 32 << 20) // 16MB init, 32MB max
            .build();

        let runtime = Runtime::new(config).expect("Failed to create runtime with heap settings");
        drop(runtime);
    }

    #[test]
    fn test_prepare_javascript() {
        let mut runtime = Runtime::new(RuntimeConfig::default()).expect("Failed to create runtime");

        let prepared = runtime
            .prepare_javascript("2 + 2", Some("calc.js"))
            .expect("Failed to prepare JavaScript");

        let result = runtime
            .evaluate_prepared_javascript(&prepared)
            .expect("Failed to evaluate prepared JavaScript");

        assert!(result.is_number());
    }

    #[test]
    fn test_prepare_javascript_multiple_executions() {
        let mut runtime = Runtime::new(RuntimeConfig::default()).expect("Failed to create runtime");

        let prepared = runtime
            .prepare_javascript("Math.random()", None)
            .expect("Failed to prepare JavaScript");

        // Execute multiple times - should work each time
        let result1 = runtime
            .evaluate_prepared_javascript(&prepared)
            .expect("First execution failed");
        assert!(result1.is_number());

        let result2 = runtime
            .evaluate_prepared_javascript(&prepared)
            .expect("Second execution failed");
        assert!(result2.is_number());

        let result3 = runtime
            .evaluate_prepared_javascript(&prepared)
            .expect("Third execution failed");
        assert!(result3.is_number());
    }

    #[test]
    fn test_prepare_javascript_with_string() {
        let mut runtime = Runtime::new(RuntimeConfig::default()).expect("Failed to create runtime");

        let prepared = runtime
            .prepare_javascript("'hello ' + 'world'", None)
            .expect("Failed to prepare JavaScript");

        let result = runtime
            .evaluate_prepared_javascript(&prepared)
            .expect("Failed to evaluate prepared JavaScript");

        assert!(result.is_string());
    }

    #[test]
    fn test_prepare_javascript_syntax_error() {
        let mut runtime = Runtime::new(RuntimeConfig::default()).expect("Failed to create runtime");

        let result = runtime.prepare_javascript("this is invalid javascript", None);
        assert!(result.is_err());
    }

    #[test]
    fn test_prepare_javascript_runtime_error() {
        let mut runtime = Runtime::new(RuntimeConfig::default()).expect("Failed to create runtime");

        // Preparation should succeed (syntax is valid)
        let prepared = runtime
            .prepare_javascript("throw new Error('test error')", None)
            .expect("Failed to prepare JavaScript");

        // But execution should fail
        match runtime.evaluate_prepared_javascript(&prepared) {
            Ok(_) => panic!("Expected error but got success"),
            Err(error_msg) => assert!(error_msg.contains("test error")),
        }
    }
}
