fn divide (x: i32, y: i32) -> Result<i32, String> {
    if y == 0 {
        Err(String::from("Cannot divide from 0"))
    } else {
        Ok( x / y)
    }
}

pub fn err_handling() {
    match divide(0, 0) {
        Ok(result) => {
            println!("result: {}", result);
        }
        Err(error) => {
            println!("error: {}", error);
        }
    }
}