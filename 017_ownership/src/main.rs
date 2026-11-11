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
//    In Rust, scalar values, tuples, and arrays are generally stored on the stack and popped off the stack when their scope is over, and can be quickly and trivially copied to make a new independent instance if another part of code needs to use the same value in different scope.
//    but the purpose of ownership is to manage heap data such as such as String, Vec, Box and other complex data types, whether they are provided by the standard library or user defined.

// 3. Ownership Rules
//    Each value in Rust has an owner.
//    Their can only be one owner at a time.
//    when the owner goes out of scope, the value will be dropped.

fn main() {}
