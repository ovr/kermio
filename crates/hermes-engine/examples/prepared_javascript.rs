use hermes_engine::{Runtime, RuntimeConfig};

fn main() -> Result<(), String> {
    let mut runtime = Runtime::new(RuntimeConfig::default())?;

    // Prepare JavaScript code once
    println!("Preparing JavaScript code...");
    let prepared = runtime.prepare_javascript(
        r#"
        function fibonacci(n) {
            if (n <= 1) return n;
            return fibonacci(n - 1) + fibonacci(n - 2);
        }
        fibonacci(10);
        "#,
        Some("fibonacci.js"),
    )?;

    println!("Executing prepared code 3 times...");

    // Execute the same prepared code multiple times efficiently
    for i in 1..=3 {
        let result = runtime.evaluate_prepared_javascript(&prepared)?;
        println!("Execution {}: result is_number = {}", i, result.is_number());
    }

    // Demonstrate the difference with regular eval
    println!("\nComparing with regular eval:");

    // Regular eval - parses every time
    for i in 1..=3 {
        let result = runtime.eval_with_result(
            r#"
            function fibonacci(n) {
                if (n <= 1) return n;
                return fibonacci(n - 1) + fibonacci(n - 2);
            }
            fibonacci(10);
            "#,
            Some("fibonacci.js"),
        )?;
        println!(
            "Regular eval {}: result is_number = {}",
            i,
            result.is_number()
        );
    }

    println!("\nWith prepareJavaScript:");
    println!("✓ Code is parsed and optimized once");
    println!("✓ Subsequent executions are faster");
    println!("✓ Useful for repeated execution of the same code");

    Ok(())
}
