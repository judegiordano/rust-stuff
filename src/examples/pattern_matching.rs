use chrono::offset::Local;
use chrono::{self, Datelike, Weekday};

pub fn example() {
    fav_color();
    conditional_loop();
    match_literal();
    match_name()
}

pub fn fav_color() {
    let favorite_color: Option<&str> = None;
    let is_tuesday: bool = Local::now().weekday() == Weekday::Tue;
    let age: Result<u8, _> = "34".parse();
    if let Some(color) = favorite_color {
        println!("using your favorite color {:#?} as the background", color);
    } else {
        println!("no favorite color give :(")
    }
    if is_tuesday {
        println!("today is tuesday!")
    } else {
        println!("today is not tuesday")
    }
    if let Ok(age) = age {
        println!("your age is: {:#?}", age);
    } else {
        println!("error parsing age");
    }
}

pub fn conditional_loop() {
    let mut stack: Vec<u32> = vec![];
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{:#?}", top);
    }

    let v: Vec<char> = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{:#?} at index {:#?}", value, index);
    }
}

pub fn match_literal() {
    let x = 1;
    match x {
        1 => println!("value is one"),
        2 => println!("value is two"),
        3 => println!("value is three"),
        _ => println!("value is something else"),
    }
}

pub fn match_name() {}
