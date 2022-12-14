#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri_plugin_main_js::include_main;

fn main() {
  let js_files = include_main!("index.ts");

  tauri::Builder::default()
    .plugin(tauri_plugin_main_js::init())
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
