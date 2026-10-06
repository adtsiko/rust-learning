use axum::{routing::{get, post}, Router};

use crate::handlers::score::{add_score, list_scores, top_scores};
use crate::service::LeaderboardService;

pub fn create_router(service: LeaderboardService) -> Router {
    Router::new()
        .route("/scores", post(add_score).get(list_scores))
        .route("/scores/top", get(top_scores))
        .with_state(service)
}