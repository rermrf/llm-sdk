use crate::IntoRequest;
use derive_builder::Builder;
use reqwest::{Client, RequestBuilder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
pub struct CreateImageRequest {
    /// A text description of the desired image(s). The maximum length is 4000 characters for dall-e-3.
    pub prompt: String,
    /// The model to use for image generation. Only support Dall-e-3
    #[builder(default)]
    pub model: ImageModel,
    /// The number of images to generate. Must be between 1 and 10. For dall-e-3, only n=1 is supported.
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<usize>,
    /// The quality of the image that will be generated. hd creates images with finer details and greater consistency across the image. This param is only supported for dall-e-3.
    #[builder(default)]
    pub quality: ImageQuality,
    /// The format in which the generated images are returned. Must be one of url or b64_json
    #[builder(default)]
    pub response_format: ImageResponseFormat,
    /// The size of the generated images. Must be one of 1024x1024, 1792x1024, or 1024x1792 for dall-e-3 models.
    pub size: ImageSize,
    /// The style of the generated images. Must be one of vivid or natural. Vivid causes the model to lean towards generating hyper-real and dramatic images. Natural causes the model to produce more natural, less hyper-real looking images. This param is only supported for dall-e-3.
    pub style: Option<String>,
    /// A unique identifier representing your end-user, which can help OpenAI to monitor and detect abuse.
    pub user: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImageModel {
    #[serde(rename = "dall-e-3")]
    DallE3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImageQuality {
    Standard,
    Hd,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImageResponseFormat {
    Url,
    B64Json,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImageSize {
    #[serde(rename = "1024x1024")]
    Size1024x1024,
    #[serde(rename = "1792x1024")]
    Size1792x1024,
    #[serde(rename = "1024x1792")]
    Size1024x1792,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateImageResponse {}

impl IntoRequest for CreateImageResponse {
    fn into_request(self, client: Client) -> RequestBuilder {
        client
            .post("https://api.openai.com/v1/images/generations")
            .json(&self)
    }
}

impl Default for ImageModel {
    fn default() -> Self {
        ImageModel::DallE3
    }
}

impl Default for ImageQuality {
    fn default() -> Self {
        ImageQuality::Standard
    }
}

impl Default for ImageResponseFormat {
    fn default() -> Self {
        ImageResponseFormat::Url
    }
}
