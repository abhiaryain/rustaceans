// Rust is an expression-based language, meaning most things in Rust evaluate to a value.

// 1. Statements are instructions that perform some action but do not return a value.
// 2. Expressions are combinations of values, operators, and other constructs that evaluate to a single resultant value.

fn get_value() -> i32 {
    let first = 10; // Statement
    let second = 30; // Statement
    first + second // Expression
}

// 3. Since statements do not return values, we cannot assign a statement directly to a variable.
//  `let x = (let y = 6);` // panics -> expected expression, found `let` statement

// 4. Expressions can be part of other expressions or statements, and statements can contain expressions.
fn main() {
    // Here, `{}` creates a block expression.
    // `let x = 3;` is a statement inside the block.
    // `x + 1` is the final expression of the block.
    // The block evaluates to `4`, which is assigned to `y`.
    let y = {
        let x = 3; // Statement containing an expression (3)
        x + 1 // Expression (evaluates to 4)
    };

    println!("value: {}", get_value());
    println!("y: {y}")
}

// 5. Statements: Variable declarations (let), function definitions, struct definitions, enum definitions, module declarations, constant and static declarations, type aliases, and trait/implementation definitions.
// 6. Expressions: Literals, variables, arithmetic operations, function calls, macro calls, method calls, block expressions, if expressions, match expressions, loop expressions, and assignment expressions.
//    Some macros can expand into statements or expressions depending on how they are used.
// 7. An expressions used as a value does not have a trailing semicolon. Adding a semicolon to an expression turns it into a statement and discards its resulting value.

// 8. Blocks are expressions
//    A block`{}` is an expression that can contain multiple statements and an optional final expression.
//    The value of a block expression is the value of its final expression.
//    If a block has no final expression, it evaluates to `()` (unit type).

// 9. `if` expressions
//    In Rust, `if` and `else` are expressions, meaning they can return values.
//    Both branches must evaluate to compatible types.
//    (e.g. `fn main() { let number = 10; let result = if number > 5 {100} else {200} }`)

// 10. Expressions can return `()`
//    Not every expression returns a meaningful data value. Some expressions evaluate to `()` (unit type).
//    For example, calling a function that doesn't return a value explicitly evaluates to `()`.
