// ======================================================================================
//                              RUST INTEGER TYPE REFERENCE
// ======================================================================================
//  Length          | Signed | Signed Range (2^n)      | Unsigned | Unsigned Range (2^n)
// -----------------+--------+-------------------------+----------+----------------------
//  8-bit           | i8     | -2^7   to (2^7 - 1)     | u8       | 0 to (2^8 - 1)
//  16-bit          | i16    | -2^15  to (2^15 - 1)    | u16      | 0 to (2^16 - 1)
//  32-bit          | i32    | -2^31  to (2^31 - 1)    | u32      | 0 to (2^32 - 1)
//  64-bit          | i64    | -2^63  to (2^63 - 1)    | u64      | 0 to (2^64 - 1)
//  128-bit         | i128   | -2^127 to (2^127 - 1)   | u128     | 0 to (2^128 - 1)
//  Arch-dependent  | isize  | -2^(n-1) to (2^(n-1)-1) | usize    | 0 to (2^n - 1)
// ======================================================================================
// Note: For arch-dependent types, 'n' equals 32 on a 32-bit CPU and 64 on a 64-bit CPU.
//       Signed numbers are represented using two's complement.
//
//       Rust's default integer type is `i32`,
//       Sometime you must use `isize` or `usize` is when indexing some sort of collection
//
//       Integer division truncates toward zero to the nearest integer.
//       i.e. `let a =  10 / 3;` will be `3` instead of `3.3333333333333335`
//            `let a = -10 / 3;` will be `-3` instead of `-3.3333333333333335`
// ======================================================================================

// ======================================================================================
//                           RUST NUMBER LITERAL REPRESENTATIONS
// ======================================================================================
//  Literal Format  | Example Usage | Description / Notes
// -----------------+---------------+----------------------------------------------------
//  Decimal         | 98_222        | Standard base-10 (underscores added for clarity)
//  Hex             | 0xff          | Base-16 hexadecimal notation (starts with 0x)
//  Octal           | 0o77          | Base-8 octal notation (starts with 0o)
//  Binary          | 0b1111_0000   | Base-2 binary notation (starts with 0b)
//  Byte (u8 only)  | b'A'          | ASCII character represented directly as a u8 byte
// ======================================================================================
// Note: Number literals can also use _ as a visual separator to make the number easier to read
//       (e.g. 98_222 is the same as 98222).
// ======================================================================================

// ======================================================================================
//                           INTEGER OVERFLOW AND UNDERFLOW
// ======================================================================================
// Note: When we are compiling and running in debug mode,
//       Rust includes checks for integer overflow that cause your program to panic at runtime.
//
//       When we are compiling and running in release mode,
//       Rust does not include checks for integer overflow that cause panics.
//       Instead, if overflow occurs, Rust performs two’s complement wrapping.
//       i.e. `let count:u8 = 256` will be `0` instead of causing an overflow panic.

pub fn integer_value() {
    let items: u8 = 25;
    println!("items is {}", items);
}

fn add(a: u8, b: u8) -> u8 {
    a + b
}

pub fn integer_overflow() {
    let sum: u8 = add(255, 1); // This will panic at runtime in debug mode and two’s complement wrapping in release mode
    println!("sum is {}", sum);
}
