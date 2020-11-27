use std::{collections::HashMap, path::PathBuf};

use super::{api::Message, topic::Topic};
use tokio::{sync::mpsc, sync::mpsc::Receiver, sync::mpsc::Sender, task::JoinHandle};

use anyhow::Result;
use anyhow::{anyhow};
pub struct MessengerService {
    db: PathBuf,
    topics: HashMap<String, Sender<Message>>,
}

impl MessengerService {
    pub fn new(db: PathBuf) -> Self {
        MessengerService {
            db,
            topics: HashMap::new(),
        }
    }

    pub fn list_topics(&self) -> Vec<&String> {
        self.topics.keys().collect()
    }

    pub fn register_topic(&mut self, name: String) {
        let (tx_msg, rx_msg) = mpsc::channel::<Message>(100);

        let mut topic = Topic::new(name.clone(), self.db.clone(), rx_msg);
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
