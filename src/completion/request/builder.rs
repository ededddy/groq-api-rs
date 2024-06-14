use super::{Message, Request, ResponseFormat, StopEnum, Tool, ToolChoiceEnum};
use serde_json::Value;

/// Provides fluent api for building the request object for chat completion
///
/// The field types, defaults and description could be found from [the official doc](https://console.groq.com/docs/api-reference#chat-create)
///
/// Here and [Request](../../request/struct.Request.html) just a 1:1 mapping from it
#[derive(Debug)]
pub struct RequestBuilder {
    // unused for openai integration only
    logit_bias: Option<serde_json::Value>,
    // unused for openai integration only
    logprobs: bool,         // default false
    frequency_penalty: f32, // defaults to 0
    max_tokens: Option<u32>,
    messages: Vec<Message>,
    model: String,
    n: u32,                          // defaults to 1
    presence_penalty: f32,           // defaults to 0
    response_format: ResponseFormat, // defaults to text,
    seed: Option<i32>,
    stop: Option<StopEnum>,
    stream: bool,     // default false
    temperature: f32, // defaults to 1
    tool_choice: Option<ToolChoiceEnum>,
    tools: Option<Vec<Tool>>,
    top_logprobs: Option<u8>,
    top_p: f32, // defaults to 1
    user: Option<String>,
}

impl RequestBuilder {
    pub fn new(model: String) -> Self {
        //! # Important Note
        //! The builder method of modifying messages filed is hidden because the reposibility is
        //! shifted to the client struct.
        //! such that the client struct can maintain the message history and can be reused.
        //!
        //! # Description
        //! Instantiates a RequestBuilder struct with a set of default values for the request object of groq chat completion API.
        //! ```ignore no_run
        //! Self {
        //!    logit_bias: None,
        //!    logprobs: false,
        //!    frequency_penalty: 0.0,
        //!    max_tokens: None,
        //!    messages: Vec::new(),
        //!    model,
        //!    n: 1,
        //!    presence_penalty: 0.0,
        //!    response_format: ResponseFormat {
        //!        response_type: "text".into(),
        //!    },
        //!    seed: None,
        //!    stop: None,
        //!    stream: false,
        //!    temperature: 1.0,
        //!    tool_choice: None,
        //!    tools: None,
        //!    top_logprobs: None,
        //!    top_p: 1.0,
        //!    user: None,
        //!}
        //!```
        Self {
            logit_bias: None,
            logprobs: false,
            frequency_penalty: 0.0,
            max_tokens: None,
            messages: Vec::new(),
            model,
            n: 1,
            presence_penalty: 0.0,
            response_format: ResponseFormat {
                response_type: "text".into(),
            },
            seed: None,
            stop: None,
            stream: false,
            temperature: 1.0,
            tool_choice: None,
            tools: None,
            top_logprobs: None,
            top_p: 1.0,
            user: None,
        }
    }

    pub fn build(self) -> Request {
        Request {
            logit_bias: self.logit_bias,
            logprobs: self.logprobs,
            frequency_penalty: self.frequency_penalty,
            max_tokens: self.max_tokens,
            messages: self.messages,
            model: self.model,
            n: self.n,
            presence_penalty: self.presence_penalty,
            response_format: self.response_format,
            seed: self.seed,
            stop: self.stop,
            stream: self.stream,
            temperature: self.temperature,
            tool_choice: self.tool_choice,
            tools: self.tools,
            top_logprobs: self.top_logprobs,
            top_p: self.top_p,
            user: self.user,
        }
    }

    pub fn with_logit_bias(mut self, logit_bias: Value) -> Self {
        self.logit_bias = Some(logit_bias);
        self
    }

    pub fn with_logprobs(mut self, logprobs: bool) -> Self {
        self.logprobs = logprobs;
        self
    }

    pub fn with_frequency_penalty(mut self, penalty: f32) -> Self {
        self.frequency_penalty = penalty;
        self
    }

    pub fn with_max_tokens(mut self, n: u32) -> Self {
        self.max_tokens = Some(n);
        self
    }

    pub(crate) fn with_messages(mut self, msgs: Vec<Message>) -> anyhow::Result<Self> {
        anyhow::ensure!(msgs.len() > 0, "message cannot be empty");
        self.messages = msgs;
        Ok(self)
    }

    pub fn with_model(mut self, model: &str) -> Self {
        self.model = model.into();
        self
    }

    pub fn with_n(mut self, n: u32) -> Self {
        self.n = n;
        self
    }

    pub fn with_presence_penalty(mut self, penalty: f32) -> Self {
        self.presence_penalty = penalty;
        self
    }

    pub fn with_response_fmt(mut self, fmt: ResponseFormat) -> Self {
        self.response_format = fmt;
        self
    }

    pub fn with_seed(mut self, seed: i32) -> Self {
        self.seed = Some(seed);
        self
    }

    pub fn with_stop(mut self, stop: &str) -> Self {
        self.stop = Some(StopEnum::Token(stop.into()));
        self
    }

    pub fn with_stops(mut self, stops: Vec<String>) -> Self {
        self.stop = Some(StopEnum::Tokens(stops));
        self
    }

    pub fn with_stream(mut self, stream: bool) -> Self {
        self.stream = stream;
        self
    }

    pub fn with_temperature(mut self, temp: f32) -> Self {
        self.temperature = temp;
        self
    }

    pub fn with_tool_choice(mut self, tool: Tool) -> Self {
        self.tool_choice = Some(ToolChoiceEnum::Tool(tool));
        self
    }
    pub fn with_auto_tool_choice(mut self) -> Self {
        self.tool_choice = Some(ToolChoiceEnum::Str("auto".into()));
        self
    }

    pub fn with_tools(mut self, tools: Vec<Tool>) -> Self {
        self.tools = Some(tools);
        self
    }

    pub fn with_top_logprobs(mut self, prob: u8) -> Self {
        self.top_logprobs = Some(prob);
        self
    }

    pub fn with_top_p(mut self, top_p: f32) -> Self {
        self.top_p = top_p;
        self
    }

    pub fn with_user(mut self, user: &str) -> Self {
        self.user = Some(user.into());
        self
    }

    pub fn is_stream(&self) -> bool {
        //! Check the request object is set to use stream for the completion response or not
        //! - true if the stream flag is on
        //! - false if the stream flag is off
        self.stream
    }
}
