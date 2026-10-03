// Difference between Constants and Variables
// Constants are always immutable by defaults.
// Constant values that cannot be changed once they are assigned.
// To declare a constant, you use the `const` keyword instead of `let`.
// `mut` keyword not allowed for constants.
// Type of constant must be known at compile time.
// We can't declare `let` variable in global scope. but we can declare constant in global scope.
// Constants can only be set with constant expression, not the result of a value that could only be computed at runtime.
// const THREE_HOURS_IN_SECONDS: u32 = 3 * 60 * 60;

// Convention for constants is to user all uppercase letters and underscores to separate words.

const PI: f64 = 3.14159265359;

let a = 10;

fn main() {
    println!("Hello, world! {}" ,PI);
}
