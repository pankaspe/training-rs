/*
 * Exercise 10: The Report Generator
 *
 * This exercise demonstrates the core concepts of Ownership and Borrowing.
 * Functions `calculate_total_username_length` and `print_usernames` borrow the
 * user data via immutable references (`&Vec<User>`), allowing `main` to retain
 * ownership and use the data after the function calls.
 */

struct User {
    username: String,
}

// Using an `impl` block to associate methods with the `User` struct is a great practice.
impl User {
    fn new(username: String) -> User {
        User { username }
    }
}

/// Calculates the combined length of all usernames in a list.
/// It BORROWS the vector of users via an immutable reference `&Vec<User>`.
fn calculate_total_username_length(users: &Vec<User>) -> Result<usize, String> {
    // check if vec is empty
    if users.is_empty() {
        return Err(String::from(
            "Impossibile calcolare: il vettore di utenti è vuoto.",
        ));
    }

    // We use a functional approach, which is concise and idiomatic in Rust.
    let total_length = users
        .iter() // 1. Get an iterator that yields `&User`.
        .map(|user| user.username.len()) // 2. For each `&User`, map it to the length of its username (`usize`).
        .sum(); // 3. Consume the iterator of `usize`s and sum them up.

    Ok(total_length)
}

/// Prints all usernames to the console.
/// This function also BORROWS the vector, ensuring `main` keeps ownership.
fn print_usernames(users: &Vec<User>) {
    println!("--- User Report ---");
    // We can iterate directly over a reference to a vector.
    // Inside the loop, `user` will be of type `&User`.
    for user in users {
        println!("- {}", user.username);
    }
    println!("-------------------");
}

fn main() {
    // `users` is the OWNER of the vector and the `User` data within it.
    let users: Vec<User> = vec![
        User::new(String::from("alice_in_wonderland")),
        User::new(String::from("bob_the_builder")),
        User::new(String::from("charlie_chaplin")),
    ];

    // We create a second vector to test the error-handling path of our function.
    let empty_users: Vec<User> = vec![];

    println!("--- Attempting to generate report for populated user list ---");

    // Call the function and use `match` to handle the `Result` it returns.
    // This forces us to handle both the success (`Ok`) and failure (`Err`) cases.
    match calculate_total_username_length(&users) {
        // --- SUCCESS PATH ---
        // If the calculation succeeds, the `Ok` variant is returned,
        // and its content is destructured into the `length` variable.
        Ok(length) => {
            println!("Length calculation was successful.");

            // ACTION 1: Call the function to print the user report.
            // `print_usernames` is called for its "side effect" (printing to the console).
            // It returns the unit type `()`, so we don't assign its result to a variable.
            print_usernames(&users);

            // ACTION 2: Print the summary line with the calculated result.
            println!("\nThe total length of all usernames is: {}", length);

            // This final message confirms that the main logic branch has completed.
            // This line still works because `main` never gave away ownership of `users`.
            println!("\nReport successfully generated for {} users.", users.len());
        }

        // --- FAILURE PATH ---
        // If the calculation fails, the `Err` variant is returned,
        // and its content (the error message String) is bound to the `e` variable.
        Err(e) => {
            // We handle the error gracefully by printing the descriptive message.
            println!("An error occurred while generating the report: {}", e);
        }
    }
}
