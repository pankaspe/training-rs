/*
 * Exercise 2: The Gatekeeper
 * Write a Rust program that checks if a person is of legal adult age.
 * The program will have a variable for the person's age and use a separate function
 * to determine if the age is 18 or older. Based on the result, it will print a
 * message indicating whether the person is allowed entry.
 */

// Defines a function named 'is_adult' to encapsulate the age-checking logic.
// It accepts one argument, 'age', which is an unsigned 32-bit integer (u32).
// It declares that it will return ('->') a boolean value (bool).
fn is_adult(age: u32) -> bool {
    // This is the core logic. It's a boolean expression that evaluates to true or false.
    // Because it is the last line of the function and has NO semicolon,
    // its result is automatically returned. This is an "implicit return".
    age >= 18
}

fn main() {
    // Declares an immutable variable to hold the age we want to check.
    // We can change this value to 17 or 25 to test both outcomes.
    let user_age: u32 = 18;

    // The 'if' statement controls the flow of the program.
    // It calls our 'is_adult' function and uses the returned boolean value.
    if is_adult(user_age) {
        // This block of code is executed if the function returns 'true'.
        // The println! macro is a statement, so it must end with a semicolon.
        println!("You can enter the venue.");
    } else {
        // This block of code is executed if the function returns 'false'.
        println!("Sorry, you cannot enter the venue.");
    }
}
