use std::collections::HashMap;

use tokio::{sync::mpsc, sync::mpsc::Receiver, sync::mpsc::Sender, task::JoinHandle};

use anyhow::Result;
use anyhow::{anyhow, Context};

use super::api::Message;

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
                  Message::Clear => {
                    self.msg_count += 1;
                    println!("recieved clear message");
                  },
                  Message::Write(data) => {
                    
                  }
                }
            }

            self
        })
    }

    // fn write_data(&self, data: Vec<u8>) {
    //     tokio::fs::
    // }
}

pub struct ConsumerHandle {
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
