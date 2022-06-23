// each value has an owner
// only one owner at a time
// when owner goes out of scope, value is dropped
// rust will never automatically create deep copies
pub fn owner() {
    let expensive_string: String = String::from("im a string");
    // manually deep copy heap data
    let copy: String = expensive_string.clone();
    println!("{:#?}, {:#?}", expensive_string, copy);
}
