// 1. `fn` keyword is used to declare/define a function, followed by the function name, a set of parentheses`()` and curly brackets`{}` define the beginning and end of the function body.
// 2. We can define multiple parameters inside the parentheses of a function, separated by commas`,`. (e.g. `fn print_user_details(name: String, age: i8) { ... }`)
// 3. The data type of every function parameter must be annotated. This helps the compiler understand what type of value the function expects and provide more useful error messages.
// 4. Rust doesn't care where we define a function in our code. It just needs to be within a scope accessible to the caller. We can define a function before or after the caller.
// 5. A functions bodies are made up of a series of statements and optionally ending in an expression.

fn print_user_details(name: String, age: i8) {
    // function definition
    println!("Hi {name}, Your are {age} years old.");
}

// 6. We can call a function by writing its name followed by a set of parentheses`()` and passing the required arguments inside the parentheses. (e.g. `print_user_details(name, age);`)
// 7. Parameters are the variables defined in a function signature, while arguments are the actual/concrete values passed when calling a function.

fn main() {
    let name = String::from("Abhishek");
    let age = 25;
    print_user_details(name, age); // function call

    let sum = add(50, 50);
    println!("sum: {sum}");
}

// 8. A function can return a value. We must declare the return type of a function using an arrow `->` followed by the return data type in the function signature (e.g. `fn add(a: u16, b: u16) -> u32 { (a + b).into() }`).
// 9. In Rust, functions implicitly return the value of their final expression without needing the `return` keyword, but we can explicitly return a value using the `return` keyword followed by the value.
// 10. Every function implicitly returns `()` (also known as the unit type) if it doesn't have a return value or a final expression that returns a value.

fn add(a: u16, b: u16) -> u32 {
    (a + b).into()
}
