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
