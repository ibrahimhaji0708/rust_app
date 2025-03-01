// use std::{thread, time::Duration};
// use donut::render_frame;
// use crate::adv_basics::fridge_operations;

mod swaping;
mod lifetimes;
mod adv_basics;


fn main() {

    println!("hello world");

    //donut.rs
    // let mut a = 0.0;
    // let mut b = 0.0;

    // loop {
    //     render_frame(a, b);
    //     a += 0.04;
    //     b += 0.02;
    //     thread::sleep(Duration::from_millis(30));
    // }

    //lifetimes
    lifetimes::lifetimes::lifetimes();

    //adv-basics
    adv_basics::fridge_operations();
    // adv_basics::fridge::cooler::Cooler();
    // adv_basics::fridge::freezer::Freezer();
}

