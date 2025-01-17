#[cfg(test)]
mod calc_test {
    use crate::core_func::calculate_scores;
    use polars::prelude::*;
    use std::error::Error;

    //rounds vec
    fn round_series(series: &Series, decimal_places: u32) -> Vec<f64> {
        let factor = 10_f64.powi(decimal_places as i32);
        series
            .f64()
            .unwrap()
            .into_no_null_iter()
            .map(|val| (val * factor).round() / factor)
            .collect()
    }

    // simple test
    #[test]
    fn test_row_wise_basic() -> Result<(), Box<dyn Error>> {
        // Create a sample DataFrame without an index column
        let df = df![
            "feature1" => [1.0, 2.0, 3.0],
            "feature2" => [4.0, 5.0, 6.0],
            "feature3" => [7.0, 8.0, 9.0],
        ]?;

        // Call the calculate_scores function
        let result = calculate_scores(&df)?;

        // Expected output
        let expected = Series::new("score".into(), vec![8.124038, 9.643651, 11.224972]);

        // test lens
        assert_eq!(result.len(), expected.len());

        // Assert equality
        assert_eq!(round_series(&result, 6), round_series(&expected, 6));
        Ok(())
    }

    // with negatives
    #[test]
    fn test_row_wise_with_negatives() -> Result<(), Box<dyn Error>> {
        let df = df![
            "feature1" => [-1.0, -2.0, -3.0],
            "feature2" => [-4.0, -5.0, -6.0],
            "feature3" => [-7.0, -8.0, -9.0],
        ]?;

        let result = calculate_scores(&df)?;

        let expected = Series::new("score".into(), vec![8.124038, 9.643651, 11.224972]);
        assert_eq!(round_series(&result, 6), round_series(&expected, 6));
        Ok(())
    }

    #[test]
    fn test_row_wise_mixed_values() -> Result<(), Box<dyn Error>> {
        let df = df![
            "feature1" => [1.0, -2.0, 3.0],
            "feature2" => [-4.0, 5.0, -6.0],
            "feature3" => [7.0, -8.0, 9.0],
        ]?;

        let result = calculate_scores(&df)?;

        let expected = Series::new("score".into(), vec![8.124038, 9.643651, 11.224972]);
        assert_eq!(round_series(&result, 6), round_series(&expected, 6));

        Ok(())
    }
}
