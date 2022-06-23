pub fn logic() {
    fizz_buzz();
    let_if();
    loop_label();
    while_loop();
    reverse();
}

pub fn fizz_buzz() {
    for i in 1..=100 {
        if i % 15 == 0 {
            println!("fizzbuzz");
            continue;
        }
        if i % 3 == 0 {
            println!("fizz");
            continue;
        }
        if i % 5 == 0 {
            println!("buzz");
            continue;
        }
        println!("{i}");
    }
}

pub fn let_if() {
    let condition: bool = true;
    let var: &str = if condition { "so true" } else { "so false" };
    println!("{var}");
}

pub fn loop_label() {
    let mut count: u8 = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining: u8 = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);
}

pub fn while_loop() {
    let mut counter: u32 = 0;
    while counter <= 10 {
        println!("{counter}");
        counter += 1;
    }
}

pub fn reverse() {
    for i in (1..=3).rev() {
        println!("{i}");
    }
}
