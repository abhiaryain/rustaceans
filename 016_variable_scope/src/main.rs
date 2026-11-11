// Variable Scope
//    Scope refers to the range withing a program for which an item is valid and can be accessed.
//    In Rust, a variable becomes valid from the point at which it is declared and remains valid until the end of its current scope.
//    Generally, a variable's scope is defined by the block `{}` in which it is declared. When execution reaches the end of that block, the variable goes out of scope and is no longer accessible.
//    A variable declared inside an inner scope cannot be accessed outside that scope.

fn main() {
    {
        // `age` is not valid here, since it's not yet declared
        let age = 25; // `age` is valid from this point forward

        println!("age: {age}");

        // do stuff with `age`
    } // this scope is now over, and `age` is no longer valid
}
