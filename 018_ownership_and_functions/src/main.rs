// Ownership & Functions
//    The mechanics of passing a value to a function are similar to those when assigning a value to a variable. Passing a variable to a function will move or copy, just as assignment does.

fn main() {
    let greet = String::from("Hello World"); // `greet` comes into scope

    greeting(greet); // value of `greet` moves into the function `greeting`
    // so `greet` no longer valid here

    let age: i32 = 25; // `age` comes into scope

    print_age(age); // Because i32 implements the Copy trait,
    // `age` does NOT move into the function,
    // so it's okay to use `age` afterward.
} // Here, `age` goes out of scope, then `greet`. However, because the value of `greet` was moved, nothing special happens.

fn greeting(greet: String) {
    // `greet` comes into scope
    println!("{greet}");
} // Here, `greet` goes out of scope and `drop` is called. The backing memory is freed.

fn print_age(age: i32) {
    // `age` comes into scope
    println!("{age}");
} // Here, `age` goes out of scope. Nothing special happens.
