use std::fmt::Display;
fn longest_with_ann<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str where T: Display {
    println!("announcement: {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

pub fn lifetimes_code() {}