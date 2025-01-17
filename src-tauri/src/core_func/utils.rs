use polars::prelude::*;
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserConfig {
    primary_ds: String,
    secondary_ds: Option<String>,
    meta_cols: Option<Vec<String>>, // this is for uninformative meta
    threshold: f64,
    controls: Option<ControlDefinitions>,
    compound_name_col: String,
    well_location_col: String,
    plate_name_col: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlDefinitions {
    control_column: Option<String>,
    control_blocks: Option<Vec<ControlBlock>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlBlock {
    name: String,
    control_wells: Option<Vec<String>>,
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
