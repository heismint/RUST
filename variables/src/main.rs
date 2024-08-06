fn main() {
    let mut x = 5;
    println!("The value of x {}", x);

    x = 6;
    println!("The value of x {}", x);

    let y = 7;
    println!("The value of y is {}", y);

    // y = 9;  This line of code won't run. In RUST, Variables are immutable by default.
    println!("The value of y is {}", y);
    
    // Constants
   // const MIN_POINT: u32 = 100_000;

    // Shadowing
    let y = 5;
    let y = y * 2;
    let y = y + 200;

    println!("The value of y is {}", y);
    
    // Scalar Types

    // Numeric Operations
    let _sum = 5 + 10; // addition
    let _difference = 43.5 - 22.8; // subtraction 
    let _product = 4 * 10; // multiplication
    let _quotient = 8 / 2; // division
    let _remainder = 43 % 5; // remainder

    // The Boolean type
    let _t = true;
    let _f: bool = false; // with explicit type annotation

    // The Character type
    let _m = 't';
    let _z = '●';
    let _emoji = '@';

    // Compound Types
    
    // The Tuple type
    let x: (i32, f64, u8) = (200, 6.4, 1); // explicit type annotation
    let _two_hundred = x.0; // Accessing a tuple value using a period(.) followed by it's index
    let _six_point_four = x.1;
    let _one = x.2;

    let tup = (200, 6.4, 1);
    let (_x,y,_z) = tup; // Using pattern matching to destructure a tuple

    println!("The value of y is {}", y);

    // The Array type
    let a = [1, 2, 3, 4, 5];

    let _first = a[0];
    let _second = a[1];

    let _b: [i32; 5] = [1, 2, 3, 4, 5]; // explicit type annotation

    let _c = [1, 2, 3, 4, 5];
    let index = 10;
    let element = a[index];

    println!("The value of element is: {}", element);
}
