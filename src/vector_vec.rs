use std::vec::Vec;
use std::fmt::Display;

pub fn vectors() {
    // let v: Vec<i32> = Vec::new();
    let mut v = vec![1, 2, 3, 4]; //immut

    v.push(6);

    let third: &i32 = &v[2];
    println!("the third elemnt in the vec is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    for i in &v {
        println!("{i}");
    }

    for i in &mut v {
        *i += 50;
    }
}

pub fn enums() {
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}