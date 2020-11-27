use actix_web::{web::Data, App, HttpServer};
use std::path::PathBuf;
use anyhow::Context;
use tokio::sync::RwLock;

mod messaging;
mod rest;

use messaging::service::MessengerService;


#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let db_dir = create_db().await?;
    let publisher_service = create_publisher_service(db_dir.clone());

    HttpServer::new(move || {
        App::new()
            .app_data(publisher_service.clone())
            .configure(rest::routes::init)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
    .context("Actix Web server failed unexpectedly")
}

fn create_publisher_service(db: PathBuf) -> Data<RwLock<MessengerService>> {
    Data::new(RwLock::new(MessengerService::new(db)))
}

async fn create_db() -> anyhow::Result<PathBuf> {
    let mut dir = std::env::current_dir().context("Failed to get current dir")?;
    dir.push("persistence");

    if !dir.exists() {
        tokio::fs::create_dir(dir.clone())
            .await
            .context("Failed to create persistence dir")?;
    }

    dir.push("data.txt");
    tokio::fs::File::create(dir.clone()).await?;

    Ok(dir)
}
