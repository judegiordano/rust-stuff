pub fn example() {
    let v1: Vec<u32> = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let v1_ter2 = v1.iter();
    for i in v1_iter {
        println!("{:#?}", i);
    }
    let sum: u32 = v1_ter2.sum();
    println!("array sum {:#?}", sum);

    let incremented_arr: Vec<u32> = v1.iter().map(|a| a + 1).collect();
    println!("{:#?}", incremented_arr);
    names();

    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];

    let in_my_size = shoes_in_size(shoes, 10);
    println!("{:#?}", in_my_size);
}

fn names() {
    let names: Vec<&str> = vec!["tim", "bob", "dan"];
    let mut normalized: Vec<String> = names
        .iter()
        .map(|a| format!("hello, my name is: {}", *a))
        .collect();

    normalized.sort();
    println!("{:#?}", normalized);
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes
        .into_iter()
        .filter(|s: &Shoe| s.size == shoe_size)
        .collect()
}

#[derive(PartialEq, Debug)]
struct Shoe {
    pub size: u32,
    pub style: String,
}
