# Training Rust: From Logical Thinking to Idiomatic Code

[![Rust Version](https://img.shields.io/badge/rust-2024-orange.svg)](https://www.rust-lang.org/)
[![License: MIT/Apache-2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue)](https://opensource.org/licenses/MIT)

A curated collection of exercises designed to build a strong foundation in Rust programming and cultivate a robust problem-solving mindset.

## Project Goal

This repository is a progressive learning path to master **Rust** by focusing on two key areas:
1.  **Logical Thinking**: Breaking down problems, structuring data, and building clean, efficient algorithms.
2.  **Idiomatic Rust**: Writing code that is not just correct, but also leverages Rust's unique features like ownership, borrowing, enums, and pattern matching in a natural and effective way.

The goal is not just to write code, but to improve the way you approach and solve problems as a programmer.

## Who is this for?

This project is for anyone with a **basic understanding of how to compile and run a Rust program**.
*   **Beginners** can use it to practice core concepts after reading the official book.
*   **Intermediates** can sharpen their reasoning and learn more idiomatic patterns.
*   **Experienced developers** from other languages can use it to quickly grasp the "Rust way" of doing things.

If you are completely new to Rust, it is highly recommended to start with the [official Rust book](https://doc.rust-lang.org/book/).

## How to Use This Repository

This project is structured as a **Cargo Workspace**. This means that each exercise is a separate, self-contained Rust package, allowing you to focus on one problem at a time.

### Prerequisites
-   You must have the Rust toolchain installed. If not, get it from [rustup.rs](https://rustup.rs/).
-   You need `git` to clone the repository.

### Steps
1.  **Clone the repository:**
    ```bash
    git clone https://github.com/your-username/training-rs.git
    ```
2.  **Navigate into the project directory:**
    ```bash
    cd training-rs
    ```
3.  **Run a specific exercise:**
    Use the `cargo run -p <package_name>` command to compile and run a single exercise. The package names are listed in the table below.

    For example, to run the second exercise:
    ```bash
    cargo run -p the-gatekeeper
    ```
4.  **Check your code (optional but recommended):**
    Before running, you can use `cargo check` for a quick compilation check or `cargo clippy` for powerful, idiomatic suggestions.
    ```bash
    # Check if the third exercise compiles
    cargo check -p the-structured-surveyor

    # Get style and correctness suggestions for the fourth exercise
    cargo clippy -p multiple-area-calculator
    ```

## The Learning Path

Each exercise builds upon the concepts introduced in the previous one.

| # | Exercise | Package Name | Key Concepts Introduced |
|:-:|:---|:---|:---|
| 1 | [Rectangle Area Calculator][ex1] | `rect-area` | `let`, variables, basic numeric types, `println!`. |
| 2 | [The Gatekeeper][ex2] | `the-gatekeeper` | Functions, parameters, return values (`bool`), `if/else` control flow. |
| 3 | [The Structured Surveyor][ex3] | `the-structured-surveyor` | `struct`, `impl`, methods (`&self`), associated functions (`new`). |
| 4 | [The Multiple Area Calculator][ex4] | `multiple-area-calculator` | `Vec<T>`, `for` loops, borrowing (`&`), `mut`, `String`. |
| 5 | [The Geometric Shapes Calculator][ex5] | `geometric-shapes-calculator` | `enum`, `match` expressions, data composition, floating-point numbers (`f64`). |
| 6 | [The Safe Divider][ex6] | `the-safe-divider` | `Option<T>` (`Some`, `None`), safe error handling, tuples. |
| 7 | [The Demanding Number Parser][ex7] | `parser-u32` | `Result<T, E>` (`Ok`, `Err`), `&str`, string methods (`.chars`, `.all`, `.parse`). |
| 8 | [The Functional Surveyor][ex8] | `the-functional-surveyor` | Iterators (`.iter()`), closures, `.filter()`, `.map()`, `.sum()`, `.collect()`. |

<!-- Internal Links to Directories (assuming this structure) -->
[ex1]: ./rect-area
[ex2]: ./the-gatekeeper
[ex3]: ./the-structured-surveyor
[ex4]: ./multiple-area-calculator
[ex5]: ./geometric-shapes-calculator
[ex6]: ./the-safe-divider
[ex7]: ./parser-u32
[ex8]: ./functional-surveyor
