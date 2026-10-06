use axum::{extract::Query, http::StatusCode, Json};
use serde::Deserialize;

use crate::models::{AddScoreRequest, Score};
use crate::service::LeaderboardService;

pub async fn add_score(
    axum::extract::State(service): axum::extract::State<LeaderboardService>,
    Json(body): Json<AddScoreRequest>,
) -> Result<Json<Score>, StatusCode> {
    service
        .add_score(body.user, body.score)
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn list_scores(
    axum::extract::State(service): axum::extract::State<LeaderboardService>,
) -> Result<Json<Vec<Score>>, StatusCode> {
    service
        .list_scores()
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

#[derive(Deserialize)]
pub struct TopQuery {
    limit: Option<usize>,
}

pub async fn top_scores(
    axum::extract::State(service): axum::extract::State<LeaderboardService>,
    Query(params): Query<TopQuery>,
) -> Result<Json<Vec<Score>>, StatusCode> {
    let limit = params.limit.unwrap_or(10);
    service
        .top_scores(limit)
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR) 
}