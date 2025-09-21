/*
 * Exercise 5: The Geometric Shape Calculator
 *
 * This program calculates the area of different geometric shapes. It demonstrates
 * the power of enums to represent a type that can be one of several variants,
 * and the `match` expression to safely execute different code for each variant.
 * This solution uses a struct/enum composition pattern for greater flexibility.
 */

// import PI from the std
use std::f64::consts::PI;

// An enum to represent the different kinds of shapes we can have.
// Each variant holds the specific data needed for that shape.
enum ShapeType {
    // The Rect variant holds named fields for width and height.
    Rect { width: u32, height: u32 },
    // The Circle variant holds the radius as a 64-bit float.
    Circle { radius: f64 },
}

// A struct to represent a generic shape.
// This is a great design pattern: it holds data common to ALL shapes (like a name)
// and uses an enum (`ShapeType`) for the parts that vary. This is called composition.
struct Shape {
    name: String,
    variants: ShapeType,
}

// Implementation block for our main `Shape` struct.
impl Shape {
    // A constructor to easily create new `Shape` instances.
    fn new(name: String, variants: ShapeType) -> Shape {
        Shape { name, variants }
    }

    // A method to calculate the area. It takes an immutable reference to self.
    // The logic is delegated to the `variants` enum using a `match` expression.
    fn calculate_area(&self) -> f64 {
        // `match` is an expression, so it returns a value that we can directly return from the function.
        // It inspects `self.variants` and executes the code for the matching arm.
        match self.variants {
            // If the variant is `Rect`, we destructure it to get `width` and `height`.
            // We cast the u32 values to f64 to ensure the multiplication results in an f64.
            ShapeType::Rect { width, height } => (width as f64) * (height as f64),

            // If the variant is `Circle`, we destructure it to get `radius`.
            // The formula for a circle's area is π * r².
            ShapeType::Circle { radius } => PI * radius.powi(2),
        }
    }
}

fn main() {
    // A vector to hold our collection of different shapes.
    let shapes: Vec<Shape> = vec![
        Shape::new(
            String::from("My Rectangle"),
            ShapeType::Rect {
                width: 20,
                height: 5,
            },
        ),
        Shape::new(String::from("My Circle"), ShapeType::Circle { radius: 7.3 }),
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
}
