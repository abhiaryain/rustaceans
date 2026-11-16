use std::collections::HashSet;

// --------------------------------------------------
// PROBLEM: Functions take ownership of String
// --------------------------------------------------

fn unique_alphabets_limitation(input: String) -> usize {
    let mut alphabets = HashSet::new();
    for character in input.chars() {
        if character >= 'a' && character <= 'z' {
            alphabets.insert(character);
        }
    }
    alphabets.len()
}

fn unique_digits_limitation(input: String) -> usize {
    let mut digits: HashSet<_> = HashSet::new();
    for character in input.chars() {
        if character >= '0' && character <= '9' {
            digits.insert(character);
        }
    }
    digits.len()
}

fn limitation() {
    let introduction = String::from("Hey! I'm a 25-year-old adult who's currently learning Rust.");

    // Ownership of `introduction` moves into the function.
    let alphabets = unique_alphabets_limitation(introduction);

    // ERROR: `introduction` was moved into the previous function and no longer valid here.
    // let digits = unique_digits_limitation(introduction);

    // ERROR: `introduction` is no longer valid here.
    // println!("Introduction: '{introduction}' contains\nUnique alphabets: {alphabets}\nUnique digits: {digits}");

    println!("Unique alphabets: {alphabets}");
}

// --------------------------------------------------
// SOLUTION 1: Clone the String
// --------------------------------------------------

// We can use `.clone()` to create an independent copy of the String and transfer ownership of that copy.
// However, cloning allocates additional memory and copies the string's contents. Avoid unnecessary cloning.
fn solution_clone() {
    let introduction = String::from("Hey! I'm a 25-year-old adult who's currently learning Rust.");
    
    let alphabets = unique_alphabets_limitation(introduction.clone());
    let digits = unique_digits_limitation(introduction.clone());

    // Original `introduction` is still valid.
    println!("Introduction: '{introduction}' contains\nUnique alphabets: {alphabets}\nUnique digits: {digits}" );
}

// --------------------------------------------------
// SOLUTION 2: Take and Return Ownership
// --------------------------------------------------

// Another solution is to return ownership from the function along with the calculated result.
// This allows the caller to regain ownership of the String.
fn unique_alphabets_solution(input: String) -> (usize, String) {
    let mut alphabets = HashSet::new();
    for character in input.chars() {
        if character >= 'a' && character <= 'z' {
            alphabets.insert(character);
        }
    }
    (alphabets.len(), input)
}

fn unique_digits_solution(input: String) -> (usize, String) {
    let mut digits = HashSet::new();
    for character in input.chars() {
        if character >= '0' && character <= '9' {
            digits.insert(character);
        }
    }
    (digits.len(), input)
}

fn solution_return_ownership() {
    let introduction = String::from("Hey! I'm a 25-year-old adult who's currently learning Rust.");

    // Ownership moves into the function.
    // The function returns both the result and ownership of the String.
    let (alphabets, introduction) = unique_alphabets_solution(introduction);

    // Ownership moves again, then comes back.
    let (digits, introduction) = unique_digits_solution(introduction);

    println!("Introduction: '{introduction}' contains\nUnique alphabets: {alphabets}\nUnique digits: {digits}");
}

// --------------------------------------------------
// THE PROBLEM WITH THESE APPROACHES
// --------------------------------------------------

// 1. Passing a String directly transfers ownership.
//    The original variable cannot be used afterward.
//
// 2. Cloning creates additional allocations and copies data.
//
// 3. Returning ownership through tuples works, but becomes
//    tedious when multiple functions need the same value.
//
// What if we want a function to access a value without
// taking ownership of it?
//
// It would be much more convenient if we could let a function
// use a value while allowing the original owner to retain it.
//
// This is where BORROWING and REFERENCES come into play.
// --------------------------------------------------

// --------------------------------------------------
// SOLUTION 3: References and Borrowing
// --------------------------------------------------

fn unique_alphabets(input: &String) -> usize {
    let mut alphabets = HashSet::new();
    for character in input.chars() {
        if character >= 'a' && character <= 'z' {
            alphabets.insert(character);
        }
    }
    alphabets.len()
}

fn unique_digits(input: &String) -> usize {
    let mut digits = HashSet::new();
    for character in input.chars() {
        if character >= '0' && character <= '9' {
            digits.insert(character);
        }
    }
    digits.len()
}

fn solution() {
    let introduction = String::from("Hey! I'm a 25-year-old adult who's currently learning Rust.");

    let alphabets = unique_alphabets(&introduction);

    let digits = unique_digits(&introduction);

    println!("Introduction: '{introduction}' contains\nUnique alphabets: {alphabets}\nUnique digits: {digits}\n");
}

fn main() {
    println!("OWNERSHIP LIMITATION");
    limitation();

    println!("\nCLONE SOLUTION");
    solution_clone();

    println!("\nRETURN OWNERSHIP SOLUTION");
    solution_return_ownership();

    println!("\nREFERENCES AND BORROWING");
    solution();
}
