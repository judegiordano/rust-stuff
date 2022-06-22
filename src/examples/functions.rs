pub fn top_level() {
    hoist_me(69, '🚀', "yo");
    expression();
    let five = return_five();
    println!("{}", five);
}

fn hoist_me(x: u32, y: char, z: &str) {
    println!("variables: {} {} {}", x, y, z);
}

fn expression() {
    let x: u32 = {
        let y: u32 = 2;
        // expressions cannot end in semicolons!
        y + 1
    };
    println!("{}", x);
}

fn return_five() -> u8 {
    5
}
