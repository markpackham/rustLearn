// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run(){
  let name = "Juan";
  // make the variable mutable
  let mut age = 20;
  println!("My name is {} and I am {}", name, age);
  age = 10;
  println!("My name is {} and I am {}", name, age);

  // Define constant (little point to this if immutable is the default)
  // If you use const you MUST define a type and it's common to use UPPERCASE
  const SECRET_ID: i32 = 000001;
  println!("ID: {}", SECRET_ID);

  // Assign multiple vars
  let ( my_name, my_age ) = ("Tim", 15);
  println!("{} is {}", my_name, my_age );
}