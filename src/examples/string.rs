pub fn example() {
    let data: &str = "initial contents";
    let s: String = data.to_string();
    println!("{:#?}", s);
    // also valid
    let s = "initial contents".to_string();
    println!("{:#?}", s);
    // also valid
    let s = String::from("initial contents");
    println!("{:#?}", s);
    // also valid
    let s = String::from("🚀");
    println!("{:#?}", s);

    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{:#?}", s);

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);

    // push char
    let mut s = String::from("lo");
    s.push('l');
    println!("{:#?}", s);
    // format macro
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{:#?}", s);

    let num_string = "0123456789";
    let zero_string = &num_string[..=0];
    println!("first char {:#?}", zero_string);
    for char in num_string.chars() {
        println!("{:#?}", char);
    }
    for byte in num_string.bytes() {
        println!("{:#?}", byte);
    }
}
