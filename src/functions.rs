// Functions - Used to store blocks of code for re-use

pub fn run(){
    greeting("Yo","Adrian");

    // Bind function values to variables
    let get_sum: i32 = add(3,4);

    println!("Sum: {}", get_sum);
}

fn greeting(greet: &str, name: &str){
    println!("{} {}, pleased to meet you.", greet, name);
}

// we want to -> (return) an int
fn add(n1:i32, n2:i32) -> i32 {
    // notice the lack of a semicolon since we are returning stuff
    n1 + n2
}