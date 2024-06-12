use super::{
    request::Request,
    response::{ErrorResponse, Response},
};

pub async fn create(req: Request) -> anyhow::Result<Response> {
    let client = reqwest::Client::new();
    let api_key = env!("GROQ_API_KEY");
    let body = client
        .post("https://api.groq.com/openai/v1/chat/completions")
        .header("Authorization", format!("Bearer {api_key}"))
        .json(&req)
        .send()
        .await?;
    if body.status() == reqwest::StatusCode::OK {
        Ok(body.json::<Response>().await?)
    } else {
        let statcode = body.status().clone();
        let mut error: ErrorResponse = serde_json::from_str(&body.text().await?)?;
        error.code = statcode;
        anyhow::bail!(error)
    }
}

#[cfg(test)]
mod completion_test {
    use crate::completion::{client::create, message::Message, request::builder};
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
}
