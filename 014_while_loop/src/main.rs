// 1. Unlike `loop`, a `while` loop does not support returning a value through `break`. A `break` statement in a `while` loop simply terminates the loop without producing a value.

fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    print_array()
}

// Looping through a collection using while
fn print_array() {
    let statuses = ["pending", "accepted", "rejected", "completed", "cancelled"];
    let length = statuses.len();
    let mut idx = 0;

    while idx < length {
        println!("{}", statuses[idx]);
        idx += 1;
    }
}
