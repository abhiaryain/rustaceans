// 1. `let` keyword is used to declare a variable
//      e.g. `let age = 5;` is a variable declaration statement that declares a variable named `age` and assigns/binds it the value 5.
// 2. By default, variables are immutable, which means once a value is assigned to a variable, it cannot be changed or cannot assign twice to the same variable.
//      To make a variable mutable, you need to declare it with the `mut` keyword
//          e.g. `let mut age = 5;` is a variable declaration statement that declares a mutable variable named `age` and assigns/binds it the value 5.

// Immutable variable example
fn main() {
    let age = 5;
    println!("age is {}", age);
    // age = 6; // This will give an error because age is "cannot assign twice to immutable variable `age`"
    println!("age is now {}", age);
}

// Mutable variable example
// fn main() {
//     let mut age = 5;
//     println!("age is {}", age);
//     age = 6; // This will not give an error because age is a mutable variable
//     println!("age is now {}", age);
// }
