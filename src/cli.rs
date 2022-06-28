use std::env;

// Grabbing terminal input and responding to it

pub fn run(){
    let args: Vec<String> = env::args().collect();
    let command: String = args[1].clone();
    let name: &str = "Jim";
    let status: &str = "100%";
  
    // to trigger this run
    // cargo run hello
    if(command == "hello") {
        println!("Hi {}, how are you", name);
    } 
    // cargo run status
    else if command == "status" {
        println!("Status is {}", status);
    } else{
        println!("That is not a valid command");
    }
}