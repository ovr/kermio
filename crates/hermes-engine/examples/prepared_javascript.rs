use hermes_engine::{Runtime, RuntimeConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut runtime = Runtime::new(RuntimeConfig::default())?;

    // Prepare JavaScript code once
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

    let result = runtime.evaluate_prepared_javascript(&prepared)?;
    println!("Executed: result is_number = {}", result.is_number());

    Ok(())
}
