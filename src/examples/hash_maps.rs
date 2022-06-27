use std::collections::HashMap;

pub fn example() {
    let mut scores: HashMap<String, u8> = HashMap::new();
    scores.insert(String::from("Team One"), 10);
    scores.insert(String::from("Team Two"), 10);
    println!("{:#?}", scores);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
    println!("{:#?}", scores);

    // [value; size]
    let arr_one: [u32; 20] = [0; 20];
    let arr_two: [u32; 20] = [1; 20];
    println!("{:#?}", arr_one);
    println!("{:#?}", arr_two);
    // will collect unique keys
    let combo: HashMap<_, _> = arr_one.iter().zip(arr_two.iter()).collect();
    println!("{:#?}", combo);

    // getting
    let team_name: String = String::from("Blue");
    let score = scores.get(&team_name).unwrap();
    println!("{:#?}", score);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // entry api
    scores.entry(String::from("Red")).or_insert(20);
    // this will do nothing because blue exists
    scores.entry(String::from("Blue")).or_insert(50);
    println!("scores {:#?}", scores);

    // text hash map
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
