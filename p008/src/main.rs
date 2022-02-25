//----Traits
// Used to define shared behavior between items. Like a Java interface

pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String
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

pub trait Summary {
    fn summarize(&self) -> String;
}

fn main() {
    let art = NewsArticle {
        author: String::from("keith"),
        headline: String::from("This just in"),
        content: String::from("Body")
    };

    let sum = art.summarize();
    println!("{}", sum);
}
