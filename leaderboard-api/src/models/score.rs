use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Score {
    pub user: String,
    pub score: u32
}

impl Score {
    pub fn new(user: String, score: u32) -> Self {
        Self {
            user,
            score
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct AddScoreRequest {
    pub user: String,
    pub score: u32
}