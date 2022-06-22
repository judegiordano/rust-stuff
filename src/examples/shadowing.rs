pub fn shadow() {
    let x: u32 = 5;

    // shadowed variable
    let x: u32 = x + 1;

    {
        let x: u32 = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);

    let spaces: &str = "     ";
    let spaces: usize = spaces.len();
    print!("spaces ({spaces}) is now a number");
}
