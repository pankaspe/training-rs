/*
 * Exercise 1: Rectangle Area Calculator
 * Write a Rust program that calculates the area of a rectangle.
 * The program should first define two variables for the rectangle’s width and height, then compute the area,
 * and finally print the result in a clear and readable format, for example:
 * `The area of the rectangle is: [result] square units.`
 */

fn main() {
    // --- First Rectangle ---

    // Define an immutable variable 'width' and assign the value 10.
    // We choose an unsigned integer (u32) because dimensions can't be negative.
    let width: u32 = 10;

    // Define an immutable variable 'height' and assign the value 5.
    let height: u32 = 5;

    // Calculate the area by multiplying width and height.
    // The result is stored in a new immutable variable 'area'.
    let area = width * height;

    // Print the calculated area to the console using a formatted string.
    // The `{}` is a placeholder that will be replaced by the value of the 'area' variable.
    println!("The area of the first rectangle is: {} square units.", area);

    // --- Second Rectangle (Demonstrating Shadowing) ---

    // "Shadow" the previous 'width' variable by declaring a new one with the same name.
    // The old 'width' with value 10 is no longer accessible from this point forward.
    let width: u32 = 30;

    // Shadow the 'height' variable as well with a new value.
    let height: u32 = 7;

    // Calculate the area for the new dimensions.
    // This creates a new 'area' variable, shadowing the previous one.
    let area = width * height;

    // Print the new calculated area.
    println!(
        "The area of the second rectangle is: {} square units.",
        area
    );
}
