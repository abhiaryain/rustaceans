// The Rust Programming Language
//   Rust is a statically typed systems programming language that is memory-safe, thread-safe, and fast.
//     Statically typed systems means that it must know the types of all variables at compile time,
//     Rust compiler can usually infer what type we want to use based on the value and how we use it
//   Rust is an ahead-of-time compiled language, meaning you can compile a program and give the executable to someone else, and they can run it even without having Rust installed.
//   Rust source code files use the `.rs` extension

// Why Rust?
//   1. Performance: Rust is blazingly fast and memory-efficient: with no runtime or garbage collector, it can power performance-critical services, run on embedded devices, and easily integrate with other languages.
//   2. Reliability: Rust’s rich type system and ownership model guarantee memory-safety and thread-safety — enabling you to eliminate many classes of bugs at compile-time.
//   3. Productivity: Rust has great documentation, a friendly compiler with useful error messages, and top-notch tooling — an integrated package manager and build tool, smart multi-editor support with auto-completion and type inspection, an auto-formatter, and more.

// Install Rust
//   Run `curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh`

// Update Rust
//   Run `rustup update`

// Uninstall Rust
//   Run `rustup self uninstall`

// Check Rust version
//   Run `rustc --version`

// Rust offline documentation
//   Run `rust doc --book` to open Rust book offline.

// Rust tools
//  1. rustc: The Rust compiler.
//  2. rustup: A command line tool for managing Rust versions and associated tools.
//  3. cargo: Rust’s build tool and package/dependency/crates manager.
//  4. rust-analyzer: Rust’s language server.
//  5. rustfmt: A tool for formatting Rust code and ensure consistent coding style.
//  6. rustfix: A tool for automatically fixing lint warnings. Run `cargo fix` to use it.
//  7. clippy: A tool for catching common mistakes and improving your Rust code. Run `cargo clippy` to use it.

// Keywords
//   Rust has a set of keywords that are reserved for use by the language. These keywords are used to declare things like variables, functions, structs, enums, traits, and more.
//     Keywords Reserved for Future Use -> abstract, become, box, do, final, macro, override, priv, try, typeof, unsized, virtual, yield
//     Keywords Currently in Use -> `as`, `async`, `await`, `break`, `const`, `continue`, `crate`, `dyn`, `else`, `enum`, `extern`, `false`, `fn`, `for`, `if`, `impl`, `in`, `let`, `loop`, `match`, `mod`, `move`, `mut`, `pub`, `ref`, `return`, `Self`, `self`, `static`, `struct`, `super`, `trait`, `true`, `type`, `unsafe`, `use`, `where`, `while`

// Identifiers
//   Identifiers are names of functions, variables, parameters, struct fields, modules, crates, constants, macros, static values, attributes, types, traits, or lifetimes.
//   Identifiers are case sensitive.
//   Identifiers cannot be start with digits.
//   Identifiers cannot be a single underscore.
//   Identifiers can only contains alphanumeric characters and underscores (except as raw identifiers).
//   Keywords cannot be used as identifiers (except as raw identifiers)

// Raw Identifiers
//   Raw identifiers are the syntax that lets you use keywords where they wouldn’t normally be allowed. You use a raw identifier by prefixing a keyword with r#.
//     e.g. `r#for` is a raw identifier for the `for` keyword.
//   The Raw identifiers allow us to use keywords as identifiers. It allows us to use libraries written in a different Rust edition in which a keyword is not reserved.
//     e.g. `try`` isn’t a keyword in the 2015 edition but is in the 2018, 2021, and 2024 editions. If you depend on a library that is written using the 2015 edition and has a try function, you’ll need to use the raw identifier syntax, `r#try` in this case, to call that function from your code on later editions.

// Rust convention
//   1. Use snake_case for variable names, functions, and file names.
//   2. Use SCREAMING_SNAKE_CASE(snake_case in uppercase) for constant names.

// Rust: Hello World Program
fn main() {
    println!("Hello World");
}

// Running Rust
// Run `rustc main.rs` to compile the Rust code, and then run the resulting executable with `./main` on Linux or MacOS, or `./main.exe` on Windows.
// Run `cargo run` to compile and run the code.

// Anatomy of Hello World Program
//   fn main() { ... } - is the main function, where fn is a keyword that defines a function and main is the name of the function.
//   println! - is a Rust macro that prints a string to the standard output. Rust macros are a way to write code that generates code to extend Rust syntax.
//   `;` semicolon - which indicates that this expression is over, Most lines of Rust code end with a semicolon.
//   "Hello World" - is a string literal.
//   println!("Hello World"); - is a call to the println! macro, which prints the string to the standard output.

// Every Rust program must have a `main` function that is the entry point of the program. In other words the `main` function is the first function that gets called when the program starts.

// Good to know
//   1. In Rust, packages of code are called crates.
//   2. `prelude` is a module that standard library/crate that it brings into the scope of every program.
