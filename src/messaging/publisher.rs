use std::collections::HashMap;

use super::{api::Message, topic::Topic};
use tokio::{sync::mpsc, sync::mpsc::Receiver, sync::mpsc::Sender, task::JoinHandle};

use anyhow::Result;
use anyhow::{anyhow, Context};
pub struct PublisherService {
    topics: HashMap<String, Sender<Message>>,
}

impl PublisherService {
    pub fn new() -> Self {
      PublisherService {
            topics: HashMap::new(),
        }
    }

    pub fn list_topics(&self) -> Vec<&String> {
        self.topics.keys().collect()
    }

    pub fn register_topic(&mut self, name: String) {
        let (tx_msg, rx_msg) = mpsc::channel::<Message>(100);

        let mut topic = Topic::new(name.clone(), rx_msg);
        topic.add_consumer();
        topic.init();

        self.topics.insert(name, tx_msg);
    }

    pub async fn send_message(&mut self, topic: String, msg: Message) -> Result<()> {
        self.topics
            .get_mut(&topic)
            .unwrap()
            .send(msg)
            .await
            .map_err(|_| anyhow!("Failed to send message"))
    }
}
