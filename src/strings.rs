// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - Use when you need to modify or own string data

pub fn run(){
    let prim = "I am a primative string";

    let mut hello = String::from("Hello I am mutable and live in the heap ");

    println!("{} you can never change me", prim);
    println!("{}", hello);

    // Get length
    println!("Length: {}", hello.len());

    // Push char
    hello.push('Z');

    // Push string
    hello.push_str(" more stuff can be added with strings vs chars");

    // Capticy in bytes
    println!("Capacity: {} bytes", hello.capacity());

    // Check if String is empty
    println!("Is Empty: {}", hello.is_empty());

    // Replace
    println!("Replace: {}", hello.replace("Hello","Meow"));

    // Contains
    println!("Contains 'Hello' {}", hello.contains("Hello"));

    hello = String::from("Hello World changed");

    // Loop through String by whitespace
    for word in hello.split_whitespace(){
        println!("{}", word);
    }

    // Create string with capacity
    let mut stringy = String::with_capacity(10);
    stringy.push('a');
    stringy.push('b');

    // Assertion testing (only shows error if it fails)
    assert_eq!(2, stringy.len());
    assert_eq!(10, stringy.capacity());

    println!("{}", stringy);

}