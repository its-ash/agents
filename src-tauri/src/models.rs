use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
    pub id: String,
    pub name: String,
    pub prompt: String,
    #[serde(default)]
    pub model: Option<String>,
    #[serde(default)]
    pub provider: Provider,
    #[serde(default)]
    pub runs: Vec<Run>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Provider {
    #[default]
    Openrouter,
    Openai,
    Copilot,
    Claude,
    Ollama,
}

impl Provider {
    pub fn endpoint(&self) -> &str {
        match self {
            Provider::Openrouter => "https://openrouter.ai/api/v1/chat/completions",
            Provider::Openai => "https://api.openai.com/v1/chat/completions",
            _ => "",
        }
    }

    pub fn auth_header(&self, key: &str) -> String {
        match self {
            Provider::Openrouter => format!("Bearer {key}"),
            Provider::Openai => format!("Bearer {key}"),
            _ => String::new(),
        }
    }

    pub fn default_model(&self) -> &str {
        match self {
            Provider::Openrouter => "anthropic/claude-3.5-sonnet",
            Provider::Openai => "gpt-4o-mini",
            _ => "",
        }
    }

    pub fn is_api(&self) -> bool {
        matches!(self, Provider::Openrouter | Provider::Openai)
    }

    pub fn is_cli(&self) -> bool {
        matches!(self, Provider::Copilot | Provider::Claude | Provider::Ollama)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DetectedTools {
    pub copilot: bool,
    pub copilot_path: Option<String>,
    pub claude: bool,
    pub claude_path: Option<String>,
    pub ollama: bool,
    pub ollama_path: Option<String>,
    pub ollama_models: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Run {
    pub id: String,
    pub output: String,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,
    #[serde(default)]
    pub model: Option<String>,
    #[serde(default)]
    pub tokens: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    #[serde(default)]
    pub openrouter_key: Option<String>,
    #[serde(default)]
    pub openai_key: Option<String>,
    #[serde(default)]
    pub default_provider: Provider,
    #[serde(default)]
    pub default_model: Option<String>,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            openrouter_key: None,
            openai_key: None,
            default_provider: Provider::Openrouter,
            default_model: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CliTestResult {
    pub ok: bool,
    pub message: String,
    pub command: String,
    pub found: bool,
}

pub fn extract_placeholders(template: &str) -> Vec<String> {
    let re = regex_lite::Regex::new(r"\{\{\s*([a-zA-Z0-9_]+)\s*\}\}").unwrap();
    let mut seen = std::collections::HashSet::new();
    let mut out = Vec::new();
    for cap in re.captures_iter(template) {
        if let Some(name) = cap.get(1) {
            let n = name.as_str().to_string();
            if seen.insert(n.clone()) {
                out.push(n);
            }
        }
    }
    out
}

pub fn render_template(template: &str, values: &std::collections::HashMap<String, String>) -> String {
    let re = regex_lite::Regex::new(r"\{\{\s*([a-zA-Z0-9_]+)\s*\}\}").unwrap();
    re.replace_all(template, |caps: &regex_lite::Captures| {
        let key = caps.get(1).unwrap().as_str();
        values.get(key).cloned().unwrap_or_else(|| format!("{{{key}}}"))
    })
    .to_string()
}