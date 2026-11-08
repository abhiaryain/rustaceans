// 1. An array is a collection of multiple values stored in a contiguous block of memory with a known, fixed size.
// 2. Each element in an array must have the same data type.
// 3. Arrays have a fixed length, meaning once declared, they cannot grow or shrink in size.
// 4. We define a array by writing a comma-separated list of values inside square brackets`[]`.

pub fn implicitly_typed_array() {
    let status = ["pending", "complete", "rejected"]; // Implicitly typed array

    println!("status length: {}", status.len());
}

// 5. We specify an array's type using square brackets`[]` containing the element data type, a semicolon`;`, and the number of elements in the array. (e.g. `let shapes: [&str; 4];`)

pub fn explicitly_typed_array() {
    let shapes: [&str; 4] = ["square", "circle", "rectangle", "triangle"]; // Explicitly typed array

    for shape in shapes {
        println!("{shape}");
    }
}

// 6. We can also initialize an array to contain the same value for each element by specifying the initial value, followed by a semicolon, and then the length of the array in square brackets. (e.g. `let visited = [false; 10];`)
// 7. We can access/modify individual array elements using zero-based indexing. (e.g. `shape[2]`)

pub fn access_modify_array() {
    let mut visited = [false; 10]; // Array initialization with the same value

    visited[5] = true;
    println!("visited[5]: {}", visited[5])
}

// 8. When we try to access/modify an array element using an index that is outside its valid range, the program panics at runtime with an index-out-of-bounds error.

// pub fn out_of_bound_error() {
//     let mut visited = [false; 10]; // Array initialization with the same value

//     visited[11] = true; // panic -> index out of bounds
//     println!("visited[5]: {}", visited[5])
// }

// 9. Arrays implement the Copy trait when their element type implements Copy and the array length is within the applicable trait implementation.

pub fn copy_trait_array() {
    let numbers = [10, 20, 30];

    let copied_numbers = numbers;

    println!("Original: {:?}", numbers);
    println!("Copied: {:?}", copied_numbers);
}

//   String does not implement copy trait
pub fn copy_trait_string_array() {
    let names = [String::from("Alice"), String::from("Bob")];

    let copied_names = names;

    // println!("{:?}", names); // Error: value borrowed after move

    println!("{:?}", copied_names);
}

// 10. Rust provides slices to access a portion of an array without copying its elements.

pub fn slice_array() {
    let numbers = [10, 20, 30, 40, 50];

    let slice = &numbers[1..4];

    println!("{:?}", slice); // [20, 30, 40]
}
