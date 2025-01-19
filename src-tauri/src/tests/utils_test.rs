#[cfg(test)]
mod tests {
    use crate::core_func::df_to_json;
    use polars::prelude::*;
    use serde_json::Value;
    use std::error::Error;

    /// Helper function to create a sample DataFrame
    fn create_sample_dataframe() -> DataFrame {
        df![
            "name" => ["Alice", "Bob", "Charlie"],
            "age" => [25, 30, 35],
            "score" => [85.5, 92.0, 88.0]
        ]
        .unwrap()
    }

    /// Test: Convert a valid DataFrame to JSON
    #[test]
    fn test_df_to_json_valid_dataframe() -> Result<(), Box<dyn Error>> {
        let df = create_sample_dataframe();
        let json_value = df_to_json(&df);

        // Expected JSON structure
        // let expected = serde_json::json!([
        //     {"name": "Alice", "age": 25, "score": 85.5},
        //     {"name": "Bob", "age": 30, "score": 92.0},
        //     {"name": "Charlie", "age": 35, "score": 88.0}
        // ]);
        //
        println!("{:?}", json_value);
        assert!(json_value.is_ok());
        // assert_eq!(json_value, expected);
        Ok(())
    }
}
