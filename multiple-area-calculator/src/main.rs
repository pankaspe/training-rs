/*
 * Exercise 4: The Multiple Area Calculator
 *
 * This program calculates the total area of a collection of rectangles.
 * It uses a `Vec<Rect>` to store multiple `Rect` instances and a `for` loop
 * to iterate over them, summing up their individual areas.
 */

// Added `Debug` trait for easy printing of the struct.
#[derive(Debug)]
struct Rect {
    // A String to hold the name of the rectangle. `String` is an owned, heap-allocated string type.
    name: String,
    width: u32,
    height: u32,
}

impl Rect {
    // The constructor now also takes a `String` for the name.
    fn new(name: String, width: u32, height: u32) -> Rect {
        Rect {
            name,
            width,
            height,
        }
    }

    // The area method remains the same, calculating the area from the struct's own data.
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    // A `Vec<Rect>` is a growable list of `Rect` structs.
    // We use the `vec!` macro to initialize it with three rectangles.
    let shapes_collection: Vec<Rect> = vec![
        // `String::from()` creates an owned `String` from a string literal.
        Rect::new(String::from("First rect"), 20, 5),
        Rect::new(String::from("Second rect"), 14, 7),
        Rect::new(String::from("Third rect"), 18, 3),
    ];

    // We declare a mutable variable `total_area` to act as an accumulator.
    // It must be mutable because its value will change inside the loop.
    let mut total_area: u32 = 0;

    // A `for` loop to iterate over the vector.
    // `&shapes_collection` creates an iterator that borrows each element immutably.
    // This allows us to read each `rect` without taking ownership of it.
    for rect in &shapes_collection {
        // Inside the loop, we print the area of the current rectangle.
        println!(
            "The area of '{}' is: {} square units.", // Used single quotes for clarity
            rect.name,
            rect.area()
        );

        // The `+=` operator adds the area of the current rectangle to our accumulator.
        total_area += rect.area();
    }

    // After the loop finishes, we print the final aggregated result.
    // We pass `total_area` directly. Primitive types like u32 are `Copy`,
    // so there's no need to pass a reference (`&`) to println!.
    println!(
        "\nTotal area of all rectangles is: {} square units.",
        total_area
    );
}
