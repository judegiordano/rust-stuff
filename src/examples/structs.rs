use serde::{Deserialize, Serialize};

use crate::helpers::type_of;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Color(i32, i32, i32);

#[derive(Debug, Serialize, Deserialize)]
pub struct Point(i32, i32, i32);

#[derive(Debug, Serialize, Deserialize)]
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
        ..user
    };
    println!("spread user: {:#?}", spread_user);
}
