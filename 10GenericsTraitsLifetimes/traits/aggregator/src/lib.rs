use std::{
    clone,
    fmt::{Debug, Display},
};

pub trait Summary {
    // Traits can both declare and implement default functionality.
    //fn summarize(&self) -> String;
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
    // ! default functionality can be overriden in type implementation
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    // no need for method implementation if the default functionality is desired
    fn summarize_author(&self) -> String {
        format!("Author: {}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// allows for parameters with different types, implementing Summary
pub fn notify(item: &impl Summary) {
    println!("Breaking News! {}", item.summarize());
}

// restricts parameters to the same type
pub fn notify1<T: Summary>(item1: &T, item2: &T) {}

//parameters should implement both traits
pub fn notify2(item: &(impl Summary + Display)) {}

// mix of notify2 and notify3
pub fn notify3<T: Summary + Display>(item: T) {}

// Long trait bounds
pub fn some_function1<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    unimplemented!()
}

//Using {where}
fn some_function2<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    unimplemented!()
}

//---------------
// Returning types implementing traits. Only 1 type!
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}
