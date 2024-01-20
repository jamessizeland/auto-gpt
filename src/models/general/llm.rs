use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Clone)]
pub struct Message {
    /// The role of the message sender
    pub role: String,
    /// The content of the message
    pub content: String,
}

#[derive(Debug, Serialize)]
pub struct ChatCompletion {
    /// The model used for the completion
    pub model: String,
    /// The content of the conversation so far
    pub messages: Vec<Message>,
    /// The randomness of the response
    pub temperature: f32,
}

#[derive(Debug, Deserialize)]
pub struct APIMessage {
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct APIChoice {
    pub message: APIMessage,
}

#[derive(Debug, Deserialize)]
pub struct APIResponse {
    pub choices: Vec<APIChoice>,
}
