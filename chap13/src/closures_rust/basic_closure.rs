pub fn addn() {
    let add = |a, b| a + b;
    let res = add(5, 5);
    println!("the result is {}", res);
}

pub fn capture_by_reference() {
    let x = 5;
    let print_x = || println!("x is {}", x);
    print_x();
}

pub fn capture_by_value() {
    let x = String::from("hello");
    let takes_ownership = || {
        println!("x inside closure: {} ", x);
    };
    takes_ownership();
}

pub fn capture_by_mut_reference() {
    let mut x = 5;
    let mut change_x = || {
        x += 1;
    };
    change_x();
    println!("x after closure {}", x);
}