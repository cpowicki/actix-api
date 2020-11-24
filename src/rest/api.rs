use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateTopic {
  pub name : String
}

#[derive(Deserialize)]
pub struct SendMessage {
  topic: String,
  data: String
}

impl SendMessage {
  pub fn get_topic(&self) -> &String {
    &self.topic
  }

  pub fn get_data(&self) -> &String {
    &self.data
  }
}