use serde::{Deserialize, Serialize};

pub fn example() {
    let list = vec![10, 100, 34, 67, 101];
    let answer = largest_abstraction(&list);
    println!("{:#?}", answer);

    let char_list = vec!['y', 'm', 'a', 'q', 'z'];
    let result = largest_abstraction(&char_list);
    println!("The largest char is {}", result);

    let tweet = Tweet::new(
        "horse_ebooks",
        "of course, as you probably already know, people",
    );
    println!("{:#?}", tweet);
    println!("{:#?}", tweet.summarize());
    println!("{:#?}", tweet.summarize_author());
    notify(&tweet);
}

fn largest_abstraction<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest_num = &list[0];
    for i in list {
        if i > largest_num {
            largest_num = &i;
        }
    }
    largest_num
}

#[allow(dead_code)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    #[allow(dead_code)]
    pub fn x(&self) -> &T {
        &self.x
    }
}

#[allow(dead_code)]
struct MultiPoint<T, U> {
    x: T,
    y: U,
}

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
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

    fn summarize_author(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Tweet {
    pub fn new(username: &str, content: &str) -> Tweet {
        Tweet {
            username: String::from(username),
            content: String::from(content),
            reply: false,
            retweet: false,
        }
    }
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
