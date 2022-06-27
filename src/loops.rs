// Loops - Used to iterate until a condition is met

pub fn run(){
    
    let mut count = 0;

    // loop {
    //   count += 1;
    //   println!("Number: {}", count);
  
    //   if count == 5 {
    //     break;
    //   }
    // }

    // While Loop
    // while count <= 50{
    //     if count % 15 == 0 {
    //         println!("FizzBuzz");
    //     }
    //     else if count % 3 == 0 {
    //         println!("Buzz");
    //     }
    //     else if count % 5 == 0{
    //         println!("Fizz");
    //     }
    //     else{
    //         println!("{}",count);
    //     }
    //     count += 1;
    // }

    // For Range
    for count in 0..50{
        if count % 15 == 0 {
            println!("FizzBuzz");
        }
        else if count % 3 == 0 {
            println!("Buzz");
        }
        else if count % 5 == 0{
            println!("Fizz");
        }
        else{
            println!("{}",count);
        }
    }


}