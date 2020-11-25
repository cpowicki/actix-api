use std::{collections::HashMap, path::PathBuf};

use super::{
    api::Message,
    consumer::{Consumer, ConsumerHandle},
};
use tokio::{sync::mpsc, sync::mpsc::Receiver, sync::mpsc::Sender, task::JoinHandle};

use anyhow::Result;
use anyhow::{anyhow, Context};

pub enum TopicMutation {
    AddConsumer(Consumer),
}

pub struct Topic {
    name: String,
    db: PathBuf,
    index: usize,
    rx: Receiver<Message>,
    consumers: Vec<ConsumerHandle>,
}

impl Topic {
    pub fn new(name: String, db: PathBuf, rx: Receiver<Message>) -> Self {
        Topic {
            name,
            db,
            index: 0,
            consumers: vec![],
            rx,
        }
    }

    pub fn add_consumer(&mut self) {
        let (tx, rx) = mpsc::channel::<Message>(100);

        let handle = Consumer::new(self.consumers.len() as u32, self.db.clone(), rx).init();

        self.consumers.push(ConsumerHandle::new(tx, handle));
    }

    pub async fn send(&mut self, msg: Message) -> Result<()> {
        match self.consumers.get_mut(self.index) {
            Some(consumer) => consumer.send(msg).await,
            None => Err(anyhow!("Failed to send message")),
        }
    }

    pub fn init(mut self) -> JoinHandle<Topic> {
        tokio::spawn(async move {
            while let Some(message) = self.rx.recv().await {
                self.index = self.index % (self.consumers.len());

                if let Err(e) = self.send(message).await {
                    println!("Failed to send message {:?}", e);
                }

                self.index += 1;
            }

            self
        })
    }
}
