// Re-export jsi-rs as the jsi module for JSI API access
pub use jsi_rs as jsi;

// CXX bridge module
mod bridge;

mod error;
pub use error::{Error, Result};

mod config;
pub use config::{RuntimeConfig, RuntimeConfigBuilder};

mod runtime;
pub use runtime::{CompiledBytecode, PreparedJavaScript, Runtime};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::RuntimeConfig;

    #[test]
    fn test_eval_simple() -> Result<()> {
        let mut runtime = Runtime::new(RuntimeConfig::default())?;
        runtime.eval("2 + 2", None)?;
        Ok(())
    }

    #[test]
    fn test_eval_error() -> Result<()> {
        let mut runtime = Runtime::new(RuntimeConfig::default())?;
        let result = runtime.eval("throw new Error('test error')", None);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("test error"));
        Ok(())
    }

    #[test]
    fn test_eval_with_result_number() -> Result<()> {
        let mut runtime = Runtime::new(RuntimeConfig::default())?;
        let result = runtime.eval_with_result("2 + 2", None)?;
        assert!(result.is_number());
        Ok(())
    }

    #[test]
    fn test_eval_with_result_string() -> Result<()> {
        let mut runtime = Runtime::new(RuntimeConfig::default())?;
        let result = runtime.eval_with_result("'hello'", None)?;
        assert!(result.is_string());
        Ok(())
    }

    #[test]
    fn test_eval_with_result_bool() -> Result<()> {
        let mut runtime = Runtime::new(RuntimeConfig::default())?;
        let result = runtime.eval_with_result("true", None)?;
        assert!(result.is_bool());
        Ok(())
    }

    #[test]
    fn test_eval_with_result_null() -> Result<()> {
        let mut runtime = Runtime::new(RuntimeConfig::default())?;
        let result = runtime.eval_with_result("null", None)?;
        assert!(result.is_null());
        Ok(())
    }

    #[test]
    fn test_eval_with_result_undefined() -> Result<()> {
        let mut runtime = Runtime::new(RuntimeConfig::default())?;
        let result = runtime.eval_with_result("undefined", None)?;
        assert!(result.is_undefined());
        Ok(())
    }

    #[test]
    fn test_eval_with_result_object() -> Result<()> {
        let mut runtime = Runtime::new(RuntimeConfig::default())?;
        let result = runtime.eval_with_result("({foo: 'bar'})", None)?;
        assert!(result.is_object());
        Ok(())
    }

    #[test]
    fn test_runtime_config_builder() -> Result<()> {
        let config = RuntimeConfigBuilder::new()
            .heap_size(64 << 20, 512 << 20)
            .enable_eval(false)
            .enable_jit(false)
            .enable_es6_proxy(true)
            .build();

        let mut runtime = Runtime::new(config)?;

        let result = runtime.eval("eval('1 + 1')", None);
        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn test_runtime_config_disable_generators() -> Result<()> {
        let config = RuntimeConfigBuilder::new().enable_generator(false).build();

        let mut runtime = Runtime::new(config)?;

        let result = runtime.eval("function* gen() { yield 1; }", None);
        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn test_runtime_config_heap_settings() -> Result<()> {
        let config = RuntimeConfigBuilder::new()
            .heap_size(16 << 20, 32 << 20)
            .build();

        let runtime = Runtime::new(config)?;
        drop(runtime);
        Ok(())
    }

    #[test]
    fn test_prepare_javascript() -> Result<()> {
        let mut runtime = Runtime::new(RuntimeConfig::default())?;

        let prepared = runtime.prepare_javascript("2 + 2", Some("calc.js"))?;

        let result = runtime.evaluate_prepared_javascript(&prepared)?;

        assert!(result.is_number());
        Ok(())
    }

    #[test]
    fn test_prepare_javascript_multiple_executions() -> Result<()> {
        let mut runtime = Runtime::new(RuntimeConfig::default())?;

        let prepared = runtime.prepare_javascript("Math.random()", None)?;

        let result1 = runtime.evaluate_prepared_javascript(&prepared)?;
        assert!(result1.is_number());

        let result2 = runtime.evaluate_prepared_javascript(&prepared)?;
        assert!(result2.is_number());

        let result3 = runtime.evaluate_prepared_javascript(&prepared)?;
        assert!(result3.is_number());
        Ok(())
    }

    #[test]
    fn test_prepare_javascript_with_string() -> Result<()> {
        let mut runtime = Runtime::new(RuntimeConfig::default())?;

        let prepared = runtime.prepare_javascript("'hello ' + 'world'", None)?;

        let result = runtime.evaluate_prepared_javascript(&prepared)?;

        assert!(result.is_string());
        Ok(())
    }

    #[test]
    fn test_prepare_javascript_syntax_error() -> Result<()> {
        let mut runtime = Runtime::new(RuntimeConfig::default())?;

        let result = runtime.prepare_javascript("this is invalid javascript", None);
        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn test_prepare_javascript_runtime_error() -> Result<()> {
        let mut runtime = Runtime::new(RuntimeConfig::default())?;

        let prepared = runtime.prepare_javascript("throw new Error('test error')", None)?;

        match runtime.evaluate_prepared_javascript(&prepared) {
            Ok(_) => panic!("Expected error but got success"),
            Err(error_msg) => assert!(error_msg.contains("test error")),
        }
        Ok(())
    }

    #[test]
    fn test_compile_and_eval_bytecode() -> Result<()> {
        let mut runtime = Runtime::new(RuntimeConfig::default())?;

        let bytecode = Runtime::compile_to_bytecode("2 + 2", Some("calc.js"))?;

        runtime.eval_bytecode(&bytecode)?;
        Ok(())
    }

    #[test]
    fn test_bytecode_roundtrip() -> Result<()> {
        let mut runtime = Runtime::new(RuntimeConfig::default())?;

        let bytecode = Runtime::compile_to_bytecode("'hello world'", None)?;

        let bytes = bytecode.as_bytes();
        let bytecode2 = CompiledBytecode::from_bytes(bytes);

        runtime.eval_bytecode(&bytecode2)?;
        // Tests multiple evaluations, nothing should crash
        runtime.eval_bytecode(&bytecode)?;
        runtime.eval_bytecode(&bytecode)?;

        Ok(())
    }

    #[test]
    fn test_bytecode_size() -> Result<()> {
        let bytecode = Runtime::compile_to_bytecode("const x = 42;", None)?;

        assert!(!bytecode.is_empty());
        assert!(bytecode.len() > 0);

        let bytes = bytecode.as_bytes();
        assert_eq!(bytes.len(), bytecode.len());
        Ok(())
    }
}
