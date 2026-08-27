mod cli_runner;
mod commands;
mod detect;
mod llm;
mod models;
mod models_seed;
mod storage;

use commands::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .manage(AppState::new())
        .invoke_handler(tauri::generate_handler![
            commands::list_agents,
            commands::create_agent,
            commands::update_agent,
            commands::delete_agent,
            commands::get_placeholders,
            commands::run_agent,
            commands::get_settings,
            commands::save_settings,
            commands::detect_tools,
            commands::test_tool,
            commands::export_agents,
            commands::import_agents_from_file,
            commands::import_agents_from_repo,
        ])
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
