use super::{
    message::Message,
    request::{self, Request},
    response::{ErrorResponse, Response},
};
use crate::completion::response::StreamResponse;
use futures::StreamExt;
use reqwest::header;
use reqwest_eventsource::{Event, EventSource};

#[derive(Debug, Clone)]
pub enum CompletionOption {
    NonStream(Response),
    Stream(Vec<StreamResponse>),
}

pub struct Groq {
    api_key: String,
    messages: Vec<Message>,
    client: reqwest::Client,
}

impl Groq {
    fn new(api_key: &str) -> Self {
        Self {
            api_key: api_key.into(),
            client: reqwest::Client::new(),
            messages: Vec::new(),
        }
    }

    fn add_message(mut self, msg: Message) -> Self {
        self.messages.push(msg);
        self
    }

    fn add_messages(mut self, msgs: Vec<Message>) -> Self {
        self.messages.extend(msgs);
        self
    }

    fn clear_messages(mut self) -> Self {
        self.messages.clear();
        self.messages.shrink_to(3);
        self
    }

    async fn create_stream_completion(
        &self,
        req: request::builder::RequestBuilder,
    ) -> anyhow::Result<CompletionOption> {
        /* REMARK:
         * https://github.com/jpopesculian/reqwest-eventsource/
         * https://parsec.cloud/en/how-the-reqwest-http-client-streams-responses-in-a-web-context/
         */
        let req = req.with_messages(self.messages.clone())?.build();
        anyhow::ensure!(
            req.is_stream(),
            "'create_stream_completion' func must have the stream flag turned on in request body"
        );
        let mut stream = EventSource::new(
            self.client
                .post("https://api.groq.com/openai/v1/chat/completions")
                .header(header::AUTHORIZATION, format!("Bearer {}", self.api_key))
                .header(header::ACCEPT, "text/event-stream")
                .json(&req),
        )?;
        let mut bufs: Vec<StreamResponse> = Vec::new();
        while let Some(event) = stream.next().await {
            match event {
                Ok(Event::Open) => println!("Connection Open!"),
                Ok(Event::Message(message)) => {
                    if message.data == "[DONE]" {
                        break;
                    }
                    bufs.push(serde_json::from_str(&message.data)?);
                }
                Err(err) => {
                    stream.close();
                    anyhow::bail!("Error: {}", err);
                }
            }
        }
        stream.close();

        Ok(CompletionOption::Stream(bufs))
    }

    async fn create_non_stream_completion(
        &self,
        req: request::builder::RequestBuilder,
    ) -> anyhow::Result<CompletionOption> {
        let req = req.with_messages(self.messages.clone())?.build();
        let body = (self.client)
            .post("https://api.groq.com/openai/v1/chat/completions")
            .header(header::AUTHORIZATION, format!("Bearer {}", self.api_key))
            .json(&req)
            .send()
            .await?;
        if body.status() == reqwest::StatusCode::OK {
            Ok(CompletionOption::NonStream(body.json::<Response>().await?))
        } else {
            let statcode = body.status().clone();
            let mut error: ErrorResponse = serde_json::from_str(&body.text().await?)?;
            error.code = statcode;
            anyhow::bail!(error)
        }
    }

    pub async fn create(
        &self,
        req: request::builder::RequestBuilder,
    ) -> anyhow::Result<CompletionOption> {
        if !req.is_stream() {
            self.create_non_stream_completion(req).await
        } else {
            self.create_stream_completion(req).await
        }
    }
}

#[cfg(test)]
mod completion_test {
    use crate::completion::{client::Groq, message::Message, request::builder};

    #[tokio::test]
    async fn create_completion() -> anyhow::Result<()> {
        let messages = vec![Message::UserMessage {
            role: Some("user".to_string()),
            content: Some("Explain the importance of fast language models".to_string()),
            name: None,
            tool_call_id: None,
        }];
        let request = builder::RequestBuilder::new("mixtral-8x7b-32768".to_string());
        let api_key = env!("GROQ_API_KEY");

        let client = Groq::new(api_key);
        let client = client.add_messages(messages);

        let res = client.create(request).await;
        assert!(res.is_ok());
        Ok(())
    }

    #[tokio::test]
    async fn create_stream_completion() -> anyhow::Result<()> {
        let messages = vec![Message::UserMessage {
            role: Some("user".to_string()),
            content: Some("Explain the importance of fast language models".to_string()),
            name: None,
            tool_call_id: None,
        }];
        let request =
            builder::RequestBuilder::new("mixtral-8x7b-32768".to_string()).with_stream(true);
        let api_key = env!("GROQ_API_KEY");

        let client = Groq::new(api_key);
        let client = client.add_messages(messages);

        let res = client.create(request).await;
        assert!(res.is_ok());
        Ok(())
    }

    #[tokio::test]
    async fn error_does_return() -> anyhow::Result<()> {
        let messages = vec![Message::UserMessage {
            role: Some("user".to_string()),
            content: Some("Explain the importance of fast language models".to_string()),
            name: None,
            tool_call_id: None,
        }];
        let request =
            builder::RequestBuilder::new("mixtral-8x7b-32768".to_string()).with_stream(true);
        let api_key = "";

        let client = Groq::new(api_key);
        let client = client.add_messages(messages);

        let res = client.create(request).await;
        assert!(res.is_err());
        eprintln!("{}", res.unwrap_err());
        Ok(())
    }
}
