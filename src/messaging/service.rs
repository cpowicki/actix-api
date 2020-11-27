use std::{collections::HashMap, path::PathBuf};

use super::{api::Message, topic::Topic};
use actix_web::error::{ErrorInternalServerError, ErrorNotFound, Result};
use tokio::sync::mpsc::{channel, Sender};

use anyhow::anyhow;

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
    
    // Lists all available topics
    pub fn list_topics(&self) -> Vec<&String> {
        self.topics.keys().collect()
    }

    // Add a topic
    pub fn register_topic(&mut self, name: String) {
        let (tx_msg, rx_msg) = channel::<Message>(100);

        let mut topic = Topic::new(name.clone(), self.db.clone(), rx_msg);
        topic.add_consumer();
        topic.init();

        self.topics.insert(name, tx_msg);
    }

    // Send message to consumers of a topic
    pub async fn send_message(&mut self, topic: String, msg: Message) -> Result<()> {
        match self.topics.get_mut(&topic) {
            Some(topic) => topic.send(msg).await.map_err(|_| ErrorInternalServerError(anyhow!("Failed to send message"))),
            None => Err(ErrorNotFound(anyhow!("No topic found")))
        }
    }
}
