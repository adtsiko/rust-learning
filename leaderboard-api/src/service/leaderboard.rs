use std::io::Result;

use crate::models::Score;
use crate::repository::ScoreRepository;

#[derive(Clone)]
pub struct LeaderboardService {
    repo: ScoreRepository
}

impl LeaderboardService{

    pub fn new(repo: ScoreRepository) -> Self {
        Self{repo}
    }

    pub fn add_score(&self, user: String, score: u32) -> Result<Score> {
        let mut scores = self.repo.load_all()?;

        let new_score = Score::new(user, score);
        
        scores.push(new_score.clone());

        self.repo.save_all(&scores)?;
        Ok(new_score)

    }

    pub fn top_scores(&self, limit: usize) -> Result<Vec<Score>> {
        let mut scores = self.repo.load_all()?;
        println!("{:?}", scores);
        scores.sort_by(|a, b| b.score.cmp(&a.score));
        scores.truncate(limit);
        Ok(scores)
    }

    pub fn list_scores(&self) -> Result<Vec<Score>> {
        self.repo.load_all()
    }
}