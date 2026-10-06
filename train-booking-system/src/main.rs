use std::net::SocketAddr;

use train_booking_system::routes;
use train_booking_system::repository::TrainCompanyFileRepository;
use train_booking_system::service::TrainService;

#[tokio::main]
async fn main() {

    let repo = TrainCompanyFileRepository::new("train.json".to_string());
    let service = TrainService::new(repo);
    let app = routes::create_router(service);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("listening on {}", addr);

    axum::serve(listener, app).await.unwrap();    

}