// use std::io;
// common programming concepts
pub fn varbl() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}

pub fn shadowing() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}

pub fn tuple() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_x, y, _z) = tup;
    println!("the value of y is: {y}");
}

pub fn func() {
    let x = plus_one(5);
    println!("The value of x is: {x}");
}
fn plus_one(x: i32) -> i32 {
    x + 1
}

// control flow: 
// if-else 
pub fn if_block() {
    let number = 7;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}
pub fn if_else_block() {
    let number = 6;
    let condition = true;
    let num = if condition { 5 } else { 6 };

    println!("The value of number is: {num}");
    
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3 or 2");
    }
}

//loops
pub fn loops() {
    // loop {
    //     println!("again!");
    // }

    // let mut counter = 0;

    // let result = loop {
    //     counter += 1;

    //     if counter == 10 {
    //         break counter * 2;
    //     }
    // };

    // println!("The result is {result}");
    //another 'loop
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

pub fn while_loop() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

pub fn for_loop() {
    let a = [10, 20, 30, 40, 50];
    
    for element in a {
        println!("the value is: {element}");
    }
}


// ownership && borrowing
