// Difference between Constants and Variables
//   1. Constants are always immutable by defaults and not allowed to be mutable with `mut` keyword.
//   2. Constant values that cannot be changed once they are assigned.
//   3. To declare a constant, you use the `const` keyword instead of `let`.
//   4. `mut` keyword not allowed for constants.
//   5. Type of constant must be known at compile time and must be annotated.
//   6. We can't declare `let` variable in global scope. but we can declare constant in any scope including global scope.
//   7. Constants can only be set with constant expression, not the result of a value that could only be computed at runtime.

// Convention for constants
// 1. Use SCREAMING_SNAKE_CASE(snake_case in uppercase) for constant names.

const PI: f64 = 3.14159265359;

// let a = 10; // This will give an error because `let` cannot be used for global variables.

fn main() {
    const THREE_HOURS_IN_SECONDS: u32 = 3 * 60 * 60; // This is a constant expression, not a value that could only be computed at runtime.
    
    println!("PI is {}", PI);
    println!("Three hours in seconds is {}", THREE_HOURS_IN_SECONDS);
}
