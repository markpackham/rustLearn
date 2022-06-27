// Reference Pointers - Point to a resource in memory

pub fn run() {
    // Primative Array
    let arr1 = [1,2,3];
    let arr2 = arr1;

    // With non-primitives like Vectors, if you assign another variable to a piece of data, 
    // the first variable will no longer hold that value. You'll need to use a reference 
    // (&) to point to the resource

  // Vector
  let vec1 = vec![1, 2, 3];
  // without the "&"" this won't run
  let vec2 = &vec1;

//   Primative Values: ([1, 2, 3], [1, 2, 3])
//   Non Primative Values: ([1, 2, 3], [1, 2, 3])
    println!("Primative Values: {:?}", (arr1, arr2));
    // without the "&"" this won't run
    println!("Non Primative Values: {:?}", (&vec1, vec2));
}