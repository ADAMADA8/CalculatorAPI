use crate::handlers::{minus, plus};
use axum::Router;
use axum::routing::get;

pub(crate) fn app_router() -> Router {
    let app_routes = Router::new()
        .route("/plus", get(plus))
        .route("/minus", get(minus));
    Router::new().nest("/api/v1", app_routes)
}
