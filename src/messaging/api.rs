use std::collections::HashMap;

use tokio::{sync::mpsc, sync::mpsc::Receiver, sync::mpsc::Sender, task::JoinHandle};

use anyhow::{anyhow, Context};

#[derive(Debug, Clone)]
pub struct Content {
    data: Vec<u8>,
}

impl Content {
    pub fn new(data: Vec<u8>) -> Self {
        Content { data }
    }
}

#[derive(Clone)]
pub enum Message {
    Write(Content),
    Clear,
}