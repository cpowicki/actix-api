use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateTopic {
    name: String,
}

impl CreateTopic {
    pub fn get_name(&self) -> &String {
        &self.name
    }
}

#[derive(Deserialize)]
pub struct SendMessage {
    topic: String,
    data: String,
}

impl SendMessage {
    pub fn get_topic(&self) -> &String {
        &self.topic
    }

    pub fn get_data(&self) -> &String {
        &self.data
    }
}
