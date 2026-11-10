use std::io;

// 1. In Rust, an `if` expression is used to execute a block of code conditionally.
// 2. Rust does not implicitly convert other data types into Boolean values. The condition must always evaluate to a Boolean value (true or false).
// 3. Rust support `if age >= 18 { }` and parentheses around `if` condition `if (age >= 18) {}` both are valid and behave identically. However, Rust warns `warning: unnecessary parentheses around `if` condition`.
// 4. How Rust Evaluates `if`, `else if`, and `else` in sequence.
//    Rust checks the first `if` condition.
//    If the condition evaluates to `true`, its corresponding code block executes.
//    Once a matching condition is found, Rust skips all remaining `else if` and `else` blocks.
//    If the first condition is `false`, Rust checks the next `else if` condition, if one exists.
//    If none of the conditions evaluates to `true`, the `else` block executes, if provided.
//    If there is no else block, Rust simply continues executing the next instruction.
// 5. `if` expressions are sometimes called `arms`

fn main() {
    let mut age = String::new();
    io::stdin()
        .read_line(&mut age)
        .expect("Please input a valid age.");

    let age: u8 = age.trim().parse().expect("Please input a valid age.");

    if age >= 90 {
        println!("Congratulations, you are eligible to vote, and we provide vehicle service to reach the pooling booth.")
    } else if age >= 18 {
        println!("Congratulations, you are eligible to vote.");
    } else {
        println!("Sorry, you are not eligible to vote.");
    }

    let citizen = get_citizen(age);
    println!("You are {citizen}.");
}

// 6. As we know `if`, `else if` and `else` are expressions, we can use it with `let` statements to assign the outcome to a variable.
//    The value of the whole `if` expression depends on which block of code executes. This means the values that have the potential to be results from each arm of the if must be the same type.
fn get_citizen(age: u8) -> String {
    let citizen = if age >= 90 {
        "senior citizen"
    } else if age >= 18 {
        "adult"
    } else {
        "minor"
    };
    return citizen.to_string();
}
