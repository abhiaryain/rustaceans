// Difference between Mutability and Shadowing
//   1. In shadowing we are creating new variable with the same name withing the same scope using `let` keyword.
//   2. Shadowing creates a fresh variable with the same name, allowing you to change its type. Mutability, on the other hand, allows you to change the value of an existing variable, but its type must remain the same.
//   3. Shadowing allocates new memory for the variable, while mutability reuses/overwrites the existing memory.
//   4. Scope Impact: Shadowed variable can be isolated inside {} block, while mutable variable changes persist across the entire scope.

fn main() {
    // 1. Initial binding: The variable starts as a string literal
    let spaces = "   ";
    println!("1. Original spaces string: '{}'", spaces);

    // 2. Shadowing to change TYPE and VALUE:
    // We count the spaces. The type changes from &str to usize.
    let spaces = spaces.len();
    println!("2. Shadowed to length (numeric): {}", spaces);

    // 3. Shadowing to change VALUE (same type):
    // We multiply the number. The type remains usize.
    let spaces = spaces * 2;
    println!("3. Shadowed with math operation: {}", spaces);

    // 4. Shadowing inside a nested BLOCK scope:
    {
        // This variable only exists inside these curly braces {}
        let spaces = "Nested Scope Text";
        println!("4. Inside nested block scope: '{}'", spaces);
    } // The nested 'spaces' is dropped here

    // 5. Returning to the outer scope:
    // The value from step 3 is restored because the block scope ended.
    println!("5. Back in outer scope: {}", spaces);
}
