mod compound;
mod scalar;

// Data Type
//  Rust is a statically typed language, which means that it must know the types of all variables at compile time.
//  Rust compiler can usually infer what type we want to use based on the value and how we use it.
//  Every value in Rust is of a certain data type

// Rust has 2 types of data types
//   1. Scalar - A scalar type represents a single value.
//       Rust has four primary scalar types:
//         1. Integers
//         2. Floating point numbers
//         3. Booleans (true or false)
//         4. Characters
//   2. Compound - Group of multiple values into one type
//       1. Primitive Compound Types
//           1. Tuples
//           2. Arrays
//           3. Slices
//       2. Compound Types
//           1. Structs
//           2. Enums
//           3. Unions

// Data type and their memory allocation depends on the type's contents and how it is used
//   In Rust, scalar values, tuples, and arrays are generally stored on the stack when they are local variables and contain no heap-allocated data.
//   Types such as String, Vec, and Box use heap allocation for their dynamically allocated data, while their stack variables hold pointers, lengths, capacities, or other metadata.
//   Structs, enums, and unions do not automatically use heap allocation. They are generally stored wherever their owning value is stored (e.g., on the stack or heap).
//   Slices ([T]) are dynamically sized views into a sequence of elements and do not allocate memory themselves.

fn main() {
    let age = 10; // Implicitly typed as an i32
    println!("Implicitly typed: age is {}", age);

    let age: i8 = 10; // Explicitly typed as an i8
    println!("Explicit typed: age is {}", age);

    scalar::integer::integer_value();
    // scalar::integer::integer_overflow(); // This will panic at runtime
    scalar::floating_point::floating_point();
    scalar::boolean::boolean();
    scalar::character::character();

    compound::tuple::tuple();
    compound::array::implicitly_typed_array();
    compound::array::explicitly_typed_array();
    compound::array::access_modify_array();
    // compound::array::out_of_bound_error(); // This will panic at runtime
    compound::array::copy_trait_array();
    compound::array::copy_trait_string_array();
    compound::array::slice_array();
}
