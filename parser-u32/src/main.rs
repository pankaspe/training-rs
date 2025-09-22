/*
 * Exercise 7: The Demanding Number Parser
 *
 * This program demonstrates robust error handling using the `Result<T, E>` enum.
 * The function `parse_positive_number` validates a string and returns either
 * `Ok(number)` on success or a descriptive `Err(message)` on failure.
 */

/// Parses a string slice into a u32, applying strict validation rules.
fn parse_positive_number(s: &str) -> Result<u32, String> {
    // GUARD CLAUSE 1: Fail fast if the input string is empty.
    // Using an early `return` makes the function's logic flatter and easier to read.
    if s.is_empty() {
        return Err(String::from("Invalid input: string is empty."));
    }

    // GUARD CLAUSE 2: Check if all characters are digits.
    // `.chars()` creates an iterator over the characters.
    // `.all(...)` checks if every item from the iterator matches a condition.
    // `c.is_digit(10)` checks if a character is a numeric digit in base 10.
    if !s.chars().all(|c| c.is_digit(10)) {
        return Err(String::from(
            "Invalid input: contains non-numeric characters.",
        ));
    }

    // HAPPY PATH: If all guards have passed, the string is valid.
    // We can now attempt to parse it. `.parse()` itself returns a Result,
    // because the number might be too large to fit in a u32.
    match s.parse::<u32>() {
        Ok(number) => Ok(number),
        // We use a wildcard `_` because we don't care about the specific parse error details here.
        Err(_) => Err(String::from(
            "Invalid input: the number is too large for a u32.",
        )),
    }
}

fn main() {
    // A vector of test cases to validate our function.
    let test_cases = vec!["123", "", "abc", "45a67", "0", "99999999999999999999"];

    // Iterate over the test cases.
    for element in test_cases {
        println!("Parsing '{}'...", element);

        // Call our parsing function.
        let result = parse_positive_number(element);

        // Match on the result to perform an ACTION (printing).
        // Both arms now execute a `println!`, so they both have the same type: `()`.
        // This resolves the compilation error.
        match result {
            Ok(value) => println!("  Success! Parsed number: {}", value),
            Err(error_message) => println!("  Error: {}", error_message),
        }
    }
}
