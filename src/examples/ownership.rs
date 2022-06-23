// each value has an owner
// only one owner at a time
// when owner goes out of scope, value is dropped
// rust will never automatically create deep copies
pub fn owner() {
    deep_clone();
    referencing();
    mutable_referencing();
    slice_me();
}

fn deep_clone() {
    let expensive_string: String = String::from("im a string");
    // manually deep copy heap data
    let copy: String = expensive_string.clone();
    println!("{:#?}, {:#?}", expensive_string, copy);
}

fn referencing() {
    let expensive_string: String = String::from("sup");
    // point to reference of expensive string owner
    let len: usize = calculated_len(&expensive_string);
    println!("{:#?}", len);
    println!(
        "we still have ownership of this string: {:#?}",
        expensive_string
    );
}

fn calculated_len(s: &String) -> usize {
    // borrow
    s.len()
}

fn mutable_referencing() {
    let mut expensive_string: String = String::from("hello");
    println!("{:#?}", expensive_string);
    // can only have ONE mutable reference to a piece of data at a time
    change(&mut expensive_string);
    println!("{:#?}", expensive_string);

    let ref_one = &expensive_string;
    let ref_two = &expensive_string;
    println!("{}, {}", ref_one, ref_two);
    // this is fine because the final mutable reference
    // is used after the previous references are done
    let ref_three = &mut expensive_string;
    println!("{}", ref_three);
}

fn change(s: &mut String) {
    s.push_str(", world")
}

fn slice_me() {
    let mut input: String = String::from("hello world");
    let word = first_word(&input);
    println!("first word: {:#?}", word);
    input.clear();
    slice_word();
    slice_arr();
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

fn slice_word() {
    let s = String::from("hello world");
    let hello = &s[..5];
    let world = &s[6..];
    let whole_word = &s[..];
    println!("{:#?}", hello);
    println!("{:#?}", world);
    println!("{:#?}", whole_word);
}

fn slice_arr() {
    let arr = [1, 2, 3, 4, 5];
    println!("{:#?}", &arr[1..3]);
    assert_eq!(&arr[1..3], &[2, 3]);
}
