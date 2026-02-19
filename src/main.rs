mod handlers;
mod math;
mod services;

use crate::services::app_router;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let app = app_router();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;
    Ok(())
}
