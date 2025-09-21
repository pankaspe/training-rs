/*
 * Exercise 6: The Safe Divider
 *
 * This exercise introduces Rust's primary mechanism for handling values that
 * might be absent: the `Option<T>` enum. We create a division function that
 * returns `Some(result)` on success and `None` on failure (division by zero),
 * forcing the calling code to handle both possibilities safely.
 */

// A function that safely divides two f64 numbers.
// It returns an `Option<f64>` to explicitly handle the case of failure.
fn safe_divider(numerator: f64, denominator: f64) -> Option<f64> {
    // Check if the denominator is zero.
    if denominator != 0.0 {
        // If it's not zero, perform the division and wrap the result in `Some`.
        // This indicates a successful computation.
        Some(numerator / denominator)
    } else {
        // If the denominator is zero, return `None`.
        // This indicates that the operation could not be completed.
        None
    }
}

fn main() {
    // --- Test Case 1: Using a Tuple for input ---

    // A tuple is a simple, fixed-size collection of values of different types.
    // Here we use it to group the numerator and denominator.
    let division_inputs: (f64, f64) = (18.0, 9.0);

    println!("First test with tuple inputs: {:?}", division_inputs);

    // Call the function with values from the tuple, accessed by index (.0, .1).
    let result1 = safe_divider(division_inputs.0, division_inputs.1);

    // Use `match` to handle the `Option` returned by the function.
    // `match` forces us to cover every possible variant of the enum.
    match result1 {
        // If the result is `Some`, the value is extracted into the variable `v`.
        Some(v) => println!("Result: {}", v),
        // If the result is `None`, we handle the error case gracefully.
        None => println!("Error: Cannot divide by zero!"),
    }

    // --- Test Case 2: Division by zero ---

    let numerator: f64 = 16.0;
    let denominator: f64 = 0.0;

    println!("\nSecond test with inputs: {} / {}", numerator, denominator);

    let result2 = safe_divider(numerator, denominator);

    match result2 {
        Some(v) => println!("Result: {}", v),
        None => println!("Error: Cannot divide by zero!"),
    }
}
