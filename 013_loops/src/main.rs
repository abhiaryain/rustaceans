// 1. Loops are used to execute a block of code more than once.
// 2. Rust has three kinds of loops: `loop`, `while` and `for`

// loop: useful to execute a block of code over and over again either forever or until you explicitly tell it to stop.
fn main() {
    let retry = 10;
    let mut attempt = 1;
    loop {
        println!("Trying {attempt}/{retry}");
        attempt += 1;
        if retry == attempt {
            break; // `break` exits the loop without returning a meaningful value. The loop evaluates to `()` (unit type)
        }
    } // Here, we are using loop as expression that's why we don't need semicolon`;`, However we can use semicolon`;` to make this statement, but we are not assigning loop's return value to so it don't affect.

    loop_expression();

    loop_label();
}

// 3. As we know `loop` is an expressions, we can use it with `let` statements to assign the outcome to a variable.
fn loop_expression() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // This looks confusion we have learned about statements and expression that once we add semicolon`;`` to a expression it become statement, then how `break counter * 2;` is an expression
                               // Here, semicolon`;` terminates the `break` expression within the block.
                               // `counter * 2` evaluated to 20, and `break` uses that value as loop's return value.
        }
    }; // semicolon`;` is required here is make entire `let result = loop { ... }` as statement.

    println!("The result is {result}");
}

// 4. Disambiguating with Loop Labels
//    When we have nested loops, `break` and `continue` apply to the innermost loop in which they are used.
//    We can optionally specify a loop label on a loop that you can use with `break` or `continue` to specify that that those keywords apply to the labeled instead of innermost loop.
//    Loop labels must begin with a single quote.
fn loop_label() {
    let mut count = 0;
    let remaining_result = 'counting_up: loop {
        println!("count = {count}");

        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");

            if(remaining == 9){
                break;
            }

            if(count == 2){
                break 'counting_up remaining * 2;
            }
            remaining -= 1;
        }
        count += 1;
    };
    println!("End count = {count}");
    println!("Twice of last remaining = {remaining_result}")
}
