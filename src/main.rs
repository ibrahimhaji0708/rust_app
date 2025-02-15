// use std::{thread, time::Duration};
// use donut::render_frame;

// mod math;
// mod conversions;
// mod utils;
mod practice;
mod ownr_brrw;
pub mod donut;
pub mod error_handling;
mod area_rect_structs;
mod swaping;

fn main() {
    // // Example: Calling modules to solve various tasks
    // println!("1. Accept radius and print its value:");
    // math::calculate_circle_properties();

    // println!("\n2. Accept marks of 5 subjects and calculate percentage:");
    // math::calculate_percentage();

    // println!("\n3. Accept a character and print its ASCII value:");
    // utils::ascii_value();

    // println!("\n4. Convert a string to uppercase or lowercase:");
    // utils::convert_case();

    // println!("\n5. Sum of numbers from 1 to n:");
    // math::sum_upto_n();

    // println!("\n6. Reverse 5 strings:");
    // utils::reverse_strings();

    // println!("\n7. Calculate surface area and volume of a box:");
    // math::box_properties();

    // println!("\n8. Convert Celsius to Fahrenheit and Kelvin:");
    // conversions::temperature_conversion();

    // println!("\n9. Check if a number is an Armstrong number:");
    // utils::check_armstrong();

    // println!("\n10. Print all even or odd numbers from 1 to 100:");
    // math::print_even_or_odd();

    // println!("\n11. Print all prime numbers from 1 to 100:");
    // math::print_primes();

    // println!("\n12. Print the first 13 terms of the Fibonacci series:");
    // math::fibonacci_series();

    // println!("\n13. Display product table of a given number:");
    // math::multiplication_table();

    // println!("hello world");

    // practice::varbl();
    // practice::shadowing();
    // practice::tuple();
    // practice::func();

    // practice::if_block();
    // practice::if_else_block();
    // practice::loops();
    // practice::while_loop();
    // practice::for_loop();

    //ownership and borrowing 
    // ownr_brrw::egs();

    //structs mods traits
    // area_rect_structs::rect_structs();

    //error_handling
    // error_handling::err_handling();

    //swaping
    swaping::swap();

    //donut.rs
    // let mut a = 0.0;
    // let mut b = 0.0;

    // loop {
    //     render_frame(a, b);
    //     a += 0.04;
    //     b += 0.02;
    //     thread::sleep(Duration::from_millis(30));
    // }
}

