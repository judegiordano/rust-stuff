use serde::{Deserialize, Serialize};

use crate::utils::http_client::Request;

#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    #[serde(rename = "userId")]
    user_id: u32,
    id: Option<u32>,
    title: String,
    completed: bool,
}

pub async fn get_todos() -> Result<(), reqwest::Error> {
    let mut client = Request {
        base_url: "https://jsonplaceholder.typicode.com",
    };
    let response: Vec<Todo> = client.get(Some("todos")).await?;
    println!("{:#?}", response);
    Ok(())
}
