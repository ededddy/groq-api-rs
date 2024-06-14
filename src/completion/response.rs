use chrono::{serde::ts_seconds, Utc};
use serde::Deserialize;
use std::fmt::Display;

/// Response object responsible for representing error object returned
/// # Difference from groq's
/// - Added Status Code field for convenience
#[derive(Debug, Deserialize, Clone)]
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

#[derive(Debug, Deserialize, Clone)]
pub struct ErrorBody {
    #[serde(rename(deserialize = "type"))]
    pub error_type: String,
    pub message: String,
}

/// Response object responsible for representing completion chunk object returned
/// # Difference from standard completion object
/// - The x_groq struct contains the server stream event ID and usage info at the last message
#[derive(Debug, Deserialize, Clone)]
pub struct StreamResponse {
    pub id: String,
    pub object: String,
    #[serde(with = "ts_seconds")]
    pub created: chrono::DateTime<Utc>,
    pub model: String,

    pub system_fingerprint: Option<String>,
    pub choices: Vec<StreamChoice>,
    pub x_groq: Option<XGroq>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct StreamChoice {
    pub index: u32,
    pub delta: ChoiceDelta,
    pub logprobs: Option<f32>,
    pub finish_reason: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ChoiceDelta {
    role: Option<String>,
    content: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct XGroq {
    pub id: String,
    pub usage: Option<UsageInfo>,
}

/// Response object responsible for representing completion object returned
#[derive(Debug, Deserialize, Clone)]
pub struct Response {
    pub id: String,
    pub object: String,
    #[serde(with = "ts_seconds")]
    pub created: chrono::DateTime<Utc>,
    pub model: String,

    pub system_fingerprint: Option<String>,
    pub choices: Vec<Choice>,
    pub usage: UsageInfo,
}

#[derive(Debug, Deserialize, Clone)]
pub struct UsageInfo {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
    pub prompt_time: f32,
    pub completion_time: f32,
    pub total_time: f32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Choice {
    pub index: u32,
    pub message: ChoiceMessage,
    pub finish_reason: String,
    pub logprobs: Option<f32>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ChoiceMessage {
    pub role: String,
    pub content: String,
}
