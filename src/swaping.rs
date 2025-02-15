pub fn swap() {
    let mut a = 5;
    let mut b = 10;

    println!("Before swapping: a = {}, b = {}", a, b);

    // Swapping using arithmetic operations
    a = a + b;
    b = a - b;
    a = a - b;

    println!("After swapping: a = {}, b = {}", a, b);
}
