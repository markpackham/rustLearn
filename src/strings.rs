// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - Use when you need to modify or own string data

pub fn run(){
    let prim = "I am a primative string";

    let mut hello = String::from("Hello I am mutable and live in the heap ");

    // Get length
    println!("Length: {}", hello.len());

    // Push char
    hello.push('Z');

    // Push string
    hello.push_str(" more stuff can be added with strings vs chars");

    println!("{} you can never change me", prim);
    println!("{}", hello);

}