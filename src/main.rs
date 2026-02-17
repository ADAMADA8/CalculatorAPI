use axum::{
    routing::get,
    Router,
};
use rand::{rng, RngExt};

#[tokio::main]
async fn main() {
    let app_routes = Router::new().route("/random", get(random));
    let app = Router::new().nest("/api/v0", app_routes);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn random() -> String {
    rng().random_range(1..100).to_string()
}