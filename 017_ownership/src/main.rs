// 1. Ownership is a set of rules that govern how a Rust manges memory.
//   In some programming languages, memory is managed automatically through a
//   mechanism called Garbage Collection (GC). The garbage collector periodically
//   identifies memory that is no longer being used and frees it.
//
//   In other languages, programmers must manually allocate and deallocate memory,
//   taking responsibility for managing memory throughout the program.
//
//   Rust takes a different approach using a system called Ownership, which manages
//   memory without requiring a garbage collector.
//
//   Ownership is based on a set of rules that the Rust compiler checks at compile
//   time. If any of these rules are violated, the program will not compile.
//
//   The ownership system does not introduce runtime overhead because its rules
//    are enforced at compile time rather than while the program is running.

// 2. Stack and Heap
//    Both the stack and the heap are parts of memory available to your code to use at runtime.
//    Data stored on the stack must have a known and fixed size at compile time.
//    Data with unknown size at compile time or a size that might change at runtime must be stored on the heap.
//    Value is on the stack or the heap affects how languages behaves.
//
//    Stack is referred to as `last in, first out (LIFO)` - Adding data is called pushing onto the stack, and removing data is called popping off the stack.
//
//    When we put data on the heap, the memory allocator finds an empty spot in the heap that is big enough, marks it as begin used, and returns a pointer, which is the address of the that location. This process is called `allocating on the heap` or `memory allocation` or `allocating/allocation` (pushing values onto the stack is not considered allocating).
//    Because the pointer to the heap is a known and fixed size, we can store the pointer on the stack, but when we want the actual data, we must follow the pointer.
//
//    Pushing to the stack is faster than allocating on the heap because the allocator never has to search for a place to store new data; that location is always at the top of stack.
//    Accessing data in heap is generally slower that accessing data on the stack because we have to follow a pointer to get there.
//
//    When a functions called, the value passed into the function (including, potentially, pointers to data on the heap) and the function's local variables get pushed onto the stack. when the function is over, those values get popped of the stack.
//
//    Keeping track of what parts of code are using what data on the heap, minimized the amount of duplicate data on the heap and cleaning up unused data on the heap so that you don't run out of space are all problems that ownership addresses.
//
//    In Rust, scalar values, `tuples`, and `arrays` are generally stored on the stack and popped off the stack when their scope is over, and can be quickly and trivially copied to make a new independent instance if another part of code needs to use the same value in different scope.
//    but the purpose of ownership is to manage heap data such as such as `String`, `Vec`, `Box` and other complex data types, whether they are provided by the standard library or user defined.

// 3. Ownership Rules
//    Each value in Rust has an owner.
//    Their can only be one owner at a time.
//    when the owner goes out of scope, the value will be dropped.

// 4. Memory & Allocation
//    

// 5. Variable & Data Interacting with Move
//    let x = 5;
//    let y = x;
//
//    Here, Bind the value 5 to variable `x`,
//          Then, make a copy of the value in x and bind it to `y`.
//          Now, we have two variables `x` and `y` and both equal 5.
//          because integers are simple values with known and fixed sized and these two 5 values are pushed onto stack.
//
//    Now, Lets looks at the `String` version:
//    let s1 = String::from("Hello");
//    let s2 = s1;
//
//    Here,
//          A `String` consist three parts, On the left: a pointer to the memory that holds the contents of the string, length and a capacity. Stored on the stack.
//          On the right: Memory on the heap that holds the contents.
//
//                   S1
//          |------------------|          |---------------|
//          | name     | value |          | index | value |
//          | -------- | ----- |          | ----- | ----- |
//          | pointer  |  *----|--------->| 0     | H     |
//          | length   | 5     |          | 1     | e     |
//          | capacity | 5     |          | 2     | l     |
//                                        | 3     | l     |
//                                        | 4     | o     |
//
//          Length: How much memory the contents or the String are currently using in bytes.
//          Capacity: Total amount of memory String has received from the allocator in bytes.
//
//    When we assign `s1` to `s2`, the String data is copied meaning we copy the pointer, the length and the capacity that are on the stack. We do not copy the data on the heap that the pointer refers to. In order to avoid runtime performance overhead of allocating memory.
//
//                   S1
//          |------------------|
//          | name     | value |
//          | -------- | ----- |
//          | pointer  |   *---|----|
//          | length   | 5     |    |
//          | capacity | 5     |    |       |---------------|
//                                  |       | index | value |
//                                  |       | ----- | ----- |
//                                  |------>| 0     | H     |
//                    s2            |       | 1     | e     |
//          |------------------|    |       | 2     | l     |
//          | name     | value |    |       | 3     | l     |
//          | -------- | ----- |    |       | 4     | o     |
//          | pointer  |   *---|----|
//          | length   | 5     |
//          | capacity | 5     |
//
//    Rust automatically calls the `drop` function and cleans up the heap memory for the variables goes out of scope.
//        But in above illustration, `s1` and `s2` both pointers pointing to same memory location. This is a problem when both `s1` and `s2` goes out of scope, they will both try to free the same memory. This is known as a double free error and is one of the memory safety bugs.
//        Freeing memory twice can lead to memory corruption, which can potentially lead to security vulnerability.
//
//        To ensure memory safety, after the line `let s2 = s1;`, Rust considers `s1` as no longer valid. Therefore, Rust doesn't need to free anything when `s1` goes out of scope, only `s2` is valid, when it goes out of scope it alone will free the memory.
//*       Move: It's like shallow copy, but because Rust invalidates the first variable, instead of being called shallow copy, it's know as move.
//
//             S1 (Invalidate)
//          |------------------|
//          | name     | value |
//          | -------- | ----- |
//          | pointer  |   *---|----|
//          | length   | 5     |    |
//          | capacity | 5     |    |       |---------------|
//                                  |       | index | value |
//                                  |       | ----- | ----- |
//                                  |------>| 0     | H     |
//                    s2            |       | 1     | e     |
//          |------------------|    |       | 2     | l     |
//          | name     | value |    |       | 3     | l     |
//          | -------- | ----- |    |       | 4     | o     |
//          | pointer  |   *---|----|
//          | length   | 5     |
//          | capacity | 5     |
//

