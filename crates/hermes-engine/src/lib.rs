#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// Include the bindgen-generated bindings
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

// Re-export jsi-rs as the jsi module for JSI API access
pub use jsi_rs as jsi;

pub mod runtime;
pub use runtime::Runtime;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_runtime_creation() {
        let runtime = Runtime::new().expect("Failed to create runtime");
        drop(runtime);
    }

    #[test]
    fn test_eval_simple() {
        let mut runtime = Runtime::new().expect("Failed to create runtime");
        runtime.eval("2 + 2", None).expect("Failed to evaluate");
    }

    #[test]
    fn test_eval_error() {
        let mut runtime = Runtime::new().expect("Failed to create runtime");
        let result = runtime.eval("throw new Error('test error')", None);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("test error"));
    }
}
