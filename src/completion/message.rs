use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
#[serde(untagged)]
pub enum Message {
    SystemMessage {
        content: Option<String>,
        name: Option<String>,
        role: Option<String>,
        tool_call_id: Option<String>,
    },
    UserMessage {
        content: Option<String>,
        name: Option<String>,
        role: Option<String>,
        tool_call_id: Option<String>,
    },
    AssistantMessage {
        content: Option<String>,
        name: Option<String>,
        role: Option<String>,
        tool_calls: Option<Vec<ToolCall>>,
        tool_call_id: Option<String>,
    },
    ToolMessage {
        content: Option<String>,
        name: Option<String>,
        role: Option<String>,
        tool_call_id: Option<String>,
    },
}

#[derive(Debug, Serialize, Clone)]
pub struct ToolCall {
    pub id: Option<String>,
    #[serde(rename(serialize = "type"))]
    pub tool_type: Option<String>,
    pub function: AssistantFunc,
}

#[derive(Debug, Serialize, Clone)]
pub struct AssistantFunc {
    pub arguments: Option<String>,
    pub name: Option<String>,
}
