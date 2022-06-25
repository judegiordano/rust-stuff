pub fn flow() {
    flow_38();
    if_let();
}

#[allow(clippy::single_match)]
fn flow_38() {
    let config = Some(3u8);
    match config {
        Some(max) => println!("max is {:#?}", max),
        _ => (),
    }
}

fn if_let() {
    let config = Some(3u8);
    if let Some(value) = config {
        println!("max is {:#?}", value);
    } else {
        println!("max is {:#?}", config);
    }
}
