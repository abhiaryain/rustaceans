mod scalar;

// Data Type
//  Rust is a statically typed language, which means that it must know the types of all variables at compile time.
//  Rust compiler can usually infer what type we want to use based on the value and how we use it.
//  Every value in Rust is of a certain data type

// Rust has 2 types of data types
//   1. Scalar - A scalar type represents a single value.
//       Rust has four primary scalar types:
//         1. Integers
//         2. Floating point numbers
//         3. Booleans (true or false)
//         4. Characters
//   2. Compound

fn main() {
    let age = 10; // Implicitly typed as an i32
    println!("Implicitly typed: age is {}", age);

    let age: i8 = 10; // Explicitly typed as an i8
    println!("Explicit typed: age is {}", age);

    scalar::integer::integer_value();
    scalar::integer::integer_overflow(); // This will panic at runtime
    scalar::floating_point::floating_point();
    scalar::boolean::boolean();
    scalar::character::character();
}
