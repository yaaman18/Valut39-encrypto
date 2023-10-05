// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use std::path::PathBuf;

mod encrypto;

#[tauri::command]
async fn wrap_handle_data(input_seed: String, password: String) -> tauri::Result<String> {
    encrypto::handle_data(input_seed, password).await
}

#[tauri::command]
async fn wrap_minimalize_seeds(args: encrypto::MinimalizeSeedsArgs) -> tauri::Result<String> {
    encrypto::minimalize_seeds(args).await
}

#[tauri::command]
fn read_wordlist_file() -> tauri::Result<String> {
  let mut path_en = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
  path_en.push("src");
  path_en.push("resources");
  path_en.push("wordlist_en.txt");

  let contents = fs::read_to_string(path_en)?;
  Ok(contents)
}

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            wrap_handle_data,
            wrap_minimalize_seeds,
            read_wordlist_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}