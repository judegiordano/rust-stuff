use crate::utils::helpers::*;

pub fn types() {
    // start as string
    let transform_me = "42";
    println!("current type: {}", type_of(&transform_me));
    // parse to unsigned 32-bit
    let transform_me: u32 = transform_me.parse().expect("not a number");
    println!("current type: {}", type_of(&transform_me));
    // parse to bool
    let transform_me = true;
    println!("current type: {}", type_of(&transform_me));
    // readable unsigned 64
    let transform_me: u64 = 32_000_000_000;
    println!("current type: {}", type_of(&transform_me));
    // float 64
    // on most modern machines, f64 is about as fast as f32, but with more precision
    let transform_me: f64 = 420.69;
    println!("current type: {}", type_of(&transform_me));
    // char type uses single quotes
    let transform_me: char = 'a';
    println!("current type: {}", type_of(&transform_me));
    let transform_me: char = '🚀';
    println!("current type: {}", type_of(&transform_me));
    // tuple
    let transform_me: (u32, f64, u8) = (500, 6.4, 1);
    println!("current type: {}", type_of(&transform_me));
    // destructure tuple syntax
    let (x, y, z) = transform_me;
    print!("tuple values {x}, ");
    print!("{y}, ");
    print!("{z}");
    println!();
    // access index of tuple
    println!("tuple index 0{}", transform_me.0);
    // array: [type; size]
    // must have same type, have fixed length
    let transform_me: [i32; 5] = [1, 2, 3, 4, 5];
    println!("current type: {}", type_of(&transform_me));
    // can auto set values and size of array
    let transform_me: [i32; 20] = [3; 20];
    for i in transform_me {
        print!("{} ", i);
    }
}
