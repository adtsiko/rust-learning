use std::fs::OpenOptions;
use std::io::Write;
use std::io::Result;
use std::path::Path;
use std::fs;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Score  {
    user: String,
    score: u32,
}

impl Score {
    pub fn new(user: String, score: u32) -> Self {
        Self{
            user, score
        }
    }
}


const SCORES_FILE: &str = "scores.json";

pub fn save_scores(score: Score) -> Result<()> {
    
    let mut scores: Vec<Score> = load_scores()?;
    scores.push(score);

    let json = serde_json::to_string_pretty(&scores)?;

    fs::write(SCORES_FILE, json)?;

    Ok(())

}

fn load_scores() -> Result<Vec<Score>> {
    if !Path::new(SCORES_FILE).exists() {
        init_scores_file()?;
    }

    let contents = fs::read_to_string(SCORES_FILE)?;

    serde_json::from_str(&contents)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
}

fn init_scores_file() -> Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(SCORES_FILE)?;
    
    file.write_all(b"[]")?;
    Ok(())
}

pub fn list_scores() -> Result<()> {
    let scores = load_scores()?;
    for score in scores {
        println!("{} : {}", score.user, score.score);
    }

    Ok(())
}

pub fn top_scores(limit: u8) -> Result<()> {
    let mut scores = load_scores()?;

    scores.sort_by_key(|s| s.score);

    for score in scores.iter().take(limit.into()) {
        println!("{} : {}", score.user, score.score);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn score_new_sets_user_and_score() {
        let score = Score::new(String::from("Anesu"), 40);

        assert_eq!(score.user, "Anesu");
        assert_eq!(score.score, 40);
    }
}