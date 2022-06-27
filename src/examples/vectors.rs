use serde::{Deserialize, Serialize};

pub fn example() {
    let v: Vec<u32> = Vec::new();
    println!("{:#?}", v);
    let mut v: Vec<u32> = vec![1, 2, 3];
    println!("{:#?}", v);
    v.push(4);
    println!("{:#?}", v);
    let copy = &v[0];
    println!("second element: {:#?}", copy);

    match &v.get(0) {
        Some(value) => println!("found: {:#?}", value),
        None => println!("nothing found"),
    }

    for i in &v {
        println!("{:#?}", i);
    }

    let mut add_me: Vec<u32> = vec![1, 2, 3];
    for i in &mut add_me {
        *i += 10;
    }
    println!("{:#?}", add_me);
    add_me.pop();
    println!("{:#?}", add_me);

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("{:#?}", row);
}

#[derive(Debug, Deserialize, Serialize)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
