use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct Sentence {
    pub id: String,
    pub sentence : String
}