#[cfg(test)]
mod as_tests {
    use crate::core_func::activity_scores::*;
    use crate::core_func::UserConfig;
    use polars::prelude::*;
    use std::error::Error;
    use std::usize;

    #[test]
    fn test_get_activity_scores_single_dataset() -> Result<(), Box<dyn Error>> {
        let primary_df = df![
            "compound" => ["C1", "C2", "C3"],
            "well" => ["A1", "A2", "A3"],
            "plate" => ["P1", "P1", "P1"],
            "feature1" => [1.0, 2.0, 3.0],
            "feature2" => [4.0, 5.0, 6.0]
        ]?;

        let config = UserConfig {
            primary_ds: "path/to/primary.csv".to_string(),
            secondary_ds: None,
            compound_name_col: "compound".to_string(),
            well_location_col: "well".to_string(),
            plate_name_col: "plate".to_string(),
            meta_cols: None,
            threshold: 0.5,
            controls: None,
        };

        let (primary_res, secondary_res) = get_activity_scores(primary_df, None, &config)?;

        // Secondary dataset should be None
        assert!(secondary_res.is_none());

        // Primary result should have an additional column for activity scores
        assert_eq!(primary_res.shape(), (3 as usize, 4 as usize));

        println!("{:?}", primary_res);
        Ok(())
    }

    #[test]
    fn test_get_activity_scores_primary_and_secondary() -> Result<(), Box<dyn Error>> {
        let primary_df = df![
            "compound" => ["C1", "C2", "C3"],
            "well" => ["A1", "A2", "A3"],
            "plate" => ["P1", "P1", "P1"],
            "feature1" => [1.0, 2.0, 3.0],
            "feature2" => [4.0, 5.0, 6.0]
        ]?;

        let secondary_df = df![
            "compound" => ["C4", "C5", "C6"],
            "well" => ["B1", "B2", "B3"],
            "plate" => ["P2", "P2", "P2"],
            "feature1" => [3.0, 2.0, 1.0],
            "feature2" => [6.0, 5.0, 4.0]
        ]?;

        let config = UserConfig {
            primary_ds: "path/to/primary.csv".to_string(),
            secondary_ds: Some("path/to/secondary.csv".to_string()),
            compound_name_col: "compound".to_string(),
            well_location_col: "well".to_string(),
            plate_name_col: "plate".to_string(),
            meta_cols: None,
            threshold: 0.5,
            controls: None,
        };

        let (primary_res, secondary_res) =
            get_activity_scores(primary_df, Some(secondary_df), &config)?;

        // Both primary and secondary datasets should be processed
        assert_eq!(primary_res.shape(), (3 as usize, 4 as usize));
        assert_eq!(secondary_res.unwrap().shape(), (3 as usize, 4 as usize));
        Ok(())
    }

    #[test]
    fn test_get_activity_scores_with_meta_cols() -> Result<(), Box<dyn Error>> {
        let primary_df = df![
            "compound" => ["C1", "C2", "C3"],
            "well" => ["A1", "A2", "A3"],
            "plate" => ["P1", "P1", "P1"],
            "feature1" => [1.0, 2.0, 3.0],
            "feature2" => [4.0, 5.0, 6.0]
        ]?;

        let config = UserConfig {
            primary_ds: "path/to/primary.csv".to_string(),
            secondary_ds: None,
            compound_name_col: "compound".to_string(),
            well_location_col: "well".to_string(),
            plate_name_col: "plate".to_string(),
            meta_cols: Some(vec!["feature2".to_string()]), // Exclude feature2
            threshold: 0.5,
            controls: None,
        };

        let (primary_res, _) = get_activity_scores(primary_df, None, &config)?;

        // Check that excluded columns are not in the resulting scores
        let col_names = primary_res
            .get_column_names()
            .iter()
            .map(|x| x.as_str().contains("feature2"))
            .any(|x| x);
        assert!(col_names);

        Ok(())
    }

    #[test]
    fn test_get_activity_scores_invalid_meta_cols() {
        let primary_df = df![
            "compound" => ["C1", "C2", "C3"],
            "well" => ["A1", "A2", "A3"],
            "plate" => ["P1", "P1", "P1"],
            "feature1" => [1.0, 2.0, 3.0],
            "feature2" => [4.0, 5.0, 6.0]
        ]
        .unwrap();

        let config = UserConfig {
            primary_ds: "path/to/primary.csv".to_string(),
            secondary_ds: None,
            compound_name_col: "invalid_col".to_string(), // Invalid column
            well_location_col: "well".to_string(),
            plate_name_col: "plate".to_string(),
            meta_cols: None,
            threshold: 0.5,
            controls: None,
        };

        let result = get_activity_scores(primary_df, None, &config);
        assert!(result.is_err());
    }

    #[test]
    fn test_get_activity_scores_empty_dataset() -> Result<(), Box<dyn Error>> {
        let primary_df = DataFrame::empty();

        let config = UserConfig {
            primary_ds: "path/to/primary.csv".to_string(),
            secondary_ds: None,
            compound_name_col: "compound".to_string(),
            well_location_col: "well".to_string(),
            plate_name_col: "plate".to_string(),
            meta_cols: None,
            threshold: 0.5,
            controls: None,
        };

        let result = get_activity_scores(primary_df, None, &config);

        assert!(result.is_err());
        Ok(())
    }
}
