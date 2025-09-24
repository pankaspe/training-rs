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
fn calculate_total_username_length(users: &Vec<User>) -> usize {
    // We use a functional approach, which is concise and idiomatic in Rust.
    users
        .iter() // 1. Get an iterator that yields `&User`.
        .map(|user| user.username.len()) // 2. For each `&User`, map it to the length of its username (`usize`).
        .sum() // 3. Consume the iterator of `usize`s and sum them up.
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

    // We pass an immutable reference `&users` to the function.
    // `main` still owns `users`.
    let total_length = calculate_total_username_length(&users);
    println!("Total length of all usernames: {}", total_length);

    // We can pass another reference to `users` because it was never moved.
    print_usernames(&users);

    // This line compiles and runs without error, proving that `main`
    // has retained ownership of the `users` vector throughout the program.
    println!("\nReport successfully generated for {} users.", users.len());
}
