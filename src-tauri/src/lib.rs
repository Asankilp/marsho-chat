use configs::config::{load_marsho_config, load_model_config, save_marsho_config, MarshoConfig};
use futures_util::StreamExt;
use handlers::handler::MarshoHandler;
use models::context::MarshoContext;
use serde_json::Value;
use tauri::ipc::Channel;
use tauri::AppHandle;
mod configs;
mod handlers;
mod models;
mod schemas;
mod utils;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

#[derive(Clone, serde::Serialize)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase", tag = "event", content = "data")]
enum ChatEvent {
    Started {
        question: String
    },
    Stopped {
        message: Value
    },
}


#[tauri::command]
fn get_configs() -> Result<Value, String> {
    let marsho_config = load_marsho_config().map_err(|e| e.to_string())?;
    let model_config = load_model_config().map_err(|e| e.to_string())?;

    let mut config = serde_json::json!({});
    config["marsho"] = serde_json::to_value(marsho_config).map_err(|e| e.to_string())?;
    config["model"] = serde_json::to_value(model_config).map_err(|e| e.to_string())?;

    Ok(config)
}

#[tauri::command]
async fn save_configs(marsho_config: Value) -> Result<(), String> {
    save_marsho_config(&marsho_config).unwrap();

    Ok(())
}

#[tauri::command]
async fn make_chat(question: String, on_event: Channel<ChatEvent>) {
    let marsho_config = load_marsho_config().unwrap_or_else(|_| MarshoConfig::default());

    let model_config = load_model_config().unwrap();

    let context = MarshoContext::new();
    let mut handler = MarshoHandler::new(marsho_config, model_config);
    println!("aaaaa");
    on_event.send(ChatEvent::Started {
        question: question.clone(),
    }).unwrap();
    let mut results = handler.handle(question.to_string(), context).await;
    while let Some(message) = results.next().await {
        on_event.send(ChatEvent::Stopped {
            message: message,
        }).unwrap();
        
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let _ = load_marsho_config();
    let _ = load_model_config();
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![make_chat, get_configs, save_configs])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
