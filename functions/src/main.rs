fn main() {
    println!("Hello, world!");

    another_banger(24, 25);

    let x = five();
    println!("The value of x is: {}", x);

    let x = plus_one(5);
    println!("The value of x is: {}", x);
}

fn another_banger(x: i32, y: i32) {
    println!("Another Banger is: {}", x);
    println!("Soon he'll be: {}", y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
