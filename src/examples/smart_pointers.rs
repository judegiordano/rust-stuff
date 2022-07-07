use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn example() {
    boxes();
    pointer();
    let m: MyBox<String> = MyBox::new(String::from("Rust"));
    hello(&m);
}

pub fn boxes() {
    let x: u32 = 5;
    let y: Box<u32> = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

pub fn pointer() {
    let x: u32 = 5;
    let y: &u32 = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

pub fn my_struct() {
    let x: u32 = 5;
    let y: MyBox<u32> = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

pub fn hello(name: &str) {
    println!("Hello, {}!", name);
}
