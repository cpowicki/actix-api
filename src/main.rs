use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use std::time::Duration;

mod messaging;
use messaging::api::{Data, Message, Publisher};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut publisher = Publisher::new();
    publisher.register_topic("test".to_owned());
    publisher.send_message("test".to_owned(), Message::Clear).await;

    HttpServer::new(|| App::new().service(hello))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
