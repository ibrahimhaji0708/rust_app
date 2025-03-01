use std::io;

pub fn temperature_conversion() {
    let mut celsius = String::new();
    println!("Enter temperature in Celsius:");
    io::stdin().read_line(&mut celsius).unwrap();
    let celsius: f64 = celsius.trim().parse().unwrap();
    println!("Fahrenheit: {:.2}", celsius * 9.0 / 5.0 + 32.0);
    println!("Kelvin: {:.2}", celsius + 273.15);
}
