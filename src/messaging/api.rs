use std::collections::HashMap;

use tokio::{sync::mpsc, sync::mpsc::Receiver, sync::mpsc::Sender, task::JoinHandle};

use anyhow::anyhow;
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct Data {
    label: String,
    data: Vec<u8>,
}

impl Data {
    pub fn new(label: String, data: Vec<u8>) -> Self {
        Data { label, data }
    }
}

#[derive(Clone)]
pub enum Message {
    Write(Data),
    Clear,
}

pub enum TopicMutation {
    AddConsumer(Consumer),
}

pub struct Publisher {
    topics: HashMap<String, Sender<Message>>,
}

impl Publisher {
    pub fn new() -> Self {
        Publisher {
            topics: HashMap::new(),
        }
    }

    pub fn register_topic(&mut self, name: String) {
        let (tx_msg, rx_msg) = mpsc::channel::<Message>(100);

        let mut topic = Topic::new(name.clone(), rx_msg);
        topic.add_consumer();
        topic.init();

        self.topics.insert(name, tx_msg);
    }

    pub async fn send_message(&mut self, topic: String, msg: Message) {
        self.topics.get_mut(&topic).unwrap().send(msg).await;
    }
}

pub struct Topic {
    name: String,
    index: usize,
    rx: Receiver<Message>,
    consumers: Vec<ConsumerHandle>,
}

struct ConsumerHandle {
    join: JoinHandle<Consumer>,
    tx: Sender<Message>,
}

impl ConsumerHandle {
    pub fn new(tx: Sender<Message>, join: JoinHandle<Consumer>) -> Self {
        ConsumerHandle { join, tx }
    }

    pub async fn send(&mut self, msg: Message) -> Result<()> {
        self.tx
            .send(msg)
            .await
            .map_err(|_| anyhow!("Failed to send message to consumer"))
    }
}

impl Topic {
    pub fn new(name: String, rx: Receiver<Message>) -> Self {
        Topic {
            name,
            index: 0,
            consumers: vec![],
            rx,
        }
    }

    pub fn add_consumer(&mut self) {
        let (tx, rx) = mpsc::channel::<Message>(100);

        let handle = Consumer::new(self.consumers.len() as u32, rx).init();

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

pub struct Consumer {
    id: u32,
    msg_count: u64,
    rx: Receiver<Message>,
}

impl Consumer {
    pub fn new(id: u32, rx: Receiver<Message>) -> Self {
        Consumer {
            id,
            msg_count: 0,
            rx,
        }
    }

    pub fn init(mut self) -> JoinHandle<Consumer> {
        tokio::spawn(async move {
            while let Some(message) = self.rx.recv().await {
                match message {
                    Message::Clear => println!("recieved clear message"),
                    Message::Write(data) => println!("recieved write {:?}", data),
                }
            }

            self
        })
    }
}
