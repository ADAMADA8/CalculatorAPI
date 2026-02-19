use crate::handlers::{subtract, add, multiply, divide};
use axum::Router;
use axum::routing::get;

pub(crate) fn app_router() -> Router {
    let app_routes = Router::new()
        .route("/add", get(add))
        .route("/subtract", get(subtract))
        .route("/multiply", get(multiply))
        .route("/divide", get(divide));
    Router::new().nest("/api/v1", app_routes)
}
