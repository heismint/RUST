fn main() {
    {
        // Variable scope
        let _s = "hello"; // _s is valid here
                          // do stuff with _s
    } // _s is no longer valid

    {
        let mut x = String::from("Hello"); // The double colon allows us namespace the from function under the string type
        x.push_str(" World!!"); // push_str appends a literal to a string
        println!("{}", x); // This will print Hello World!!
    }

    {
        let x = 5;
        let y = x;

        println!("x = {}, y = {}", x, y);
    }

    {
        let s1 = String::from("Hello");
        let s2 = s1.clone(); // Clone s1 to s2

        println!("{}", s2);
    }

    {
        let x1 = String::from("Chale");
        let x2 = x1.clone();

        println!("x1 = {}, x2 = {}", x1, x2);
    }

    let s = String::from("Pops");

    takes_ownership(s); // s is moved here, can't be used after this

    let x = 5;

    makes_copy(x);

    let y1 = gives_ownership();

    println!("{}", y1);

    let y2 = String::from("My head!!");
    let y3 = takes_and_gives_back(y2.clone()); // Clone y2 to y3
    println!("{}", y3);
    
    let a1 = String::from("Morning!!"); // Create new variable a1

    let (a2, len) = calculate_length(a1.clone()); // Clone a1 to calculate_length
    println!("Length of a2 is: '{}', {}.", a2, len);
}

// Ownership and functions
fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

// Return values and scope
fn gives_ownership() -> String {
    let some_string = String::from("Rust my gee mapami naw");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String{
    a_string
}

fn calculate_length(s: String) -> (String, usize) { // Returning multiple values using a tuple
    let length = s.len();
    (s, length)
}

