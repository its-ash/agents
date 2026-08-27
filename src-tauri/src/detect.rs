use crate::models::DetectedTools;

pub async fn detect() -> DetectedTools {
    let copilot_path = which::which("gh").ok();
    let copilot = copilot_path.is_some();

    let claude_path = which::which("claude").ok();
    let claude = claude_path.is_some();

    let ollama_path = which::which("ollama").ok();
    let ollama = ollama_path.is_some();

    let ollama_models = if ollama {
        list_ollama_models().await.unwrap_or_default()
    } else {
        Vec::new()
    };

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

async fn list_ollama_models() -> Result<Vec<String>, std::io::Error> {
    let output = tokio::process::Command::new("ollama")
        .arg("list")
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .kill_on_drop(true)
        .output()
        .await?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut models = Vec::new();
    for (i, line) in stdout.lines().enumerate() {
        if i == 0 {
            continue;
        }
        let name = line.split_whitespace().next();
        if let Some(n) = name {
            if !n.is_empty() {
                models.push(n.to_string());
            }
        }
    }
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