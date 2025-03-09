pub fn lifetimes_code() {
    let str1 = String::from("abcd");
    let str2 = String::from("xyz");

    let result = longest(str1.as_str(), str2.as_str());
    println!("longest: {result}");
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}