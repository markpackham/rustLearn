pub fn run() {
    // Print to console
    println!("Hello from print.rs");

    // Basic Formatting
    println!("{} is {} year old", "Billy", 999);

    // Positional Arguments
    println!("{0} is from {1} and {0} likes to {2}", "Billy", "London", "dance");

    // Named Arguments
    println!("{name} likes to play {activity}", name = "Bob", activity = "drums");

    // Placeholder traits
    println!("Binrary: {:b} - Hex: {:x} - Octal: {:o}", 1, 1, 1);
    println!("Binrary: {:b} - Hex: {:x} - Octal: {:o}", 5, 5, 5);
    println!("Binrary: {:b} - Hex: {:x} - Octal: {:o}", 10, 10, 10);

    // Placeholder for debug tool trait
    println!("{:?}", (12, true, "hello"));

    // Basic maths
    println!("1 + 2 = {}", 1 + 2);
}