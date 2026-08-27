use crate::{cli_runner, detect, llm, models, storage};
use std::collections::HashMap;
use tauri::State;
use tokio::sync::Mutex;

pub struct AppState {
    pub agents: Mutex<Vec<models::Agent>>,
    pub settings: Mutex<models::AppSettings>,
}

impl AppState {
    pub fn new() -> Self {
        let agents = storage::load_agents();
        let settings = storage::load_settings();
        Self {
            agents: Mutex::new(agents),
            settings: Mutex::new(settings),
        }
    }
}

#[tauri::command]
pub async fn list_agents(state: State<'_, AppState>) -> Result<Vec<models::Agent>, String> {
    let agents = state.agents.lock().await;
    Ok(agents.clone())
}

#[tauri::command]
pub async fn create_agent(
    name: String,
    prompt: String,
    provider: Option<models::Provider>,
    model: Option<String>,
    state: State<'_, AppState>,
) -> Result<models::Agent, String> {
    let mut agents = state.agents.lock().await;
    let id = format!("agent-{}", chrono::Utc::now().timestamp_millis());
    let provider = provider.unwrap_or_else(|| {
        let s = state.settings.blocking_lock();
        s.default_provider.clone()
    });
    let agent = models::Agent {
        id,
        name,
        prompt: if prompt.trim().is_empty() {
            "Write something about {{topic}}.".into()
        } else {
            prompt
        },
        model,
        provider,
        runs: vec![],
    };
    agents.insert(0, agent.clone());
    storage::save_agents(&agents).map_err(|e| e.to_string())?;
    Ok(agent)
}

