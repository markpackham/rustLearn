// Tuples group together values of different types
// Max 12 elements

pub fn run(){
    let tuple_demo: (&str, &str, i8, i64) = ("Billy", "Bob", 8, 99999999);

    println!("{} is from {} and is {} with an IQ of {}", tuple_demo.0, tuple_demo.1, tuple_demo.2, tuple_demo.3);
}