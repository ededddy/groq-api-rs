use chrono::Utc;
pub struct Response {
    pub id: String,
    pub object: String,
    pub created: chrono::DateTime<Utc>,
    pub model: String,
    pub system_fingerprint: Option<String>,
    pub choices: Vec<Choice>,
    pub usage: UsageInfo,
}

pub struct UsageInfo {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
    pub prompt_time: f32,
    pub completion_time: f32,
    pub total_time: f32,
}

pub struct Choice {
    pub index: u32,
    pub message: ChoiceMessage,
    pub finish_reason: String,
    pub logprobs: Option<f32>,
}

pub struct ChoiceMessage {
    pub role: String,
    pub content: String,
}
