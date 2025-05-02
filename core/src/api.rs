use crate::datamodel::{Weight,tallies::Tallies};

use crate::error::ApiError;
use ndarray::{Array1, Array2, ArrayView3};

/// `Phase` enum represents different states or phases of a substance.
#[derive(Clone, PartialEq, Copy)]
pub enum Phase {
    Liquid,
    Gas,
}

/// Type of estimator to retrieve data from MC Particle
#[derive(Copy, Clone)]
pub enum Estimator {
    MonteCarlo,
    Weighted,
}

/// A trait for postprocessing operations on simulation results.
///
/// This trait defines various methods for analyzing and retrieving data from simulation results.
pub trait PostProcessReader {
    /// Returns a reference to the time data from the simulation results.
    ///
    /// # Returns
    /// * `&[f64]` - A slice containing the time data.
    fn time(&self) -> &[f64];

    fn weight(&self) -> &Weight;

    fn tallies(&self) -> Option<&Tallies>;

    /// Returns a 1D array view of the time data from the simulation results.
    ///
    /// # Returns
    /// * `ArrayView1<f64>` - A 1D array view containing the time data.
    fn time_array(&self) -> Array1<f64>;

    /// Retrieves the maximum number of biological export events.
    ///
    /// This method determines the maximum number of export events specifically related
    /// to biological data dumps.
    ///
    /// # Returns
    /// * `usize` - The number of biological export events, or `0` if no events are found.
    fn get_max_n_export_bio(&self) -> usize;

    /// Retrieves the total number of export events from the simulation results.
    ///
    /// This count includes all types of export actions.
    ///
    /// # Returns
    /// * `usize` - The total number of export events.
    fn n_export(&self) -> usize;

    fn get_property_names(&self) -> Vec<String>;

    /// Computes the spatial average concentration for a specific species and phase.
    ///
    /// # Arguments
    /// * `species` - The index of the species for which to calculate the average.
    /// * `phase` - The phase (e.g., liquid or gas) to consider.
    ///
    /// # Returns
    /// * `Array1<f64>` - A 1D array containing the spatial average concentrations over time.
    fn get_spatial_average_concentration(&self, species: usize, phase: Phase) -> Array1<f64>;

    fn get_spatial_average_property(&self, key:&str) ->  Result<Array2<f64>, ApiError>;

    fn get_spatial_average_biomass_concentration(&self) -> Result<Array1<f64>, ApiError>;

    fn get_concentrations(&self, phase: Phase) -> ArrayView3<f64>;

    fn get_spatial_average_mtr(&self, species: usize) -> Result<Array1<f64>, ApiError>;

    /// Computes the time average concentration for a specific species, position, and phase.
    ///
    /// # Arguments
    /// * `species` - The index of the species for which to calculate the average.
    /// * `position` - The position in the simulation domain to consider.
    /// * `phase` - The phase (e.g., liquid or gas) to consider.
    ///
    /// # Returns
    /// * `Result<Array1<f64>, String>` - A 1D array containing the time average concentrations,
    ///   or an error message if the calculation fails.
    fn get_time_average_concentration(
        &self,
        species: usize,
        position: usize,
        phase: Phase,
    ) -> Result<Array1<f64>, ApiError>;

    /// Calculates the biomass concentration over time.
    ///
    /// # Returns
    /// * `Result<Array2<f64>, String>` - A 2D array containing biomass concentrations over time,
    ///   or an error message if the calculation fails.
    fn get_biomass_concentration(&self) -> Result<Array2<f64>, ApiError>;

    fn get_probes(&self) -> Result<Array1<f64>, ApiError>;
    

    /// Calculates the total growth in number.
    ///
    /// # Returns
    /// * `Array1<f64>` - A 1D array containing the summed growth numbers over time.
    fn get_growth_in_number(&self) -> Array1<f64>;

    /// Retrieves a reference to the 2D array of particle numbers.
    ///
    /// # Returns
    /// * `&Array2<f64>` - A reference to the 2D array of particle numbers.
    fn get_number_particle(&self) -> &Array2<f64>;

    /// Fetches specific properties of the model at a given export index.
    ///
    /// # Arguments
    /// * `key` - The key identifying the property to retrieve.
    /// * `i_export` - The export index for which to retrieve the property.
    ///
    /// # Returns
    /// * `Result<Array1<f64>, String>` - A 1D array of property values,
    ///   or an error message if the retrieval fails.
    fn get_properties(&self, key: &str, i_export: usize) -> Result<Array1<f64>, ApiError>;

