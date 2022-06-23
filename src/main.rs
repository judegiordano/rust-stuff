pub mod examples;
pub mod utils;

pub use crate::examples::*;
pub use crate::utils::*;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    // range::range();
    // shadowing::shadow();
    // game::init_game();
    // math::operations();
    // data_types::types();
    // functions::top_level();
    // control_flow::logic();
    // ownership::owner();
    request::get_todos().await?;
    Ok(())
}
