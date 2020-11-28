use anyhow::Result;

use super::consumer::Consumer;

#[derive(Debug, Clone)]
pub struct Content {
    data: Vec<u8>,
}

impl Content {
    pub fn new(data: Vec<u8>) -> Self {
        Content { data }
    }

    pub fn get_data(self) -> Vec<u8> {
        self.data
    }
}

#[derive(Clone, Debug)]
pub enum Message {
    Write(Content),
    Clear,
    Kill,
}

impl Sendable for Message {}

pub trait Consumable {
    type Operator;
    fn consume(o: &mut Self::Operator, msg: &mut Self) -> Result<()>;
}

impl Consumable for Message {
    type Operator = Consumer;
    fn consume(o: &mut Consumer, msg: &mut Self) -> Result<()> {
        Ok(())
    }
}

pub trait Sendable {}
