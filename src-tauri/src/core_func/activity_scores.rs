use crate::core_func::utils::*;
use polars::prelude::*;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::error::Error;

/// Creates Activity scores for given datasets
pub fn get_activity_scores(
    primary_ds: DataFrame,
    secondary_ds: Option<DataFrame>,
    config: &UserConfig,
) -> Result<(DataFrame, Option<DataFrame>), Box<dyn Error>> {
    let mut meta_cols = vec![
        config.compound_name_col.as_str(),
        config.well_location_col.as_str(),
        config.plate_name_col.as_str(),
    ];
    if let Some(meta) = &config.meta_cols {
        meta_cols.extend(meta.iter().map(|x| x.as_str()).collect::<Vec<&str>>());
    }

    let (primary_ds_meta, primary_ds_feats) = parse_df(&primary_ds, &meta_cols)?;
    let primary_scores = calculate_scores(&primary_ds_feats)?;
    let primary_res = primary_ds_meta.hstack(&[primary_scores.into()])?;

    if let Some(second_ds) = secondary_ds {
        let (secondary_ds_meta, secondary_ds_feats) = parse_df(&second_ds, &meta_cols)?;
        let secondary_scores = calculate_scores(&secondary_ds_feats)?;
        let secondary_res = secondary_ds_meta.hstack(&[secondary_scores.into()])?;

        return Ok((primary_res, Some(secondary_res)));
    }

    return Ok((primary_res, None));
}

// Reads the user config path and then returns the respective dataframes
pub fn read_ds(config: &UserConfig) -> Result<(DataFrame, Option<DataFrame>), Box<dyn Error>> {
    let primary_path = config.primary_ds.clone();
    let primary_df = CsvReadOptions::default()
        .with_has_header(true)
        .try_into_reader_with_file_path(Some(primary_path.into()))?
        .finish()?;

    let sercondary_df = if let Some(secondary_path) = config.secondary_ds.clone() {
        Some(
            CsvReadOptions::default()
                .with_has_header(true)
                .try_into_reader_with_file_path(Some(secondary_path.into()))?
                .finish()?,
        )
    } else {
        None
    };

    return Ok((primary_df, sercondary_df));
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
