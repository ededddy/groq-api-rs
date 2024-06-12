use chrono::{serde::ts_seconds, Utc};
use serde::Deserialize;
use std::fmt::Display;

#[derive(Debug, Deserialize)]
pub struct ErrorResponse {
    pub error: ErrorBody,

    #[serde(skip_deserializing)]
    pub code: reqwest::StatusCode,
}
impl Display for ErrorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "status_code : {}, error : {:?}", self.code, self.error)
    }
}

#[derive(Debug, Deserialize)]
pub struct ErrorBody {
    #[serde(rename(deserialize = "type"))]
    pub error_type: String,
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct Response {
    pub id: String,
    pub object: String,
    #[serde(with = "ts_seconds")]
    pub created: chrono::DateTime<Utc>,
    pub model: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_fingerprint: Option<String>,
    pub choices: Vec<Choice>,
    pub usage: UsageInfo,
}

#[derive(Debug, Deserialize)]
pub struct UsageInfo {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
    pub prompt_time: f32,
    pub completion_time: f32,
    pub total_time: f32,
}

#[derive(Debug, Deserialize)]
pub struct Choice {
    pub index: u32,
    pub message: ChoiceMessage,
    pub finish_reason: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<f32>,
}

#[derive(Debug, Deserialize)]
pub struct ChoiceMessage {
    pub role: String,
    pub content: String,
}
