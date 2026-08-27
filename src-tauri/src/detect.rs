use crate::models::DetectedTools;

pub async fn detect() -> DetectedTools {
    let copilot_path = which::which("gh").ok();
    let copilot = copilot_path.is_some();

    let claude_path = which::which("claude").ok();
    let claude = claude_path.is_some();

    let ollama_path = which::which("ollama").ok();
    let ollama_models = list_ollama_models().await.unwrap_or_default();
    let ollama = ollama_path.is_some() || !ollama_models.is_empty();

    DetectedTools {
        copilot,
        copilot_path: copilot_path.map(|p| p.to_string_lossy().to_string()),
        claude,
        claude_path: claude_path.map(|p| p.to_string_lossy().to_string()),
        ollama,
        ollama_path: ollama_path.map(|p| p.to_string_lossy().to_string()),
        ollama_models,
    }
}

async fn list_ollama_models() -> Result<Vec<String>, String> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(2))
        .build()
        .map_err(|e| e.to_string())?;

    let resp = client
        .get("http://127.0.0.1:11434/api/tags")
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !resp.status().is_success() {
        return Err(format!("ollama api returned {}", resp.status()));
    }

    let v: serde_json::Value = resp.json().await.map_err(|e| e.to_string())?;
    let models = v
        .get("models")
        .and_then(|m| m.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|m| m.get("name").and_then(|n| n.as_str()))
                .map(|s| s.to_string())
                .collect()
        })
        .unwrap_or_default();
    Ok(models)
}

pub fn command_for(provider: &crate::models::Provider, model: Option<&str>, prompt: &str) -> (String, Vec<String>) {
    match provider {
        crate::models::Provider::Copilot => (
            "gh".into(),
            vec!["copilot".into(), "suggest".into(), "-t".into(), "shell".into(), prompt.into()],
        ),
        crate::models::Provider::Claude => (
            "claude".into(),
            vec!["-p".into(), prompt.into()],
        ),
        crate::models::Provider::Ollama => {
            let m = model.unwrap_or("llama3.2");
            (
                "ollama".into(),
                vec!["run".into(), m.into(), prompt.into()],
            )
        }
        _ => ("echo".into(), vec![prompt.into()]),
    }
}

pub fn is_http(provider: &crate::models::Provider) -> bool {
    matches!(provider, crate::models::Provider::Ollama)
}