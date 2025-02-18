use crate::api::{PostProcessReader};

use crate::datamodel::Weight;
use crate::{ApiError, Phase, PostProcess};
use ndarray::{Array1, Array2, ArrayView3, Axis};

#[derive(Debug)]
pub struct ConcatPostPrcess {
    dataset: Vec<PostProcess>,
}

impl ConcatPostPrcess {
    pub fn new(folder: &[&str], root: Option<String>) -> Result<Self, String> {
        if folder.len() > 1 {
            let dataset: Vec<PostProcess> = folder
                .iter()
                .map(|f| PostProcess::new(f, root.clone()))
                .collect::<Result<Vec<_>, _>>()?; // Collect into a Result and propagate errors
            if dataset.is_empty() {
                return Err("Need at least one file".to_string());
            }
            Ok(Self { dataset })
        } else {
            Err("Need at least one file".to_string())
        }
    }

    /// Retrieves the last time value from each dataset in the collection.
    ///
    /// # Returns
    /// * `Result<Vec<f64>, String>` - A vector of the last time values for each dataset or an error message if any dataset is empty.
    pub fn get_time_end(&self) -> Result<Vec<f64>, String> {
        self.dataset
            .iter()
            .map(|ds| {
                ds.time()
                    .last()
                    .copied()
                    .ok_or_else(|| "Dataset has no time values".to_string())
            })
            .collect()
    }
}

impl PostProcessReader for ConcatPostPrcess {
    fn time(&self) -> &[f64] {
        todo!()
    }

    fn get_concentrations(&self, phase: Phase) -> ArrayView3<f64> {
        todo!()
    }

    fn get_spatial_average_biomass_concentration(&self) -> Result<Array1<f64>, ApiError> {
        let mut concatenated = Array1::<f64>::default(0);
        for postprocess in &self.dataset {
            match postprocess.get_spatial_average_biomass_concentration() {
                Ok(data) => concatenated.append(Axis(0), data.view()).unwrap(),
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Ok(concatenated)
    }

    fn get_property_names(&self) -> Vec<String> {
        self.dataset[0].get_property_names() //Names SHOULD be the same
    }

    /// Concatenates the time arrays from all datasets into a single array view.
    ///
    /// # Returns
    /// * `ArrayView1<f64>` - A concatenated array view of time data.
    fn time_array(&self) -> Array1<f64> {
        let concatenated: Vec<f64> = self
            .dataset
            .iter()
            .flat_map(|postprocess| postprocess.time_array().to_vec())
            .collect();
        Array1::from_vec(concatenated)
    }

    fn get_max_n_export_bio(&self) -> usize {
        self.dataset
            .iter()
            .map(|postprocess| postprocess.get_max_n_export_bio())
            .sum()
    }

    fn n_export(&self) -> usize {
        self.dataset
            .iter()
            .map(|postprocess| postprocess.n_export())
            .sum()
    }

    fn get_spatial_average_concentration(&self, species: usize, phase: Phase) -> Array1<f64> {
        let mut concatenated = Array1::<f64>::default(0);
        for postprocess in &self.dataset {
            let data = postprocess.get_spatial_average_concentration(species, phase);
            concatenated.append(Axis(0), data.view()).unwrap();
        }
        concatenated
    }

    fn get_time_average_concentration(
        &self,
        species: usize,
        position: usize,
        phase: Phase,
    ) -> Result<Array1<f64>, ApiError> {
        let mut concatenated = Array1::<f64>::default(0);
        for postprocess in &self.dataset {
            match postprocess.get_time_average_concentration(species, position, phase) {
                Ok(data) => concatenated.append(Axis(0), data.view()).unwrap(),
                Err(e) => return Err(e),
            }
        }
        Ok(concatenated)
    }

    fn get_spatial_average_mtr(&self, species: usize) -> Result<Array1<f64>, ApiError> {
        let mut concatenated = Array1::<f64>::default(0);
        for postprocess in &self.dataset {
            match postprocess.get_spatial_average_mtr(species) {
                Ok(data) => {
                    concatenated.append(Axis(0), data.view()).unwrap();
                }
                e => {
                    return e;
                }
            };
        }
        Ok(concatenated)
    }

    fn get_biomass_concentration(&self) -> Result<Array2<f64>, ApiError> {
        let mut concatenated = Array2::<f64>::default((0, 0));
        let mut init = false;
        for postprocess in &self.dataset {
            match postprocess.get_biomass_concentration() {
                Ok(data) => {
                    if !init {
                        concatenated = data;
                        init = true;
                    } else if let Err(err) = concatenated.append(Axis(0), data.view()) {
                        return Err(ApiError::Default(err.to_string()));
                    }
                }

                Err(e) => return Err(e),
            }
        }
        Ok(concatenated)
    }

    fn get_growth_in_number(&self) -> Array1<f64> {
        todo!()
    }

    fn weight(&self) -> &Weight {
        //FIXME
        self.dataset[0].weight()
    }

    fn get_number_particle(&self) -> &Array2<f64> {
        todo!()
    }

    fn get_properties(&self, key: &str, i_export: usize) -> Result<Array1<f64>, ApiError> {
        todo!()
    }

    fn get_time_population_mean(&self, key: &str) -> Result<Array1<f64>, ApiError> {
        todo!()
    }

    fn get_histogram_array(
        &self,
        n_bins: usize,
        i_export: usize,
        key: &str,
    ) -> Result<(Array1<f64>, Array1<f64>), ApiError> {
        todo!()
    }

    fn get_histogram(
        &self,
        n_bins: usize,
        i_export: usize,
        key: &str,
    ) -> Result<(Vec<f64>, Vec<f64>), ApiError> {
        todo!()
    }

    fn get_population_mean(&self, key: &str, i_export: usize) -> Result<f64, ApiError> {
        todo!()
    }
}
