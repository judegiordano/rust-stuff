#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    pub fn width(&self) -> bool {
        self.width > 0
    }

    #[allow(dead_code)]
    pub fn height(&self) -> bool {
        self.height > 0
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

pub fn implementation() {
    let rectangle: Rectangle = Rectangle {
        width: 10,
        height: 20,
    };
    let area: u32 = rectangle.area();
    println!("area {:#?}", area);
    println!(
        "rectangle has a nonzero width: {:#?}, width is {}",
        rectangle.width(),
        rectangle.width
    );

    let square: Rectangle = Rectangle::square(5);
    println!("square {:#?}", square);
    println!("square area {:#?}", square.area());
}
