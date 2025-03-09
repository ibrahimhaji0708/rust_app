use std::fmt::Debug;

trait Summary {
    fn summarize(&self) -> String;
}

struct Article<'a, T: Debug> {
    title: &'a str,
    content: T,
}

impl<'a, T> Summary for Article<'a, T>
where
    T: Debug,
{
    fn summarize(&self) -> String {
        format!("{} - {:?}", self.title, self.content)
    }
}

fn print_summary<T>(item: T)
where
    T: Summary,
{
    println!("{}", item.summarize());
}

pub fn lifetimes_code() {
    let content = String::from("Rust is awesome!");
    let article = Article {
        title: "Rust News",
        content,
    };

    print_summary(article);
}