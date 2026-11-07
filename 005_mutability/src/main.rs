// Mutable variable example
fn main() {
    let mut age = 5;
    println!("age is {}", age);
    age = 6; // This will not give an error because age is a mutable variable
    println!("age is now {}", age);
}
