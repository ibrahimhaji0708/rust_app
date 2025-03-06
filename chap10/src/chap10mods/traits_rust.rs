pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String,
}
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {}", self.headline, self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool, 
}
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub trait Summary {
    fn summarize(&self) -> String;
}

pub fn trait_code() {
    let tweet =  Tweet {
        username: String::from("ibrahim3595"),
        content: String::from("hello world</>"),
        reply: false,
        retweet: false,
    };
    let article = NewsArticle {
        author: String::from("Ibrahim Haji"),
        headline: String::from("he is coding"),
        content: String::from("dont disturb"),
    };

    println!("tweet summary: {}", tweet.summarize());
    println!("article summary: {}", article.summarize());
}