use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Foo {
    pub id: String,
    pub message: String,
}