#[tauri::command]
pub async fn update_agent(
    id: String,
    name: String,
    prompt: String,
    provider: Option<models::Provider>,
    model: Option<String>,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let mut agents = state.agents.lock().await;
    if let Some(a) = agents.iter_mut().find(|a| a.id == id) {
        a.name = name;
        a.prompt = prompt;
        if let Some(p) = provider {
            a.provider = p;
        }
        a.model = model;
    }
    storage::save_agents(&agents).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn delete_agent(id: String, state: State<'_, AppState>) -> Result<(), String> {
    let mut agents = state.agents.lock().await;
    agents.retain(|a| a.id != id);
    storage::save_agents(&agents).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn get_placeholders(template: String) -> Result<Vec<String>, String> {
    Ok(models::extract_placeholders(&template))
}

#[tauri::command]
pub async fn run_agent(
    id: String,
    values: HashMap<String, String>,
    state: State<'_, AppState>,
) -> Result<models::Run, String> {
    let (prompt, provider, model, api_key) = {
        let agents = state.agents.lock().await;
        let agent = agents
            .iter()
            .find(|a| a.id == id)
            .ok_or("Agent not found")?
            .clone();
        let settings = state.settings.lock().await;
        let rendered = models::render_template(&agent.prompt, &values);
        let key = match agent.provider {
            models::Provider::Openrouter => settings.openrouter_key.clone(),
            models::Provider::Openai => settings.openai_key.clone(),
            _ => None,
        };
        let key = if agent.provider.is_api() {
            key.ok_or_else(|| format!("No API key configured for {:?}", agent.provider))?
        } else {
            String::new()
        };
        (rendered, agent.provider, agent.model.clone(), key)
    };

    let run = if provider.is_cli() {
        cli_runner::run_tool(&provider, model.as_deref(), &prompt)
            .await
            .map_err(|e| e.to_string())?
    } else {
        let model_str = model.as_deref().unwrap_or("");
        llm::complete(&provider, &api_key, model_str, &prompt)
            .await
            .map_err(|e| e.to_string())?
    };

    let mut agents = state.agents.lock().await;
    if let Some(a) = agents.iter_mut().find(|a| a.id == id) {
        a.runs.insert(0, run.clone());
    }
    storage::save_agents(&agents).map_err(|e| e.to_string())?;
    Ok(run)
}

#[tauri::command]
pub async fn detect_tools() -> Result<models::DetectedTools, String> {
    Ok(detect::detect().await)
}

#[tauri::command]
pub async fn test_tool(provider: models::Provider) -> Result<models::CliTestResult, String> {
    Ok(cli_runner::test_tool(&provider).await)
}

#[tauri::command]
pub async fn get_settings(
    state: State<'_, AppState>,
) -> Result<models::AppSettings, String> {
    let s = state.settings.lock().await;
    Ok(s.clone())
}

#[tauri::command]
pub async fn save_settings(
    settings: models::AppSettings,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let mut s = state.settings.lock().await;
    *s = settings;
    storage::save_settings(&s).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn export_agents(state: State<'_, AppState>, app: tauri::AppHandle) -> Result<Option<String>, String> {
    use tauri_plugin_dialog::DialogExt;

    let agents = state.agents.lock().await;

    let folder = app.dialog()
        .file()
        .set_title("Select folder to export agents into")
        .blocking_pick_folder();

    let Some(folder) = folder else {
        return Ok(None);
    };

    let folder = folder.into_path().map_err(|e| e.to_string())?;
    let agents_dir = folder.join("agents");
    std::fs::create_dir_all(&agents_dir).map_err(|e| e.to_string())?;

    for a in agents.iter() {
        let safe_name = a.name
            .chars()
            .map(|c| if c.is_alphanumeric() || c == '-' || c == '_' { c } else { '-' })
            .collect::<String>();
        let file_path = agents_dir.join(format!("{}.md", safe_name));
        let mut content = format!("# {}\n\n", a.name);
        if let Some(m) = &a.model {
            content.push_str(&format!("<!-- model: {} -->\n", m));
        }
        content.push_str(&format!("<!-- provider: {:?} -->\n\n", a.provider));
        content.push_str(&a.prompt);
        std::fs::write(&file_path, content).map_err(|e| e.to_string())?;
    }

    Ok(Some(agents_dir.to_string_lossy().to_string()))
}

#[tauri::command]
pub async fn import_agents_from_file(
    state: State<'_, AppState>,
    app: tauri::AppHandle,
) -> Result<usize, String> {
    use tauri_plugin_dialog::DialogExt;

    let folder = app.dialog()
        .file()
        .set_title("Select folder containing agent .md files")
        .blocking_pick_folder();

    let Some(folder) = folder else {
        return Ok(0);
    };

    let folder = folder.into_path().map_err(|e| e.to_string())?;
    let (default_provider, default_model) = {
        let s = state.settings.lock().await;
        (s.default_provider.clone(), s.default_model.clone())
    };

    let imported = parse_agent_files(&folder, &default_provider, &default_model)?;

    let mut agents = state.agents.lock().await;
    let count = imported.len();
    for a in imported {
        let mut agent = a;
        agent.id = format!("agent-{}", chrono::Utc::now().timestamp_millis());
        agent.runs = vec![];
        agents.insert(0, agent);
    }
    storage::save_agents(&agents).map_err(|e| e.to_string())?;
    Ok(count)
}

#[tauri::command]
pub async fn import_agents_from_repo(
    repo_url: String,
    state: State<'_, AppState>,
) -> Result<usize, String> {
    let tmp = std::env::temp_dir().join(format!("agent-studio-import-{}", chrono::Utc::now().timestamp_millis()));

    let output = tokio::process::Command::new("git")
        .args(["clone", "--depth", "1", &repo_url, tmp.to_str().unwrap_or("")])
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .kill_on_drop(true)
        .output()
        .await
        .map_err(|e| format!("Failed to run git: {e}"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        let _ = std::fs::remove_dir_all(&tmp);
        return Err(format!("git clone failed: {stderr}"));
    }

    let (default_provider, default_model) = {
        let s = state.settings.lock().await;
        (s.default_provider.clone(), s.default_model.clone())
    };

    let imported = parse_agent_files(&tmp, &default_provider, &default_model);
    let _ = std::fs::remove_dir_all(&tmp);

    let imported = imported.map_err(|e| e.to_string())?;

    let mut agents = state.agents.lock().await;
    let count = imported.len();
    for a in imported {
        let mut agent = a;
        agent.id = format!("agent-{}", chrono::Utc::now().timestamp_millis());
        agent.runs = vec![];
        agents.insert(0, agent);
    }
    storage::save_agents(&agents).map_err(|e| e.to_string())?;
    Ok(count)
}

fn parse_agent_files(
    dir: &std::path::Path,
    default_provider: &models::Provider,
    default_model: &Option<String>,
) -> Result<Vec<models::Agent>, String> {
    let entries = std::fs::read_dir(dir).map_err(|e| e.to_string())?;
    let mut agents = Vec::new();

    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
        if ext != "md" && ext != "markdown" && ext != "txt" {
            continue;
        }

        let content = match std::fs::read_to_string(&path) {
            Ok(c) => c,
            Err(_) => continue,
        };

        let stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("Untitled")
            .to_string();

        let (name, prompt, provider, model) = parse_md_agent(&content, &stem, default_provider, default_model);

        agents.push(models::Agent {
            id: String::new(),
            name,
            prompt,
            model,
            provider,
            runs: vec![],
        });
    }

    if agents.is_empty() {
        return Err("No .md agent files found in the selected folder.".into());
    }
    Ok(agents)
}

fn parse_md_agent(
    content: &str,
    fallback_name: &str,
    default_provider: &models::Provider,
    default_model: &Option<String>,
) -> (String, String, models::Provider, Option<String>) {
    let mut name = fallback_name.to_string();
    let mut provider = default_provider.clone();
    let mut model = default_model.clone();

    let mut prompt_lines = Vec::new();

    for line in content.lines() {
        let trimmed = line.trim();
        if let Some(h1) = trimmed.strip_prefix("# ") {
            name = h1.trim().to_string();
            continue;
        }
        if let Some(comment) = trimmed.strip_prefix("<!--") {
            let comment = comment.trim_end_matches("-->").trim();
            if let Some(v) = comment.strip_prefix("model:") {
                let v = v.trim();
                if !v.is_empty() {
                    model = Some(v.to_string());
                }
                continue;
            }
            if let Some(v) = comment.strip_prefix("provider:") {
                let v = v.trim().to_lowercase();
                provider = match v.as_str() {
                    "openrouter" => models::Provider::Openrouter,
                    "openai" => models::Provider::Openai,
                    "copilot" => models::Provider::Copilot,
                    "claude" => models::Provider::Claude,
                    "ollama" => models::Provider::Ollama,
                    _ => default_provider.clone(),
                };
                continue;
            }
            continue;
        }
        prompt_lines.push(line);
    }

    let prompt = prompt_lines
        .iter()
        .skip_while(|l| l.trim().is_empty())
        .map(|l| l.to_string())
        .collect::<Vec<_>>()
        .join("\n")
        .trim()
        .to_string();

    let prompt = if prompt.is_empty() {
        "Write something about {{topic}}.".to_string()
    } else {
        prompt
    };

    (name, prompt, provider, model)
}