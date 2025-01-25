#![allow(unused_imports, dead_code)]

use core_func::*;
use std::error::Error;

// use core_func::activity_scores::*;
// use core_func::df_to_json;
// use core_func::UserConfig;
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
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, process_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// #[cfg(test)]
// mod real_data_test {
//     use crate::{process_data, read_ds, UserConfig};
//     use polars::io::json::JsonReader;
//     use polars::prelude::*;
//     use serde_json::Value;
//     use std::collections::HashMap;
//     use std::io::Cursor;
//
//     fn flatten_json(json_data: Vec<HashMap<String, Value>>) -> (Vec<(String, Vec<String>)>, usize) {
//         let mut columns: HashMap<String, Vec<String>> = HashMap::new();
//
//         for record in json_data {
//             for (key, value) in record {
//                 let column = columns.entry(key).or_insert_with(Vec::new);
//                 column.push(value.to_string());
//             }
//         }
//
//         let num_rows = columns.values().next().map(|v| v.len()).unwrap_or(0);
//         (columns.into_iter().collect(), num_rows)
//     }
//
//     #[test]
//     fn test_data() {
//         let test_config = UserConfig {
//             primary_ds: "/home/derfelt/LokeyLabFiles/QCG_TestData/primary_ds.csv".to_string(),
//             secondary_ds: Some(
//                 "/home/derfelt/LokeyLabFiles/QCG_TestData/secondary_ds.csv".to_string(),
//             ),
//             meta_cols: None,
//             controls: None,
//             threshold: 0.5,
//             compound_name_col: "longname_proper".to_string(),
//             well_location_col: "well".to_string(),
//             plate_name_col: "plate".to_string(),
//         };
//
//         match process_data(test_config) {
//             Ok((prim_ds, sec_ds)) => {
//                 assert!(!prim_ds.is_empty());
//
//                 if !sec_ds.is_empty() {
//                     std::fs::write("./bs.json", &sec_ds).expect("fail");
//
//                     // // Use JsonReader to parse the JSON string
//                     // let cursor = Cursor::new(sec_ds);
//                     // let mutdf = JsonReader::new(cursor)
//                     //     .finish()
//                     //     .expect("Failed to parse JSON into DataFrame");
//                     //
//                     // // Print and validate the flattened DataFrame
//                     // println!("{:?}", df);
//                     // assert!(
//                     //     df.height() > 0,
//                     //     "Secondary dataset is empty after flattening"
//                     // );
//                 }
//             }
//
//             Err(err) => {
//                 panic!("ERROR! : {}", err);
//             }
//         }
//     }
// }