fn _move() {
    let s1 = String::from("Hello");
    let s2 = s1; // s1 -> s2 (s1 is moved to s2)
                 // println!("{s1}"); // Here, s1 is invalid, we will get an error `value borrowed here after move`
    println!("{s2}")
}

// 6. Scope & Assignment
//    The inverse of this is true for the relationship between scoping, ownership, and memory being freed via the `drop` function as well.
//    When we assign a completely new value to an existing variable, Rust will call `drop` and free the original value's memory immediately.

fn immediate_drop() {
    let mut s = String::from("Hello");
    print!("{s} ");

    // Here, we assign a new value, "World", to `s`.
    // The original string "Hello" immediately goes out of scope.
    // Rust automatically drops the original value, "Hello", freeing its allocated memory immediately.
    s = String::from("World");

    println!("{s}");
}

// 7. Variable & Data Interacting with Clone
//    We can create deep copy of heap data of the String, not not just the stack data using `clone` method.
//
//                   S1
//          |------------------|          |---------------|
//          | name     | value |          | index | value |
//          | -------- | ----- |          | ----- | ----- |
//          | pointer  |  *----|--------->| 0     | H     |
//          | length   | 5     |          | 1     | e     |
//          | capacity | 5     |          | 2     | l     |
//                                        | 3     | l     |
//                                        | 4     | o     |
//                   S2
//          |------------------|          |---------------|
//          | name     | value |          | index | value |
//          | -------- | ----- |          | ----- | ----- |
//          | pointer  |  *----|--------->| 0     | H     |
//          | length   | 5     |          | 1     | e     |
//          | capacity | 5     |          | 2     | l     |
//                                        | 3     | l     |
//                                        | 4     | o     |

fn clone() {
    let s1 = String::from("Hello");
    let s2 = s1.clone();
    println!("s1: {s1}\ns2: {s2}");
}

// 8. Stack Only Data: Copy
//    let x = 5;
//    let y = x;
//    println!("x = {x}, y = {y}");
//
//   In above example, `x` wasn't moved to `y`, and we don't have to call `clone` but `x` and `y` both are valid,
//       Because integers have known and fixed size and stored entirely on the stack, so copies of the actual values are quick to make.
//       Their for their is no reason to make `x` invalid after creating `y`. Their is no difference between shallow and deep copy here, so calling `clone` wouldn't do anything different from the usual shallow copying, and we can leave it out.
//       This is true for all scalar types, and also tuple and array if they only contains types that also implement `Copy` trait.
//
//   Rust has a special annotation called the `Copy` trait that we can place on types that are stored on the stack as all scalar types, and also tuple and array if they only contains types that also implement `Copy` trait.
//       When a type implements the `Copy` trait, variables that use it do not move, but rather are trivially copied, making them still valid after assignment to another variable.
//

fn main() {
    _move();
    immediate_drop();
    clone();
}
