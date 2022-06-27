// Functions - Used to store blocks of code for re-use

pub fn run(){
    greeting("Yo","Adrian");

    // Bind function values to variables
    let get_sum: i32 = add(3,4);

    println!("Sum: {}", get_sum);

    // Closure (we can use outside variables rather than just block scoped ones so n3 can be included)
    let n3: i32 = 10;
    let add_nums =  |n1:i32, n2:i32| n1 + n2 + n3;
    println!("Closure Sum: {}", add_nums(1,2));
}

fn greeting(greet: &str, name: &str){
    println!("{} {}, pleased to meet you.", greet, name);
}

// we want to -> (return) an int
fn add(n1:i32, n2:i32) -> i32 {
    // notice the lack of a semicolon since we are returning stuff
    n1 + n2
}