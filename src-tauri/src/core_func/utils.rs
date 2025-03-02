use polars::prelude::*;
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::error::Error;
use std::sync::Mutex;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserConfig {
    pub primary_ds: String,
    pub secondary_ds: Option<String>,
    pub meta_cols: Option<Vec<String>>, // this is for uninformative meta
    pub threshold: f64,
    pub controls: Option<ControlDefinitions>,
    pub compound_name_col: String,
    pub well_location_col: String,
    pub plate_name_col: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlDefinitions {
    pub control_column: Option<String>,
    pub control_blocks: Option<Vec<ControlBlock>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlBlock {
    pub name: String,
    pub control_wells: Option<Vec<String>>,
}

pub fn calculate_scores(df: &DataFrame) -> Result<Series, Box<dyn Error>> {
    let scores: Vec<f64> = df
        .get_columns()
        .iter()
        .map(|col| col.f64().unwrap().into_iter().map(|opt| opt.unwrap_or(0.0)))
        .fold(None, |acc: Option<Vec<f64>>, col_iter| {
            Some(match acc {
                Some(sum_vec) => sum_vec
                    .iter()
                    .zip(col_iter)
                    .map(|(sum, val)| sum + val.powi(2))
                    .collect(),
                None => col_iter.map(|val| val.powi(2)).collect(),
            })
        })
        .unwrap()
        .into_par_iter()
        .map(|sum| sum.sqrt())
        .collect();
    return Ok(Series::new("Activity_Scores".into(), scores));
}

pub fn df_to_json(df: &DataFrame) -> Result<String, Box<dyn Error>> {
    let col_names = df
        .get_column_names()
        .par_iter()
        .map(|x| PlSmallStr::as_str(x))
        .collect::<Vec<&str>>();

    let json_rows: Mutex<Vec<serde_json::Value>> = Mutex::new(Vec::new());

    (0..df.height()).into_par_iter().for_each(|i| {
        let mut row_map = serde_json::Map::new();

        for col in &col_names {
            let raw_col = df.column(col).expect("col not found");
            let val = raw_col.get(i).expect("cant retrieve value");

            row_map.insert(col.to_string(), json!(val.to_string()));
        }

        let mut rows = json_rows.lock().unwrap();
        rows.push(json!(row_map));
    });

    let json_str = serde_json::to_string_pretty(&*json_rows.lock().unwrap())?;

    // let json_str = serde_json::to_string(&df).unwrap();

    return Ok(json_str);
}
