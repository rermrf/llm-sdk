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
    System(String),
    /// A message from a human
    User(String),
    /// A message from the model
    Model(String),
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
