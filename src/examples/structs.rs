use crate::helpers::type_of;

#[derive(Debug)]
pub struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[derive(Debug)]
pub struct Color(u32, u32, u32);

#[derive(Debug)]
pub struct Point(u32, u32, u32);

#[derive(Debug)]
pub struct Dimensions(u32, u32);

#[derive(Debug)]
pub struct Rectangle {
    width: u32,
    height: u32,
}

#[derive(Debug)]
struct AlwaysEqual;

pub fn structuring() {
    let mut user: User = User {
        email: String::from("someone@example.com"),
        username: String::from("jude_boy"),
        active: true,
        sign_in_count: 1,
    };
    println!("type of user {:#?}", type_of(&user));
    println!("user {:#?}", user);
    println!("user.active {:#?}", user.active);
    println!("user.username {:#?}", user.username);
    println!("user.email {:#?}", user.email);
    println!("user.sign_in_count {:#?}", user.sign_in_count);
    // mutate mutable struct
    user.username = String::from("new_username");
    println!("mutated user {:#?}", user);
    let user_two = build_user("a_user".to_string(), "mail@mail.com".to_string());
    println!("user two {:#?}", user_two);
    let copy = user;
    spread_syntax(copy);
    // println!("{:#?}", user); original now inaccessible
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("{:#?} {:#?}", black, origin);

    let unit_like_struct = AlwaysEqual;

    println!("unit like struct {:#?}", unit_like_struct);
    let rectangle: Dimensions = Dimensions(10, 20);
    let rectangle_area = area(&rectangle);
    println!("area {:#?}", rectangle_area);

    let rectangle = Rectangle {
        width: 30,
        height: 50,
    };
    let rec_area = rect_area(&rectangle);
    println!("rect area {:#?}", rec_area);
}

pub fn build_user(username: String, email: String) -> User {
    // init field shorthand
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

pub fn spread_syntax(user: User) {
    let spread_user = User {
        username: "override_username".to_string(),
        // we now lose ownership of original user
        ..user
    };
    println!("spread user: {:#?}", spread_user);
}

pub fn area(dimensions: &Dimensions) -> u32 {
    dimensions.0 * dimensions.1
}

pub fn rect_area(dimensions: &Rectangle) -> u32 {
    dimensions.width * dimensions.height
}
