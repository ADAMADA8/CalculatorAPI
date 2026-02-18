use axum::extract::RawQuery;
use axum::http::StatusCode;
use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let app_routes = Router::new().route("/plus", get(plus));
    let app = Router::new().nest("/api/v1", app_routes);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn plus(RawQuery(query): RawQuery) -> Result<String, StatusCode> {
    let query = query.unwrap_or_default();

    let numbers: Result<Vec<i64>, _> = query.split(',').map(|s| s.parse::<i64>()).collect();

    let nums = numbers.map_err(|_| StatusCode::BAD_REQUEST)?;
    let sum: i64 = nums.iter().sum();
    Ok(sum.to_string())
}
