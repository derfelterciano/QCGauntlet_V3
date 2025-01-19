#![allow(unused_imports, dead_code)]

use std::error::Error;

use core_func::activity_scores::*;
use core_func::df_to_json;
use core_func::UserConfig;
pub mod core_func;
mod tests;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn process_data(config: UserConfig) -> Result<(String, String), String> {
    let (primary_df, secondary_df) = read_ds(&config).unwrap();

    let (primary_scores, secondary_scores) =
        get_activity_scores(primary_df, secondary_df, &config).unwrap();

    let primary_json = df_to_json(&primary_scores).unwrap();

    let secondary_json = match secondary_scores {
        Some(ref scores) => df_to_json(scores).unwrap(),
        None => "".into(),
    };

    return Ok((primary_json, secondary_json));
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, process_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
