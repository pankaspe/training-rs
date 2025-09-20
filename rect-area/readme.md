## Exercise 1: Rectangle Area Calculator

**Problem Description:**
Write a Rust program that calculates the area of a rectangle. The program should first define two variables for the rectangle’s width and height, then compute the area, and finally print the result in a clear and readable format, for example:
`The area of the rectangle is: [result] square units.`

**Rust Pill:**
In Rust, variables are immutable by default. This means that once a value is assigned, it cannot be changed. To make a variable mutable, you use the `mut` keyword. This design choice promotes safety and predictability in the code. For this first exercise, we don’t need mutable variables.

**Programmer’s Logical Thinking:**
The problem requires calculating an area. The formula for the area of a rectangle is **base × height**.

* **Decomposition:** Break the problem into smaller steps:

  * I need a place to store the width (a variable).
  * I need a place to store the height (another variable).
  * I need to perform the multiplication operation.
  * I need a place to store the result (a third variable).
  * I need to display the result to the user.

* **Abstraction:** The dimensions could be integers (e.g., 10 meters) or decimals (e.g., 10.5 meters). To be more general and precise, it’s better to use a data type that supports decimals.

* **Sequence:** The order of operations matters. I can’t calculate the area before having both the width and height values.
