fn main() {
    let mut x = 5;
    println!("The value of x {}", x);

    x = 6;
    println!("The value of x {}", x);

    let y = 7;
    println!("The value of y is {}", y);

    // y = 9;  This line of code won't run. In RUST, Variables are immutable by default.
    println!("The value of y is {}", y);
}
