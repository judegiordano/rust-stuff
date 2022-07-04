use serde::{Deserialize, Serialize};
use std::thread;
use std::time::Duration;

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
enum ShirtColor {
    Red,
    Blue,
}

#[derive(Debug, Deserialize, Serialize)]
struct Inventory {
    pub shirts: Vec<ShirtColor>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Inventory {
    pub fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    pub fn most_stocked(&self) -> ShirtColor {
        let mut num_red: u32 = 0;
        let mut num_blue: u32 = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            return ShirtColor::Red;
        }
        ShirtColor::Blue
    }
}

pub fn example() {
    let store: Inventory = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };
    let user_pref1: Option<ShirtColor> = Some(ShirtColor::Red);
    let giveaway1: ShirtColor = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2: Option<ShirtColor> = None;
    let giveaway2: ShirtColor = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    let _expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
    let _add_one_v4 = |x: u8| x + 1;
    let _add_one_v2 = |x: u32| -> u32 { x + 1 };
    let _add_one_v3 = |x: u8| x + 1;

    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];
    list.sort_by_key(|r: &Rectangle| r.width);
    println!("{:#?}", list);
}
