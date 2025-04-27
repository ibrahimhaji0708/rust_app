// use std::{thread, time::Duration};
// use donut::render_frame;
// use crate::adv_basics::fridge_operations;
mod swaping;
mod lifetimes;
mod adv_basics;
mod vector_vec;


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
    // lifetimes::lifetimes::lifetimes();

    //vectors
    // vector_vec::enums();

    //mutability_refrencing
    let some_struct = SomeStruct { num: 3 };
    print_some_struct(&some_struct);
}

#[derive(Debug, Clone, Copy)]
struct SomeStruct {
    num: i32,
}

fn print_some_struct(the_struct: &SomeStruct) {
    println!("{:?}", the_struct);
}

fn biggest<'a>(a: &'a SomeStruct, b: &'a SomeStruct) -> &'a SomeStruct {
    if a.num > b.num {
        a 
    } else {
        b
    }
}