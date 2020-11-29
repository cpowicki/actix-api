use serde::Deserialize;

#[derive(Deserialize)]
pub struct Topic {
    pub name: String,
    pub consumer_count: usize,
}

#[derive(Deserialize)]
pub struct SendMessage {
    pub topic: String,
    pub data: String,
}
