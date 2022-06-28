// Structs - Used to create custom data types, Struct is the poor man's Class

// Traditional Struct
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// Tuple struct
struct ColorTuple(u8, u8, u8);

struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    // Construct person
    fn new(first: &str, last: &str) -> Person{
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }

    // Get Full Name (yeah we're using "self"/"this")
    // notice the lack of a ";" and use of "->" since we are changing stuff not returning it
    // as though it were a println
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}

pub fn run(){

    let mut c = Color {
        red: 255,
        green: 0,
        blue: 0,
    };

    c.red = 200;

    println!("Color: {} {} {}", c.red, c.green, c.blue);

    let mut ct = ColorTuple(255, 0, 0);

    ct.0 = 123;

    println!("Color Tuple: {} {} {}", ct.0, ct.1, ct.2);

    let mut per = Person::new("Billy","Bob");
    println!("{} {}", per.first_name, per.last_name);
    println!("{}", per.full_name());
}