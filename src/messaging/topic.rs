use std::path::PathBuf;

use super::{
    api::{Message},
    consumer::{Consumer, ConsumerHandle},
};
use tokio::{sync::mpsc, sync::mpsc::Receiver, task::JoinHandle};

use anyhow::anyhow;
use anyhow::Result;

pub enum TopicOperation {
    SendMessage(Message),
    AddConsumer,
    KillConsumer(u32),
}

pub struct Topic {
    name: String,
    db: PathBuf,
    index: usize,
    rx: Receiver<TopicOperation>,
    consumers: Vec<ConsumerHandle>,
}

impl Topic {
    pub fn new(name: String, db: PathBuf, rx: Receiver<TopicOperation>) -> Self {
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
        let id = self.consumers.len() as u32;
        let handle = Consumer::new(id, self.db.clone(), rx).init();

        self.consumers.push(ConsumerHandle::new(tx, handle));
    }

    async fn send(&mut self, msg: Message) -> Result<()> {
        match self.consumers.get_mut(self.index) {
            Some(consumer) => consumer.send(msg).await,
            None => Err(anyhow!("Failed to send message")),
        }
    }

    pub fn init(mut self) -> JoinHandle<Topic> {
        tokio::spawn(async move {
            while let Some(operation) = self.rx.recv().await {
                match operation {
                    TopicOperation::SendMessage(message) => {
                        self.index = self.index % (self.consumers.len());

                        if let Err(e) = self.send(message).await {
                            println!("Failed to send message {:?}", e);
                        }

                        self.index += 1;
                    }
                    TopicOperation::AddConsumer => self.add_consumer(),
                    TopicOperation::KillConsumer(_) => todo!(),
                }
            }

            self
        })
    }
}
