use actix_web::{get, post, web::Data, web::Json, web::ServiceConfig, HttpResponse, Responder};

use super::api::{CreateTopic, SendMessage};
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

    lock.register_topic(topic.into_inner().name);
    HttpResponse::Ok()
}

#[post("/message")]
async fn post_msg(
    json: Json<SendMessage>,
    publisher: Data<RwLock<MessengerService>>,
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
        Err(e) => e.as_response_error().error_response().into(),
    }
}

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(get_topics);
    cfg.service(post_topic);
    cfg.service(post_msg);
}
