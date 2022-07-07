use std::io::Result;
use std::time::{Duration, Instant};

pub mod examples;
pub mod grep;
pub mod tests;
pub mod utils;

pub use crate::examples::*;
pub use crate::utils::config::Config;
pub use crate::utils::*;

#[tokio::main]
async fn main() -> Result<()> {
    // let configuration: Config = Config::new().expect("error parsing environment variables");
    let start: Instant = Instant::now();
    // range::range();
    // shadowing::shadow();
    // game::init_game();
    // math::operations();
    // data_types::types();
    // functions::top_level();
    // control_flow::logic();
    // ownership::owner();
    // structs::structuring();
    // methods::implementation();
    // enums::enums();
    // if_let_control_flow::flow();
    // vectors::example();
    // string::example();
    // hash_maps::example();
    // panic::example();
    // traits_generics::example();
    // lifetime::example();
    // grep::tool::grep_cli();
    // closure::example();
    // iterations::example();
    // smart_pointers::example();
    // drop_trait::example();
    // threading::example();
    // channels::example();
    mutex::example();
    // request::get_todos().await?;

    let duration: Duration = start.elapsed();
    println!("operation complete in {:#?}", duration);
    Ok(())
}
