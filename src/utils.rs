use std::io;

pub fn ascii_value() {
    let mut input = String::new();
    println!("Enter a character:");
    io::stdin().read_line(&mut input).unwrap();
    let c = input.trim().chars().next().unwrap();
    println!("ASCII value of '{}' is {}", c, c as u8);
}

pub fn convert_case() {
    let mut input = String::new();
    println!("Enter a string:");
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();
    println!("Uppercase: {}", input.to_uppercase());
    println!("Lowercase: {}", input.to_lowercase());
}

pub fn reverse_strings() {
    let mut strings = vec![];
    for i in 1..=5 {
        let mut input = String::new();
        println!("Enter string {}:", i);
        io::stdin().read_line(&mut input).unwrap();
        strings.push(input.trim().to_string());
    }
    strings.reverse();
    println!("Reversed strings: {:?}", strings);
}

pub fn check_armstrong() {
    let mut num = String::new();
    println!("Enter a number:");
    io::stdin().read_line(&mut num).unwrap();
    let num: u32 = num.trim().parse().unwrap();
    let sum: u32 = num.to_string().chars().map(|c| c.to_digit(10).unwrap().pow(3)).sum();
    if sum == num {
        println!("{} is an Armstrong number", num);
    } else {
        println!("{} is not an Armstrong number", num);
    }
}
