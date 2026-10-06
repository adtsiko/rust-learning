/*

Structure
src/
    handlers
        service endpoint function mapping to endpoint with success and error paths
    models
        scores and score requests models, make sure the models are serialize and desirable
    repository
        
    routes
    service
    lib.rs
    main.rd
*/

use std::net::SocketAddr;

use leaderboard_api::routes;
use leaderboard_api::repository::ScoreRepository;
use leaderboard_api::service::LeaderboardService;

#[tokio::main]
async fn main() {
    let repo = ScoreRepository::new("scores.json".to_string());
    let service = LeaderboardService::new(repo);
    let app = routes::create_router(service);

    let addr = SocketAddr::from(([127,0, 0 , 1], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("listinening on {}", addr);

    axum::serve(listener, app).await.unwrap();
}
