use crate::IntoRequest;
use derive_builder::Builder;
use reqwest::{Client, RequestBuilder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Builder)]
#[builder(pattern = "mutable")]
pub struct ChatCompletionRequest {
    /// A list of messages comprising the conversation so far.
    #[builder(setter(into))]
    messages: Vec<ChatCompletionMessage>,
    /// ID of the model to use. See the model endpoint compatibility table for details on which models work with the Chat API.
    model: String,
}

#[derive(Debug, Clone, Serialize)]
pub enum ChatCompletionMessage {
    /// A message from a system
    System(SystemMessage),
    /// A message from a human
    User(String),
    /// A message from the Assistant
    Assistant(String),
    /// A message from a Tool
    Tool(String),
}

#[derive(Debug, Clone, Serialize)]
pub struct SystemMessage{
    /// The contents of the system message.
    content: String,
    /// An optional name for the participant. Provides the model information to differentiate between participants of the same role.
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChatCompletionResponse {}

impl IntoRequest for ChatCompletionRequest {
    fn into_request(self, client: Client) -> RequestBuilder {
        client
            .post("https://api.openai.com/v1/chat/completions")
            .json(&self)
    }
}
