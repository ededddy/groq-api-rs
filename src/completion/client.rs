use super::{
    request::Request,
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

async fn create_stream_completion(req: Request) -> anyhow::Result<CompletionOption> {
    /* REMARK:
     * https://github.com/jpopesculian/reqwest-eventsource/
     * https://parsec.cloud/en/how-the-reqwest-http-client-streams-responses-in-a-web-context/
     */
    anyhow::ensure!(
        req.is_stream(),
        "'create_stream_completion' func must have the stream flag turned on in request body"
    );
    let client = reqwest::Client::new();
    let api_key = env!("GROQ_API_KEY");
    let mut stream = EventSource::new(
        client
            .post("https://api.groq.com/openai/v1/chat/completions")
            .header(header::AUTHORIZATION, format!("Bearer {api_key}"))
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

async fn create_non_stream_completion(req: Request) -> anyhow::Result<CompletionOption> {
    let client = reqwest::Client::new();
    let api_key = env!("GROQ_API_KEY");
    let body = client
        .post("https://api.groq.com/openai/v1/chat/completions")
        .header(header::AUTHORIZATION, format!("Bearer {api_key}"))
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

pub async fn create(req: Request) -> anyhow::Result<CompletionOption> {
    if !req.is_stream() {
        create_non_stream_completion(req).await
    } else {
        create_stream_completion(req).await
    }
}

#[cfg(test)]
mod completion_test {
    use crate::completion::{
        client::{create, create_stream_completion},
        message::Message,
        request::builder,
    };
    use anyhow::Context;

    #[tokio::test]
    async fn create_completion() -> anyhow::Result<()> {
        let messages = vec![Message::UserMessage {
            role: Some("user".to_string()),
            content: Some("Explain the importance of fast language models".to_string()),
            name: None,
            tool_call_id: None,
        }];
        let request =
            builder::RequestBuilder::new("mixtral-8x7b-32768".to_string(), messages.clone())
                .context("the messages vec should be at least 1")?
                .with_stop("endline")
                .build();
        let res = create(request).await;
        assert!(res.is_ok());
        Ok(())
    }

    #[tokio::test]
    async fn test_stream_completion() -> anyhow::Result<()> {
        let messages = vec![Message::UserMessage {
            role: Some("user".to_string()),
            content: Some("Explain the importance of fast language models".to_string()),
            name: None,
            tool_call_id: None,
        }];
        let request =
            builder::RequestBuilder::new("mixtral-8x7b-32768".to_string(), messages.clone())
                .context("the messages vec should be at least 1")?
                .with_stream(true)
                .build();
        let res = create_stream_completion(request).await;
        assert!(res.is_ok());
        Ok(())
    }
}
