use crate::helpers::type_of;

#[derive(Debug)]
enum IpAddrType {
    V4,
    V6,
}

#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

#[derive(Debug)]
enum IpAddrTuple {
    V4(u8, u8, u8, u8),
    #[allow(dead_code)]
    V6(String),
}

pub fn enums() {
    let ip_v4 = IpAddrType::V4;
    let ip_v6 = IpAddrType::V6;
    println!("{:#?}", type_of(&ip_v4));
    println!("{:#?}", ip_v6);
    // enums with values
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));
    println!("home {:#?}", home);
    println!("loopback {:#?}", loopback);
    // enum tuple
    let home = IpAddrTuple::V4(127, 0, 0, 0);
    println!("home {:#?}", home);
    // Options
    some_none();
    // match enum
    let value = value_in_cents(&Coin::Dime);
    println!("dime value {:#?}", value);
    // none matching
    let not_null = none_matching(Some(&1));
    let null = none_matching(None);
    println!("not_null {:#?}", not_null);
    println!("null {:#?}", null);
    // catch all matching
    let roll_one = catch_all(1);
    let roll_two = catch_all(2);
    let roll_three = catch_all(3);
    println!("first roll rarity: {:#?}", roll_one);
    println!("second roll rarity: {:#?}", roll_two);
    println!("third roll rarity: {:#?}", roll_three);
}

pub fn some_none() {
    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;
    println!("some_number {:#?}", some_number);
    println!("some_string {:#?}", some_string);
    println!("absent_number {:#?}", absent_number);
}

#[derive(Debug)]
pub enum Coin {
    #[allow(dead_code)]
    Penny,
    #[allow(dead_code)]
    Nickel,
    Dime,
    #[allow(dead_code)]
    Quarter,
}

pub fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

pub fn none_matching(value: Option<&u8>) -> Option<u8> {
    match value {
        None => None,
        Some(n) => Some(n + 1),
    }
}

#[derive(Debug)]
pub enum Rarity {
    Common,
    Rare,
}

pub fn catch_all(roll: u8) -> Rarity {
    match roll {
        1 => Rarity::Rare,
        _ => Rarity::Common,
    }
}
