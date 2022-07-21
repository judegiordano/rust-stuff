use chrono::offset::Local;
use chrono::{self, Datelike, Weekday};

pub fn example() {
    fav_color();
    conditional_loop();
    match_literal();
    match_name();
    multiple_patterns();
    match_range();
    match_string_range();
    destructure_struct();
    enum_destructuring();
    tuple_spread();
    fizz_buzz();
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
    let mut stack: Vec<u32> = vec![1, 2, 3];

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

pub fn match_name() {
    let x = Some(5);
    let _y = 10;

    match x {
        Some(50) => println!("got 50"),
        Some(_y) => println!("matches y {:#?}", _y),
        _ => println!("got alternative value, {:#?}", x),
    }
}

pub fn multiple_patterns() {
    let x = 1;
    match x {
        1 | 2 => println!("received one or two"),
        3 => println!("received 3"),
        _ => println!("received {:#?}", x),
    }
}

pub fn match_range() {
    let x = 5;
    match x {
        1..=5 => println!("range was in range 1-5"),
        _ => println!("value was {:#?}", x),
    }
}

pub fn match_string_range() {
    let x = 'c';
    match x {
        'a'..='j' => println!("letter was in range a-j"),
        'k'..='z' => println!("letter was in range k-z"),
        _ => println!("letter was {:#?}", x),
    }
}

struct Point {
    x: u32,
    y: u32,
}

pub fn destructure_struct() {
    let p = Point { x: 0, y: 7 };
    // destructure and rename fields
    let Point { x: a, y: b } = p;
    assert_eq!(a, 0);
    assert_eq!(b, 7);

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
}

#[allow(dead_code)]
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: u32, y: u32 },
    Write(String),
    ChangeColor(u32, u32, u32),
}

impl Message {
    fn command(&self) {
        match self {
            Message::Quit => println!("quit variant is void"),
            Message::Move { x, y } => println!("move x {} and y {}", x, y),
            Message::Write(text) => println!("text message {}", text),
            Message::ChangeColor(r, g, b) => {
                println!("change color to red {} green {} and blue {}", r, g, b)
            }
        }
    }
}

pub fn enum_destructuring() {
    let msg = Message::ChangeColor(1, 150, 255);
    msg.command();
    let msg = Message::Move { x: 2, y: 5 };
    msg.command();
}

pub fn tuple_spread() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("{}, {}", first, last);
        }
    }
}

pub fn fizz_buzz() {
    for i in 1..=100 {
        if i % 15 == 0 {
            println!("fizzbuzz");
            continue;
        }
        if i % 3 == 0 {
            println!("fizz");
            continue;
        }
        if i % 5 == 0 {
            println!("buzz");
            continue;
        }
        println!("{i}")
    }
}
