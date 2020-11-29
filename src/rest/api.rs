use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateTopic {
    pub name: String,
}

#[derive(Deserialize)]
pub struct SendMessage {
    pub topic: String,
    pub data: String,
}
