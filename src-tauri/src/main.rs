// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod encrypto;

#[tauri::command]
async fn wrap_handle_data(input_seed: String, password: String) -> tauri::Result<String> {
    encrypto::handle_data(input_seed, password).await
}

#[tauri::command]
async fn wrap_minimalize_seeds(args: encrypto::MinimalizeSeedsArgs) -> tauri::Result<String> {
    encrypto::minimalize_seeds(args).await
}

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            wrap_handle_data,
            wrap_minimalize_seeds
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}