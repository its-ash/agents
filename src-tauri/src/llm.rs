use crate::models::{Provider, Run};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum LlmError {
    #[error("No API key configured for provider {0}")]
    NoKey(String),
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),
    #[error("API error ({code}): {message}")]
    Api { code: u16, message: String },
    #[error("No content in response")]
    NoContent,
}

#[derive(Debug, Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct ChatResponse {
    choices: Vec<ChatChoice>,
    #[serde(default)]
    usage: Option<Usage>,
}

#[derive(Debug, Deserialize)]
struct ChatChoice {
    message: ChatMessage,
}

#[derive(Debug, Deserialize)]
struct Usage {
    total_tokens: u32,
}

pub async fn complete(
    provider: &Provider,
    api_key: &str,
    model: &str,
    prompt: &str,
) -> Result<Run, LlmError> {
    let key = api_key.trim();
    if key.is_empty() {
        return Err(LlmError::NoKey(format!("{provider:?}")));
    }
    let model = if model.is_empty() {
        provider.default_model().to_string()
    } else {
        model.to_string()
    };

    let body = ChatRequest {
        model: model.clone(),
        messages: vec![ChatMessage {
            role: "user".into(),
            content: prompt.into(),
        }],
        max_tokens: Some(2048),
        temperature: Some(0.7),
    };

    let client = reqwest::Client::builder()
        .user_agent("agent-studio/0.1")
        .build()?;

    let mut req = client
        .post(provider.endpoint())
        .header("Authorization", provider.auth_header(key))
        .header("Content-Type", "application/json");

    if matches!(provider, Provider::Openrouter) {
        req = req
            .header("HTTP-Referer", "https://github.com/agent-studio")
            .header("X-Title", "Agent Studio");
    }

    let resp = req.json(&body).send().await?;
    let status = resp.status();
    let text = resp.text().await?;

    if !status.is_success() {
        let message = serde_json::from_str::<serde_json::Value>(&text)
            .ok()
            .and_then(|v| {
                v.get("error")
                    .and_then(|e| e.get("message"))
                    .and_then(|m| m.as_str())
                    .map(String::from)
            })
            .unwrap_or(text);
        return Err(LlmError::Api {
            code: status.as_u16(),
            message,
        });
    }

    let parsed: ChatResponse = serde_json::from_str(&text).map_err(|e| LlmError::Api {
        code: 0,
        message: format!("parse error: {e}"),
    })?;

    let output = parsed
        .choices
        .into_iter()
        .next()
        .ok_or(LlmError::NoContent)?
        .message
        .content;

    Ok(Run {
        id: Uuid::new_v4().to_string(),
        output,
        created_at: Utc::now(),
        model: Some(model),
        tokens: parsed.usage.map(|u| u.total_tokens),
    })
}