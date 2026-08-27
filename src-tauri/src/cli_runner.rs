use crate::{detect, models, models::Provider};
use chrono::Utc;
use std::time::Duration;
use uuid::Uuid;

const TEST_TIMEOUT: Duration = Duration::from_secs(30);
const RUN_TIMEOUT: Duration = Duration::from_secs(300);

#[derive(Debug, thiserror::Error)]
pub enum CliError {
    #[error("Executable not found: {0}")]
    NotFound(String),
    #[error("CLI exited with code {code}: {stderr}")]
    Exit { code: i32, stderr: String },
    #[error("Timed out after {0:?}")]
    Timeout(Duration),
    #[error("HTTP error: {0}")]
    Http(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

async fn exec_cli(
    provider: &Provider,
    model: Option<&str>,
    prompt: &str,
    timeout: Duration,
) -> Result<String, CliError> {
    let (bin, args) = detect::command_for(provider, model, prompt);
    let bin_path = which::which(&bin)
        .map_err(|_| CliError::NotFound(bin.clone()))?;

    let mut cmd = tokio::process::Command::new(bin_path);
    cmd.args(&args)
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .kill_on_drop(true);

    let child = cmd.spawn()?;

    let out = match tokio::time::timeout(timeout, child.wait_with_output()).await {
        Ok(r) => r.map_err(CliError::Io)?,
        Err(_) => return Err(CliError::Timeout(timeout)),
    };

    if !out.status.success() {
        let code = out.status.code().unwrap_or(-1);
        let stderr = String::from_utf8_lossy(&out.stderr).to_string();
        return Err(CliError::Exit { code, stderr });
    }
    Ok(String::from_utf8_lossy(&out.stdout).to_string())
}

async fn exec_ollama_http(
    model: Option<&str>,
    prompt: &str,
    timeout: Duration,
) -> Result<String, CliError> {
    let m = model.unwrap_or("llama3.2");
    let body = serde_json::json!({
        "model": m,
        "prompt": prompt,
        "stream": false,
    });
    let client = reqwest::Client::builder()
        .timeout(timeout)
        .build()
        .map_err(|e| CliError::Http(e.to_string()))?;
    let resp = client
        .post("http://127.0.0.1:11434/api/generate")
        .json(&body)
        .send()
        .await
        .map_err(|e| CliError::Http(e.to_string()))?;
    let status = resp.status();
    let text = resp.text().await.map_err(|e| CliError::Http(e.to_string()))?;
    if !status.is_success() {
        return Err(CliError::Exit { code: status.as_u16() as i32, stderr: text });
    }
    let v: serde_json::Value = serde_json::from_str(&text)
        .map_err(|e| CliError::Http(format!("parse: {e}")))?;
    let response = v.get("response")
        .and_then(|r| r.as_str())
        .unwrap_or("")
        .to_string();
    Ok(response)
}

pub async fn run_tool(
    provider: &Provider,
    model: Option<&str>,
    prompt: &str,
) -> Result<models::Run, CliError> {
    let output = if detect::is_http(provider) {
        exec_ollama_http(model, prompt, RUN_TIMEOUT).await?
    } else {
        exec_cli(provider, model, prompt, RUN_TIMEOUT).await?
    };

    let model_str = match provider {
        Provider::Ollama => format!("ollama:{}", model.unwrap_or("llama3.2")),
        Provider::Copilot => "copilot".into(),
        Provider::Claude => "claude".into(),
        _ => provider.default_model().into(),
    };

    Ok(models::Run {
        id: Uuid::new_v4().to_string(),
        output,
        created_at: Utc::now(),
        model: Some(model_str),
        tokens: None,
    })
}

pub async fn test_tool(provider: &Provider) -> models::CliTestResult {
    let test_prompt = "Say hello in one word.";

    let result = if detect::is_http(provider) {
        exec_ollama_http(None, test_prompt, TEST_TIMEOUT).await
    } else {
        let bin = detect::command_for(provider, None, test_prompt).0;
        if which::which(&bin).is_err() {
            return models::CliTestResult {
                ok: false,
                message: format!("'{}' not found in PATH", bin),
                command: bin,
                found: false,
            };
        }
        exec_cli(provider, None, test_prompt, TEST_TIMEOUT).await
    };

    let command_label = match provider {
        Provider::Ollama => "ollama (http API)".into(),
        Provider::Copilot => "gh copilot".into(),
        Provider::Claude => "claude".into(),
        _ => format!("{:?}", provider),
    };

    match result {
        Ok(s) => {
            let s = s.trim().to_string();
            let preview = if s.len() > 120 { format!("{}…", &s[..120]) } else { s };
            models::CliTestResult { ok: true, message: format!("Working — {}", preview), command: command_label, found: true }
        }
        Err(CliError::Timeout(d)) => models::CliTestResult {
            ok: false,
            message: format!("Timed out after {:?}. The tool may be loading a model or waiting for auth.", d),
            command: command_label,
            found: true,
        },
        Err(CliError::NotFound(n)) => models::CliTestResult {
            ok: false,
            message: format!("'{}' not found in PATH", n),
            command: command_label,
            found: false,
        },
        Err(e) => models::CliTestResult {
            ok: false,
            message: format!("{}", e),
            command: command_label,
            found: true,
        },
    }
}
