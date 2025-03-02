use std::vec::Vec;

fn vectors() {
    let v: Vec<i32> = Vec::new();
    let mut v = vec![1, 2, 3, 4];

    v.push(6);

    let third: &i32 = &v[2];
    println!("the third elemnt in the vec is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
}