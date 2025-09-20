/*
 * Exercise 3: The Structured Surveyor
 *
 * This exercise evolves the area calculator by introducing structs.
 * Instead of separate variables for width and height, we define a `Rect` struct
 * to group them. We then attach behavior (like calculating the area) directly
 * to the struct using an implementation block (`impl`).
 */

// The `derive` attribute tells the Rust compiler to automatically generate
// some code for us. `Debug` implements the Debug trait, which allows us
// to print the struct's contents for debugging purposes using the `{:?}` formatter.
#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}

// `impl` is short for "implementation". This block contains all the functions
// and methods that are associated with the `Rect` struct.
impl Rect {
    // This is an "associated function" because it doesn't take `self` as a parameter.
    // It's often used as a constructor. It is called using `::` syntax, e.g., `Rect::new()`.
    // It creates and returns a new instance of the `Rect` struct.
    fn new(width: u32, height: u32) -> Rect {
        Rect {
            // This is "Field Init Shorthand". When the variable names match the struct
            // field names, you can just write the name instead of `width: width`.
            // It's a neat, idiomatic shortcut.
            width,
            height,
        }
    }

    // This is a "method" because its first parameter is `&self`.
    // `&self` is an immutable reference to the specific instance of the struct
    // the method is being called on (e.g., `rect_a`).
    // It "borrows" the instance, so we can read its data without taking ownership.
    fn area(&self) -> u32 {
        // We access the fields of the instance using `self.field_name`.
        self.width * self.height
    }
}

// The main entry point of the program.
fn main() {
    // Create an instance of our Rect struct by calling the `new` associated function.
    let rect_a = Rect::new(30, 5);

    // Print the entire struct for debugging. The `{:?}` specifier
    // tells println! to use the `Debug` format.
    println!("The struct of rect_a is: {:?}", rect_a);

    // Call the `area` method on our `rect_a` instance.
    // The result is used directly in the formatted string.
    println!(
        "The area of rectangle a is: {} square units.",
        rect_a.area() // `rect_a` is the `&self` inside the `area` method
    );

    // Create a second, independent instance of Rect.
    let rect_b = Rect::new(12, 6);
    println!("The struct of rect_b is: {:?}", rect_b);

    println!(
        "The area of rectangle b is: {} square units.",
        rect_b.area()
    );
}
