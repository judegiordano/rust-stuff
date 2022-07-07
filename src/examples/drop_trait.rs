#[derive(Debug)]
struct CustomSmartPointer {
    pub data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("dropping CustomSmartPointer with data {:#?}", self.data);
    }
}

pub fn example() {
    let _c: CustomSmartPointer = CustomSmartPointer {
        data: String::from("my info"),
    };
    let _d: CustomSmartPointer = CustomSmartPointer {
        data: String::from("my other info"),
    };
    // data is dropped when it goes out of scope
    println!("pointers created.");
    // explicitly drop
    drop(_c);
}
