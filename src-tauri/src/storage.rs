use crate::models::{Agent, AppSettings};
use std::path::PathBuf;

pub fn app_dir() -> PathBuf {
    let base = dirs::data_dir().unwrap_or_else(|| PathBuf::from("."));
    base.join("agent-studio")
}

pub fn agents_path() -> PathBuf {
    app_dir().join("agents.json")
}

pub fn settings_path() -> PathBuf {
    app_dir().join("settings.json")
}

pub fn ensure_dir() -> std::io::Result<()> {
    let dir = app_dir();
    if !dir.exists() {
        std::fs::create_dir_all(&dir)?;
    }
    Ok(())
}

pub fn load_agents() -> Vec<Agent> {
    ensure_dir().ok();
    let path = agents_path();
    if !path.exists() {
        let seed = crate::models_seed::seed_agents();
        let _ = save_agents(&seed);
        return seed;
    }
    match std::fs::read_to_string(&path) {
        Ok(s) => serde_json::from_str(&s).unwrap_or_default(),
        Err(_) => Vec::new(),
    }
}

pub fn save_agents(agents: &[Agent]) -> std::io::Result<()> {
    ensure_dir()?;
    let path = agents_path();
    let s = serde_json::to_string_pretty(agents).unwrap();
    std::fs::write(&path, s)?;
    Ok(())
}

pub fn load_settings() -> AppSettings {
    ensure_dir().ok();
    let path = settings_path();
    if !path.exists() {
        let def = AppSettings::default();
        let _ = save_settings(&def);
        return def;
    }
    match std::fs::read_to_string(&path) {
        Ok(s) => serde_json::from_str(&s).unwrap_or_default(),
        Err(_) => AppSettings::default(),
    }
}

pub fn save_settings(s: &AppSettings) -> std::io::Result<()> {
    ensure_dir()?;
    let path = settings_path();
    let data = serde_json::to_string_pretty(s).unwrap();
    std::fs::write(&path, data)?;
    Ok(())
}