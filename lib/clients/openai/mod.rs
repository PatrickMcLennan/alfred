use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize)]
struct ResponseFormat {
    r#type: String,
}

/// Shape of the request to the OpenAI API
#[derive(Debug, Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    response_format: Option<ResponseFormat>,
}

#[derive(Debug, Serialize)]
struct Message {
    role: String,
    content: String,
}

/// Shape of the response from OpenAI’s Chat Completions API
#[derive(Debug, Deserialize)]
pub struct ChatResponse {
    pub choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
pub struct Choice {
    pub message: MessageContent,
}

#[derive(Debug, Deserialize)]
pub struct MessageContent {
    pub role: String,
    pub content: String,
}

pub async fn get_recommendations(client: &Client, prompt: &str) -> Result<String> {
    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");

    let req = ChatRequest {
        model: "gpt-4o-mini".to_string(),
        messages: vec![Message {
            role: "user".to_string(),
            content: prompt.to_string(),
        }],
        response_format: Some(ResponseFormat {
            r#type: "json_object".into(),
        }),
    };

    println!("req: {:?}", req);

    let resp = match client
        .post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(api_key)
        .json(&req)
        .send()
        .await?
        .error_for_status()?
        .json::<ChatResponse>()
        .await
    {
        Ok(resp) => resp,
        Err(e) => {
            println!("error: {:?}", e);
            return Err(anyhow::anyhow!("error: {:?}", e));
        }
    };

    let content = resp
        .choices
        .get(0)
        .map(|c| c.message.content.clone())
        .unwrap_or_default();

    Ok(content)
}