    /// Calculates the time-averaged population mean for a specific property key.
    ///
    /// # Arguments
    /// * `key` - The key identifying the property to average.
    ///
    /// # Returns
    /// * `Result<Array1<f64>, String>` - A 1D array of mean values over time,
    ///   or an error message if the calculation fails.
    fn get_time_population_mean(&self, key: &str) -> Result<Array1<f64>, ApiError>;

    /// Retrieves histogram data for a specific property key at a given export index.
    ///
    /// # Arguments
    /// * `n_bins` - The number of bins to use in the histogram.
    /// * `i_export` - The export index for which to retrieve the histogram.
    /// * `key` - The key identifying the property to calculate the histogram for.
    ///
    /// # Returns
    /// * `Result<(Array1<f64>, Array1<f64>), String>` - The histogram bins and counts,
    ///   or an error message if the calculation fails.
    fn get_histogram_array(
        &self,
        n_bins: usize,
        i_export: usize,
        key: &str,
    ) -> Result<(Array1<f64>, Array1<f64>), ApiError>;

    /// Retrieves histogram data for a specific property key at a given export index.
    ///
    /// # Arguments
    /// * `n_bins` - The number of bins to use in the histogram.
    /// * `i_export` - The export index for which to retrieve the histogram.
    /// * `key` - The key identifying the property to calculate the histogram for.
    ///
    /// # Returns
    /// * `Result<(Vec<f64>, Vec<f64>), String>` - The histogram bins and counts as vectors,
    ///   or an error message if the calculation fails.
    fn get_histogram(
        &self,
        n_bins: usize,
        i_export: usize,
        key: &str,
    ) -> Result<(Vec<f64>, Vec<f64>), ApiError>;

    /// Retrieves the population mean for a specific property key at a given export index.
    ///
    /// # Arguments
    /// * `key` - The key identifying the property to calculate the mean for.
    /// * `i_export` - The export index for which to calculate the mean.
    ///
    /// # Returns
    /// * `Result<f64, String>` - The population mean, or an error message if the calculation fails.
    fn get_population_mean(&self, key: &str, i_export: usize) -> Result<f64, ApiError>;
}

pub trait ModelEstimator {
    fn mu_direct(&self) -> Result<Array1<f64>, ApiError>;

    fn estimate(&self, etype: Estimator, key: &str, i_export: usize) -> Result<f64, ApiError>;

    fn estimate_time(&self, etype: Estimator, key: &str) -> Result<Array1<f64>, ApiError>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::Array1;

    #[test]
    fn test_monte_carlo_estimate() {
        let rx = Array1::from(vec![2.0, 4.0, 6.0]);
        let weight = 2.0;

        let result =
            crate::process::estimate(Estimator::MonteCarlo, &Weight::Single(weight), &rx).unwrap();
        let dim = rx.dim();
        let weighted_estimator: f64 = (rx * weight).sum(); // (2.0*2.0 + 4.0*2.0 + 6.0*2.0) = 24.0
        let normalization_factor = (0..dim).map(|_| weight).sum::<f64>(); // 2.0 * 3 = 6.0
        let expected_result = weighted_estimator / normalization_factor; // 24.0 / 6.0 = 4.0
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_weighted_estimate() {
        let rx = Array1::from(vec![2.0, 4.0, 6.0]);
        let weight = 2.0;

        let result =
            crate::process::estimate(Estimator::Weighted, &Weight::Single(weight), &rx).unwrap();

        let weighted_estimator: f64 = (rx * weight).sum(); // (2.0*2.0 + 4.0*2.0 + 6.0*2.0) = 24.0
        let expected_result = weighted_estimator; // Since it's weighted, it should return the weighted sum.
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_zero_weight() {
        let rx = Array1::from(vec![2.0, 4.0, 6.0]);
        let weight = 0.0;

        let result =
            crate::process::estimate(Estimator::MonteCarlo, &Weight::Single(weight), &rx).unwrap();
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_empty_array() {
        let rx = Array1::from(vec![]);
        let weight = 2.0;
        let result =
            crate::process::estimate(Estimator::Weighted, &Weight::Single(weight), &rx).unwrap();

        // With an empty array, the result should be zero
        assert_eq!(result, 0.0);
    }
}
