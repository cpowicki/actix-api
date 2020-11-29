use std::path::PathBuf;

use super::{
    api::Message,
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
    pub fn new(name: String, db_root: PathBuf, rx: Receiver<TopicOperation>) -> Self {
        let file_name = format!("{}.txt", name);

        let mut db_path = db_root.to_owned();
        db_path.push(file_name);

        Topic {
            name,
            db: db_path,
            index: 0,
            consumers: vec![],
            rx,
        }
    }

    fn add_consumer(&mut self) {
        let (tx, rx) = mpsc::channel::<Message>(100);
        let id = self.consumers.len() as u32;
        let handle = Consumer::new(id, self.db.clone(), rx).init();

        self.consumers.push(ConsumerHandle::new(tx, handle));
    }

    async fn send(&mut self, msg: Message) -> Result<()> {
        if self.consumers.len() == 0 {
            return Ok(());
        }

        self.index = self.index % (self.consumers.len());

        match self.consumers.get_mut(self.index) {
            Some(consumer) => {
                consumer.send(msg).await?;
                self.index += 1;
                Ok(())
            }
            None => Err(anyhow!("Failed to send message")),
        }
    }

    pub fn init(mut self) -> JoinHandle<Topic> {
        tokio::spawn(async move {
            if let Err(_) = tokio::fs::File::create(self.db.clone()).await {
                panic!("Failed to create file!"); // TODO handle better
            }

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
                    TopicOperation::KillConsumer(id) => {
                        let handle = self.consumers.remove(id as usize);
                        if let Ok(consumer) = handle.kill().await {
                            println!(
                                "Killed consumer {:?}, sent {:?}",
                                id,
                                consumer.get_msg_count()
                            )
                        }
                    }
                }
            }

            self
        })
    }
}
