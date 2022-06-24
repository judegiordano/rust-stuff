use std::time::{Duration, Instant};

pub mod examples;
pub mod utils;

pub use crate::examples::*;
pub use crate::utils::*;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
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
    methods::implementation();
    // request::get_todos().await?;

    let duration: Duration = start.elapsed();
    println!("operation complete in {:#?}", duration);
    Ok(())
}
