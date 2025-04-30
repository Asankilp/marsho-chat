use configs::config::{load_marsho_config, load_model_config, MarshoConfig};
use handlers::handler::MarshoHandler;
use models::context::MarshoContext;
use serde_json::Value;
mod configs;
mod handlers;
mod models;
mod schemas;
mod utils;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/


#[tauri::command]
async fn make_chat(name: &str) -> Result<String, String> {
    let mut marsho_config = load_marsho_config().unwrap_or_else(|_| MarshoConfig::default());

    let mut model_config = load_model_config().unwrap();

    let context = MarshoContext::new();
    let mut handler = MarshoHandler::new(marsho_config, model_config);
    
    handler.handle(name.to_string(), context, false)
        .await
        .map_err(|e| e.to_string())
        .and_then(|chat| {
            chat["choices"][0]["message"]["content"]
                .as_str()
                .map(|s| s.to_string())
                .ok_or_else(|| "无法获取响应内容".to_string())
        })
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    load_marsho_config();
    load_model_config();
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![make_chat])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
