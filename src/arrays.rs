// Arrays - FIXED size & type!!!! list where elements are the same data types

use std::mem;

pub fn run(){
    let mut numbers: [i64; 5] = [1,2,3,4,5];

    // Re-assign values - we can't add on to them but we can change them
    numbers[2] = 300;

    println!("{:?}", numbers);

    // Get single value
    println!("First value {}", numbers[0]);

    // Get length
    println!("Array length {}", numbers.len());

    // Arrays are stack allocated, we can get their size in bytes
    // since we use std::mem we can just write "mem::"
    // println!("Array occupies {} bytes", std::mem::size_of_val(&numbers));
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    // Get Slice
    // Slice: [1, 2]
    let slice: &[i64] = &numbers[0..2];
    println!("Slice: {:?}", slice);

}