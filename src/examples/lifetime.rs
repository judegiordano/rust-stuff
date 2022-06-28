pub fn example() {
    let res = longest("apple", "elephant");
    println!("{:#?}", res);

    // told to live the duration of the program
    let s: &'static str = "I have a static lifetime.";
    println!("{:#?}", s);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
