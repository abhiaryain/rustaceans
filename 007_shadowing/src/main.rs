// Mutability vs Shadowing
// In shadowing we are creating new variable whenever we use `let`
// We can change the type of the value and reuse the same variable name.

fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is {x}");
}
