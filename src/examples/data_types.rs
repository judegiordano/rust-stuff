use crate::utils::helpers::*;

pub fn types() {
    // start as string
    let transform_me: &str = "42";
    println!("current type: {}", type_of(&transform_me));
    // parse to unsigned 32-bit
    let transform_me: u32 = transform_me.parse().expect("not a number");
    println!("current type: {}", type_of(&transform_me));
    // parse to bool
    let transform_me: bool = true;
    println!("current type: {}", type_of(&transform_me));
    // readable unsigned 64
    let transform_me: u64 = 32_000_000_000;
    println!("current type: {}", type_of(&transform_me));
    // float 64
    let transform_me = 420.69;
    println!("current type: {}", type_of(&transform_me));
}
