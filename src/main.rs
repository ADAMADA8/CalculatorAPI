mod handlers;
mod services;

use anyhow::Result;
use crate::services::app_router;

#[tokio::main]
async fn main() -> Result<()> {
    let app = app_router();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;
    Ok(())
}