fn main() {

    // if statment
    let number = 7;

    if number < 5 {
        println!("Condition was true chale");
    } else {
        println!("Condition was false chale");
    }

    // Handling multiple conditions with else if
    let number2 = 6;

    if number2 % 4 == 0 {
        println!("number is divisible by 4 chale");
    } else if number2 % 3 == 0 {
        println!("number is divisible by 3");
    } else if number2 % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3 or 2");
    }

    // Using if in a let statement 
    let condition = false;
    let number3 = if condition {
        5
    } else {
        6
    };
    println!("The value of number is: {}", number3);

    // Repetition with loops
    loop {
        println!("Chale print am again!!");
        break;
    } 
    
    // Returning values from loops
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);

    // Conditional loops with While
    let mut number_w = 3;

    while number_w != 0 {
        println!("{}!", number_w);
        number_w = number_w - 1;
    }
    println!("CHALE!!!");

    // looping through a collection of array with for and while
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("The value is: {}", a[index]);

        index = index + 1;
    }

    // using a for loop
    let a = [10, 20, 30, 40, 50, 60];

    for element in a.iter() {
        println!("The value is: {}", element);
    }

    // countdown using for loop
    for number_con in (1..4).rev() {
        println!("{}!", number_con);
    }
    println!("CHALE!!!");
}
