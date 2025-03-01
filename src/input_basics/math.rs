use std::io;

pub fn calculate_circle_properties() {
    let mut radius = String::new();
    println!("Enter the radius of the circle:");
    io::stdin().read_line(&mut radius).unwrap();
    let radius: f64 = radius.trim().parse().unwrap();
    println!("Circle radius: {}", radius);
    println!("Circumference: {:.2}", 2.0 * 3.14159 * radius);
    println!("Area: {:.2}", 3.14159 * radius * radius);
}

pub fn calculate_percentage() {
    let mut marks = vec![];
    for i in 1..=5 {
        let mut mark = String::new();
        println!("Enter marks for subject {}:", i);
        io::stdin().read_line(&mut mark).unwrap();
        marks.push(mark.trim().parse::<f64>().unwrap());
    }
    let total: f64 = marks.iter().sum();
    println!("Percentage: {:.2}%", total / 5.0);
}

pub fn sum_upto_n() {
    let mut n = String::new();
    println!("Enter a number (n):");
    // println!()
    io::stdin().read_line(&mut n).unwrap();
    let n: i32 = n.trim().parse().unwrap();
    let sum: i32 = (1..=n).sum();
    println!("Sum from 1 to {} is {}", n, sum); 
}

pub fn box_properties() {
    let mut dimensions = vec![];
    for dim in ["length", "width", "height"] {
        let mut input = String::new();
        println!("Enter the {} of the box:", dim);
        io::stdin().read_line(&mut input).unwrap();
        dimensions.push(input.trim().parse::<f64>().unwrap());
    }
    let (l, w, h) = (dimensions[0], dimensions[1], dimensions[2]);
    println!("Surface Area: {:.2}", 2.0 * (l * w + w * h + l * h));
    println!("Volume: {:.2}", l * w * h);
}

pub fn print_even_or_odd() {
    println!("Even numbers from 1 to 100:");
    for num in 1..=100 {
        if num % 2 == 0 {
            print!("{} ", num);
        }
    }
    println!();
}

pub fn print_primes() {
    println!("Prime numbers from 1 to 100:");
    for num in 2..=100 {
        if (2..num).all(|x| num % x != 0) {
            print!("{} ", num);
        }
    }
    println!();
} 

pub fn fibonacci_series() {
    let mut a = 0;
    let mut b = 1;
    println!("Fibonacci series:");
    for _ in 0..13 {
        print!("{} ", a);
        let temp = a;
        a = b;
        b = temp + b;
    }
    println!();
}

pub fn multiplication_table() {
    let mut num = String::new();
    println!("Enter a number to generate its multiplication table:");
    io::stdin().read_line(&mut num).unwrap();
    let num: i32 = num.trim().parse().unwrap();
    for i in 1..=10 {
        println!("{} x {} = {}", num, i, num * i);
    }
}
