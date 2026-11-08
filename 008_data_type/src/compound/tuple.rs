// 1. A tuple is a compound type used to group multiple values of different types into a single value.
// 2. Tuples have a fixed length, meaning once declared, they cannot grow or shrink in size.
// 3. We define a tuple by writing a comma-separated list of values inside parentheses`()`.
// 4. We can access individual tuple elements using a period`.` followed by the zero-based index of the element.
// 5. Unit Type
//   A tuple without any elements is called a unit tuple, commonly known as the unit value.
//   Both the unit value and its corresponding type are represented by empty parentheses`()`.
//   The unit type represents an empty value and is commonly used as the return type of functions that don't return any meaningful value.
//   Expressions that don't produce another value evaluate to the unit value ().

pub fn tuple() {
    let alice = ("alice", 25); // Implicitly typed tuple

    // Accessing tuple elements using their index
    println!("Hello {}, you are {} years old.", alice.0, alice.1);

    let bob: (&str, u8) = ("bob", 22); // Explicitly typed tuple

    let (name, age) = bob; //Destructuring a tuple
    println!("Hello {name}, you are {age} years old.");

    // Unit Tuple
    let unit: () = ();
    println!("{:?}", unit);
}
