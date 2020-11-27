use std::path::PathBuf;

use actix_web::{get, post, web::Data, web::Json, App, HttpResponse, HttpServer, Responder};
use anyhow::{Context, Result};
use rest::api::{CreateTopic, SendMessage};
use tokio::sync::RwLock;

mod messaging;
use messaging::{
    api::{Content, Message},
    publisher::PublisherService,
};

mod rest;

#[get("/topic")]
async fn get_topics(publisher: Data<RwLock<PublisherService>>) -> impl Responder {
    let lock = publisher.read().await;
    HttpResponse::Ok().json(lock.list_topics())
}

#[post("/topic")]
async fn post_topic(
    topic: Json<CreateTopic>,
    publisher: Data<RwLock<PublisherService>>,
) -> impl Responder {
    let mut lock = publisher.write().await;

    lock.register_topic(topic.into_inner().name);
    HttpResponse::Ok()
}

#[post("/message")]
async fn post_msg(
    json: Json<SendMessage>,
    publisher: Data<RwLock<PublisherService>>,
) -> impl Responder {
    let mut lock = publisher.write().await;

    let msg = json.into_inner();
    let data = msg.get_data().as_bytes().to_vec();
    let content = Content::new(data);

    match lock
        .send_message(msg.get_topic().to_owned(), Message::Write(content))
        .await
    {
        Ok(()) => HttpResponse::Accepted(),
        Err(_) => HttpResponse::InternalServerError(),
    }
}

#[actix_web::main]
async fn main() -> Result<()> {
    let db_dir = create_db().await?;
    let publisher_service = create_publisher_service(db_dir.clone());

    HttpServer::new(move || {
        App::new()
            .app_data(publisher_service.clone())
            .service(get_topics)
            .service(post_topic)
            .service(post_msg)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
    .context("Actix Web server failed unexpectedly")
}

fn create_publisher_service(db: PathBuf) -> Data<RwLock<PublisherService>> {
    Data::new(RwLock::new(PublisherService::new(db)))
}

async fn create_db() -> Result<PathBuf> {
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
