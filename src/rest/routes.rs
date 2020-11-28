use actix_web::{get, post, web::Data, web::Json, web::ServiceConfig, HttpResponse, Responder};

use super::api::{AddConsumer, CreateTopic, SendMessage};
use tokio::sync::RwLock;

use crate::messaging::{
    api::{Content, Message},
    service::MessengerService,
};

#[get("/topic")]
async fn get_topics(publisher: Data<RwLock<MessengerService>>) -> impl Responder {
    let lock = publisher.read().await;
    HttpResponse::Ok().json(lock.list_topics())
}

#[post("/topic")]
async fn post_topic(
    topic: Json<CreateTopic>,
    publisher: Data<RwLock<MessengerService>>,
) -> impl Responder {
    let mut lock = publisher.write().await;

    lock.register_topic(topic.into_inner().name.to_owned());
    HttpResponse::Ok()
}

#[post("/topic-consumer")]
async fn post_consumer(
    add_consumer: Json<AddConsumer>,
    publisher: Data<RwLock<MessengerService>>,
) -> impl Responder {
    let mut lock = publisher.write().await;
    match lock.add_consumer(add_consumer.topic.to_owned()).await {
        Ok(()) => HttpResponse::Accepted(),
        Err(e) => e.as_response_error().error_response().into(),
    }
}

#[post("/message")]
async fn post_msg(
    json: Json<SendMessage>,
    publisher: Data<RwLock<MessengerService>>,
) -> impl Responder {
    let mut lock = publisher.write().await;

    let msg = json.into_inner();
    let data = msg.data.as_bytes().to_vec();
    let content = Content::new(data);

    match lock
        .send_message(msg.topic.to_owned(), Message::Write(content))
        .await
    {
        Ok(()) => HttpResponse::Accepted(),
        Err(e) => e.as_response_error().error_response().into(),
    }
}

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(get_topics);
    cfg.service(post_topic);
    cfg.service(post_msg);
}
