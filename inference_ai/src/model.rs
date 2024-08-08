use std::fmt::Debug;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Health {
    status: String
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    Assistant,
    User
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub content: String,
    pub role: Role
}

impl Message {
    pub fn new(text: String, role: Role) -> Self {
        Message { content: text, role }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAICompletion {
    model: String,
    temperature: f32,
    stream: bool,
    messages: Vec<Message>
}

impl OpenAICompletion {
    pub fn new(model: String, messages: Vec<Message>, temperature: Option<f32>, stream: Option<bool>) -> Self {
        Self { model, temperature: temperature.unwrap_or(0.7), stream: stream.unwrap_or(false), messages }
    }

    pub fn new_message(&mut self, message: Message) {
        let mut vector_container = vec![message];
        self.messages.append(&mut vector_container);
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Choice {
    pub finish_reason: String,
    pub index: u32,
    pub message: Message
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Usage {
    completion_tokens: u32,
    prompt_tokens: u32,
    total_tokens: u32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAICompletionResponse {
    pub id: String,
    pub created: u32,
    pub model: String,
    pub choices: Vec<Choice>,
    pub object: String,
    pub usage: Usage
}
