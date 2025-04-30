use super::message::BaseMessage;
use std::sync::Mutex;

#[derive(Default)]
pub struct AppContext {
    pub messages: Mutex<Vec<String>>,
}

#[derive(Clone, Debug)]
pub struct MarshoContext {
    messages: Vec<BaseMessage>,
}

impl MarshoContext {
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
        }
    }

    pub fn add(&mut self, message: BaseMessage) {
        self.messages.push(message);
    }

    pub fn get(&self) -> &Vec<BaseMessage> {
        &self.messages
    }

    pub fn reset(&mut self) {
        self.messages.clear();
    }

    pub fn set(&mut self, messages: Vec<BaseMessage>) {
        self.messages = messages;
    }
}
