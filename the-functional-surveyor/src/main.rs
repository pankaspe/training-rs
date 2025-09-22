/*
 * Exercise 8: The Functional Surveyor
 *
 * This program demonstrates a functional approach to data processing in Rust
 * using iterator chains. Instead of a manual `for` loop, we use methods like
 * `.filter()`, `.map()`, and `.sum()` to create a declarative data pipeline
 * to calculate the total area of only the circles in a collection.
 */

// import PI from the std
use std::f64::consts::PI;

// Enum for shape-specific data.
enum ShapeType {
    Rect { width: u32, height: u32 },
    Circle { radius: f64 },
}

// Struct for common shape data, using composition.
struct Shape {
    name: String,
    variants: ShapeType,
}

impl Shape {
    fn new(name: String, variants: ShapeType) -> Shape {
        Shape { name, variants }
    }

    fn calculate_area(&self) -> f64 {
        match self.variants {
            ShapeType::Rect { width, height } => (width as f64) * (height as f64),
            ShapeType::Circle { radius } => PI * radius.powi(2),
        }
    }
}

fn main() {
    let shapes: Vec<Shape> = vec![
        Shape::new(
            String::from("My 1° Rect"),
            ShapeType::Rect {
                width: 20,
                height: 5,
            },
        ),
        Shape::new(
            String::from("My 1° Circle"),
            ShapeType::Circle { radius: 7.3 },
        ),
        Shape::new(
            String::from("My 2° Rect"),
            ShapeType::Rect {
                width: 10,
                height: 3,
            },
        ),
        Shape::new(
            String::from("My 2° Circle"),
            ShapeType::Circle { radius: 9.7 },
        ),
        Shape::new(
            String::from("My 3° Circle"),
            ShapeType::Circle { radius: 5.0 },
        ),
    ];

    // We iterate over the vector by borrowing it.
    for element in &shapes {
        // For each shape, we print its name and its calculated area.
        // The `{:.2}` format specifier rounds the float to two decimal places.
        // A semicolon is required at the end of the println! statement.
        println!(
            "The area of the shape '{}' is: {:.2}",
            element.name,
            element.calculate_area()
        );
    }

    println!("-------------------------------");

    // Get only circles names
    let circle_names: Vec<String> = shapes
        .iter() // 1. Create an iterator that yields `&Shape`.
        .filter(|shape| {
            // 2. The filter closure. It receives a `&Shape`.
            // We use `matches!` macro, a clean way to check if an enum matches a variant
            // without needing a full `match` block. This is the idiomatic solution.
            // This closure returns `true` if the shape is a Circle, `false` otherwise.
            matches!(shape.variants, ShapeType::Circle { .. })
        })
        .map(|shape| shape.name.clone()) // Transform each &Shape into new CLONED string
        .collect(); // Consuma the iterator

    // Now `circle_names` is a true vector
    println!("The names of the all circles are: {:?}", circle_names);

    println!("-------------------------------");

    let total_circle_area: f64 = shapes
        .iter() // 1. Create an iterator that yields `&Shape`.
        .filter(|shape| {
            // 2. The filter closure. It receives a `&Shape`.
            // We use `matches!` macro, a clean way to check if an enum matches a variant
            // without needing a full `match` block. This is the idiomatic solution.
            // This closure returns `true` if the shape is a Circle, `false` otherwise.
            matches!(shape.variants, ShapeType::Circle { .. })
        })
        .map(|shape| {
            // 3. The map closure. It receives only `&Shape`s that are Circles.
            // We call `calculate_area` on the shape. Note the ABSENCE of a semicolon.
            // This makes it an expression, and its value (`f64`) is returned,
            // creating a new iterator that yields `f64` values.
            shape.calculate_area()
        })
        .sum(); // 4. The sum consumer. It takes the iterator of `f64`s and adds them all up.

    // The `{:.2}` format specifier rounds the float to two decimal places.
    println!("The total area of all circles is: {:.2}", total_circle_area);
}
