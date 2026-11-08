fn main() {
    single_line_comments();
    multi_line_comments();
    println!("Simple Interest: {}", simple_interest(1000, 10.0, 10))
}

// Comments are notes or explanations written in source code to improve readability and make it easier for developers to understand the code.
// Comments are ignored by the compiler and are not executed as part of the program.

// Single line comments
fn single_line_comments() {
    // Single line comments starts with two forward slashes`//`, and continues until the end of the line.
    println!("Single Line Comments"); // we can write single line comments at the end of lines containing code.
}

/* Multi line comments or block comments*/
fn multi_line_comments() {
    /*
       Multi line comment start with /* and continue until the closing */ is encountered.
    */

    println!(
        /* We can place multi line comment any where in the code unless we broke the token or literals  */
        "Multi Line Comments"
    );
}

// Documents comments used to document Rust code, such as functions, structs, enums, modules, and other items.
// Rust uses documentation comments to automatically generate HTML documentation using the rustdoc tool.

// Outer documentation comments `///``
// Inner documentation comments `//!`

/// Calculates simple interest based on the principal amount, interest rate, and time period.
///
/// # Arguments
/// * `principle` - The initial amount of money.
/// * `rate` - The annual interest rate (in percentage).
/// * `years` - The time period in years.
///
/// # Returns
/// The calculated simple interest as an `f64`.
fn simple_interest(principle: i32, rate: f64, years: i32) -> f64 {
    //! Formula: (principle * rate * years) / 100
    (principle as f64 * rate * years as f64) / 100.0
}
