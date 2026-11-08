// ======================================================================================
//                              CHARACTER TYPE REFERENCE
// ======================================================================================
//   1. Rust’s `char` type is the language’s most primitive alphabetic type
//   2. `char` literals are enclosed in single quotes, e.g. let character = 'a'; - while string literals are enclosed in double quotes, e.g. let string = String::from("hello");
//   3. `char` type is 4 bytes in size and can represent any Unicode Scalar Value, e.g. let heart_eyed_cat = '😻';
//   4. `char` can be used to represent ASCII characters, Accented letters; Chinese, Japanese, and Korean characters; emojis; and zero-width spaces, Unicode scalar values range from U+0000 to U+D7FF and U+E000 to U+10FFFF inclusive

pub fn character() {
    let character = 'a';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("character is {}", character);
    println!("z is {}", z);
    println!("heart_eyed_cat is {}", heart_eyed_cat);
}