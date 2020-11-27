use std::{collections::HashMap, path::PathBuf};

use tokio::{sync::mpsc, sync::mpsc::Receiver, sync::mpsc::Sender, task::JoinHandle};

use super::api::Message;
use anyhow::{anyhow, Context, Result};
use tokio::prelude::*;

pub struct Consumer {
    id: u32,
    rx: Receiver<Message>,
    db: PathBuf,
}

impl Consumer {
    pub fn new(id: u32, db: PathBuf, rx: Receiver<Message>) -> Self {
        Consumer { id, rx, db }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn init(mut self) -> JoinHandle<Consumer> {
        tokio::spawn(async move {
            while let Some(message) = self.rx.recv().await {
                match message {
                    Message::Clear => {
                        if let Err(e) = self.clear().await {
                            println!("Failed to clear file {:?}", e);
                        }
                    }
                    Message::Write(content) => {
                        if let Err(e) = self.write_data(content.get_data()).await {
                            println!("Failed to write to file {:?}", e);
                        }
                    }
                }
            }

            self
        })
    }

    async fn write_data(&self, data: Vec<u8>) -> Result<()> {
        let mut file = tokio::fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open(self.db.clone())
            .await?;

        file.write(&data).await?;

        Ok(())
    }

    async fn clear(&self) -> Result<()> {
        let mut file = tokio::fs::File::open(self.db.clone()).await?;

        file.write(&[]).await?;

        Ok(())
    }
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
