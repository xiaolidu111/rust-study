use std::{
    fmt::{Debug, Display},
    string,
};

pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
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

// pub fn notify1(item: impl Summary, item2: impl Summary) {
//     println!("breaking news! {}", item.summarize())
// }
pub fn notify1(item: impl Summary + Display) {
    println!("breaking news! {}", item.summarize())
}

pub fn notify2<T: Summary + Display>(item: T) {
    println!("breaking news! {}", item.summarize())
}

pub fn notify3<T: Summary + Display, U: Clone + Debug>(item1: T, item2: U) -> String {
    format!("breaking news {}", item1.summarize())
}
pub fn notify4<T, U>(item1: T, item2: U) -> String
where
    T: Summary + Display,
    U: Clone + Debug,
{
    format!("breaking news {}", item1.summarize())
}
