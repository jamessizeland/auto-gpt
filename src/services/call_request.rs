use crate::models::general::llm::{APIResponse, ChatCompletion, Message};
use anyhow::Context;
use dotenv::dotenv;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};
use std::env;

// Call large language model
pub async fn call_gpt(messages: Vec<Message>, temperature: f32) -> Result<String, anyhow::Error> {
    dotenv().ok();
    let api_key = env::var("OPEN_AI_KEY").context("OPEN_AI_KEY not found in .env")?;
    let api_org = env::var("OPEN_AI_ORG").context("OPEN_AI_ORG not found in .env")?;
    let api_url = "https://api.openai.com/v1/chat/completions";

    // create API key & open ai org header
    let mut headers = HeaderMap::new();
    headers.insert(
        "authorization",
        HeaderValue::from_str(&format!("Bearer {api_key}"))?,
    );
    headers.insert(
        "OpenAI-Organization",
        HeaderValue::from_str(&api_org.as_str())?,
    );
    let client = Client::builder().default_headers(headers).build()?;
    let chat_completion = dbg!(ChatCompletion {
        model: "gpt-3.5-turbo".to_string(),
        messages,
        temperature,
    });
    let resp_raw = client.post(api_url).json(&chat_completion).send().await?;
    // dbg!(&resp_raw.text().await?);
    let res: APIResponse = resp_raw.json().await?;
    Ok(res.choices[0].message.content.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_call_gpt() -> Result<(), anyhow::Error> {
        let messages = vec![Message {
            role: "user".to_string(),
            content: "This is a test, give me a short response please.".to_string(),
        }];
        let temperature = 0.5;
        dbg!(call_gpt(messages, temperature).await?);
        Ok(())
    }
}
