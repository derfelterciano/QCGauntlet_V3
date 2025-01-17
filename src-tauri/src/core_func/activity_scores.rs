use crate::core_func::utils::*;
use polars::prelude::*;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::error::Error;

pub fn get_activity_scores(
    primary_ds: DataFrame,
    secondary_ds: Option<DataFrame>,
    config: &UserConfig,
    exclude_experimental: bool,
) -> Result<(), Box<dyn Error>> {
    return Ok(());
}

/// Splits meta cols from feature df for calculations
fn parse_df(df: &DataFrame, meta_cols: &[&str]) -> Result<(DataFrame, DataFrame), Box<dyn Error>> {
    // get metal columns
    let meta_col_ptr: Vec<PlSmallStr> = meta_cols.iter().map(|x| PlSmallStr::from_str(x)).collect();
    let meta_df = df.select(meta_col_ptr)?;

    // identify feature columns
    let feature_cols: Vec<&str> = df
        .get_column_names()
        .par_iter()
        .map(|x| x.as_str())
        .filter(|x| !meta_cols.contains(x))
        .collect();

    let feature_cols_ptr: Vec<PlSmallStr> = feature_cols
        .par_iter()
        .map(|x| PlSmallStr::from_str(x))
        .collect();
    let feature_df = df.select(feature_cols_ptr)?;

    return Ok((meta_df, feature_df));
}
