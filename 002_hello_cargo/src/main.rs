fn main() {
    println!("Hello, world!");
}

// Cargo is Rust's build tool and package/dependency manager, which makes adding, compiling and managing dependencies painless and consistent across the Rust ecosystem.

// Check Cargo version
//   Run `cargo --version`

// Creating a new project with Cargo
//   Run `cargo new hello_cargo` - This will create a new directory called hello_cargo with a simple hello world program in `src/main.rs` and a `Cargo.toml` file.
//   Run `cargo init` - This will initialize a new project with a Cargo.toml file in the current directory.

// Build a project
//   Run `cargo build` - This will compile the project and create an executable file in the `target/debug` directory with the name of the project (in this case `hello_cargo`).
//   Run `cargo build --release` - This will compile the project and create an executable file in the `target/release` directory with the name of the project (in this case `hello_cargo`).

// Run a project
//   Run `cargo run` - This will compile the project and run the executable file.
//   Run `cargo run --release` - This will compile the project and run the executable file in release mode.

// 1. Cargo we run `cargo build` for the first time, it will download all the dependencies and build the project.
// 2. When you run `cargo build` again, it will not download the dependencies again and will not rebuild the project unless we modify the source code.
// 3. Cargo will cache the dependencies and build artifacts in the `.cargo` directory in the user's home directory.
// 4. The above rules applied to the `cargo run` command.

// Cargo Check command
//   Run `cargo check` or `cargo check --release` - This command quickly checks your code to make sure it compiles but doesn’t produce an executable.

// Clean a project
//   Run `cargo clean` - This command removes the target directory.

// Add dependency
//   Run `cargo add <dependency>`

// Remove dependency
//   Run `cargo remove <dependency>`

// Build package's documentation
//   Run `cargo doc`

// Fix lint warnings
//   Run `cargo fix`

// Format code
//   Run `cargo fmt`

// To see help for a cargo command
// Run `cargo help` or `cargo --help`
// Run `cargo help <command>` to see help for a specific command.
// Run `cargo --list` to see all available commands.