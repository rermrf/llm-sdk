use reqwest::{Client, RequestBuilder};
use serde::{Deserialize, Serialize};

use crate::IntoRequest;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletionRequest {
    /// A list of messages comprising the conversation so far.
    messages: Vec<String>,
    
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletionResponse {}

impl IntoRequest for ChatCompletionResponse {
    fn into_request(self, client: Client) -> RequestBuilder {
        client
            .post("https://api.openai.com/v1/chat/completions")
            .json(&self)
    }
}


