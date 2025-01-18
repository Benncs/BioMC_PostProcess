mod datamodel;
mod process;

use datamodel::{
    get_n_export_real, read_avg_model_properties, read_model_properties, vec_to_array_view2,
    vec_to_array_view3, Dim,
};
use datamodel::{make_histogram, Results};
use ndarray::{s, Array1, Array2, Axis};
use process::{spatial_average_concentration, Histogram};

/// `Phase` enum represents different states or phases of a substance.
#[derive(Clone, PartialEq, Copy)]
pub enum Phase {
    Liquid,
    Gas,
}

/// The `PostProcess` struct handles post-processing of simulation results.
///
/// It contains the path to the results folder, the root directory, and the processed results.
///
/// The struct uses the `Results` struct internally, which is expected to contain time and other data.
#[derive(Debug)]
pub struct PostProcess {
    folder: String,   // The folder where the simulation results are stored.
    root: String,     // The root directory for the results.
    dest: String,     // The destination path for output
    results: Results, // The results of the simulation, which will be accessed for time and other data.
}

#[derive(Debug)]
pub struct ConcatPostPrcess {
    dataset: Vec<PostProcess>,
}

/// A trait for postprocessing operations on simulation results.
///
/// This trait defines various methods for analyzing and retrieving data from simulation results.
pub trait PostProcessUser {
    /// Returns a reference to the time data from the simulation results.
    ///
    /// # Returns
    /// * `&[f64]` - A slice containing the time data.
    fn time(&self) -> &[f64];

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
    ) -> Result<Array1<f64>, String>;

    /// Calculates the biomass concentration over time.
    ///
    /// # Returns
    /// * `Result<Array2<f64>, String>` - A 2D array containing biomass concentrations over time,
    ///   or an error message if the calculation fails.
    fn get_biomass_concentration(&self) -> Result<Array2<f64>, String>;

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
    fn get_properties(&self, key: &str, i_export: usize) -> Result<Array1<f64>, String>;

    /// Calculates the time-averaged population mean for a specific property key.
    ///
    /// # Arguments
    /// * `key` - The key identifying the property to average.
    ///
    /// # Returns
    /// * `Result<Array1<f64>, String>` - A 1D array of mean values over time,
    ///   or an error message if the calculation fails.
    fn get_time_population_mean(&self, key: &str) -> Result<Array1<f64>, String>;

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
    ) -> Result<(Array1<f64>, Array1<f64>), String>;

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
    ) -> Result<(Vec<f64>, Vec<f64>), String>;

    /// Retrieves the population mean for a specific property key at a given export index.
    ///
    /// # Arguments
    /// * `key` - The key identifying the property to calculate the mean for.
    /// * `i_export` - The export index for which to calculate the mean.
    ///
    /// # Returns
    /// * `Result<f64, String>` - The population mean, or an error message if the calculation fails.
    fn get_population_mean(&self, key: &str, i_export: usize) -> Result<f64, String>;
}

impl PostProcess {
    /// Creates a new instance of `PostProcess`.
    ///
    /// # Arguments
    /// * `folder` - The name of the folder containing the simulation results.
    /// * `root` - Optional root directory. Defaults to "./results/" if not provided.
    ///
    /// # Returns
    /// * `Result<Self, String>` - Returns the `PostProcess` instance or an error message if initialization fails.
    pub fn new(folder: &str, root: Option<String>) -> Result<Self, String> {
        let _root = root.unwrap_or_else(|| "./results/".to_string());
        let result_path = format!("{}/{}/{}.h5", _root, folder, folder);
        let main = Results::new(&result_path, &_root, folder)?;
        Ok(Self {
            folder: folder.to_string(),
            root: _root,
            dest: String::new(),
            results: main,
        })
    }
}
impl PostProcessUser for PostProcess {
    fn time(&self) -> &[f64] {
        self.results.main.time()
    }

    fn get_property_names(&self) -> Vec<String> {
        self.results.get_property_name()
    }

    /// Returns an `ArrayView1<f64>` representing the time data from the simulation results.
    ///
    /// # Returns
    /// * An `ArrayView1<f64>` containing the time data.
    fn time_array(&self) -> Array1<f64> {
        let time = self.results.main.time();
        Array1::<f64>::from_vec(time.to_vec()) // TODO: safe unwrap
    }

    /// Retrieves the maximum number of export events specifically related to biological dumps from the simulation results.
    /// This value represents the real number of biological export events, which may not always align with the total number of exports.
    ///
    /// # Returns
    /// * `usize` - The maximum number of biological export events, or `0` if no events are found or if an error occurs.
    fn get_max_n_export_bio(&self) -> usize {
        get_n_export_real(self.results.get_files()).unwrap_or(0)
    }

    /// Returns the total number of export events from the simulation results.
    /// This count includes all export actions, regardless of whether they are biological or not.
    ///
    /// # Returns
    /// * `usize` - The total number of export events.
    fn n_export(&self) -> usize {
        self.results.main.records.time.len()
    }

    fn get_spatial_average_concentration(&self, species: usize, phase: Phase) -> Array1<f64> {
        // Helper
        fn process_phase(
            concentration: &Vec<f64>,
            volume: &Vec<f64>,
            nt: usize,
            dim: &Dim,
            species: usize,
        ) -> Array1<f64> {
            let cl = vec_to_array_view3(concentration, &dim, nt);
            let vol = vec_to_array_view2(volume, nt, dim.0);
            let res = spatial_average_concentration(&cl.slice(s![.., .., species]), &vol);

            res
        }

        let r = &self.results.main.records;
        let nt = r.time.len();
        let dim = &r.dim;

        match phase {
            Phase::Gas => {
                if let (Some(c), Some(v)) = (&r.concentration_gas, &r.volume_gas) {
                    return process_phase(c, v, nt, dim, species);
                }

                panic!("Gas is not present");
            }
            Phase::Liquid => {
                process_phase(&r.concentration_liquid, &r.volume_liquid, nt, dim, species)
            }
        }
    }

    fn get_time_average_concentration(
        &self,
        species: usize,
        position: usize,
        phase: Phase,
    ) -> Result<Array1<f64>, String> {
        let r = &self.results.main.records;
        let nt = r.time.len();
        let dim = &r.dim;

        let callback = |c: &Vec<f64>| {
            let cl = vec_to_array_view3(c, dim, nt);
            cl.slice(s![.., .., species]).mean_axis(Axis(0)).unwrap()
        };

        match phase {
            Phase::Liquid => Ok(callback(&r.concentration_liquid)),
            Phase::Gas => {
                if let Some(c) = &r.concentration_gas {
                    return Ok(callback(c));
                }

                Err("Gas is not present".to_string())
            }
        }
    }

    /// Calculates the biomass concentration over time for the simulation.
    ///
    /// # Returns
    /// * `Result<Array2<f64>, String>` - A 2D array with biomass concentrations or an error message.
    fn get_biomass_concentration(&self) -> Result<Array2<f64>, String> {
        let nt = self.results.main.records.time.len(); // Number of time steps
        let num_dimensions = self.results.main.records.dim.0; // Dimensionality

        // Initialize the biomass matrix
        let mut biomass_matrix = Array2::zeros((nt, num_dimensions));

        // Attempt to read model mass
        if let Err(err) =
            datamodel::read_model_mass(self.results.get_files(), &mut biomass_matrix, nt)
        {
            return Err(format!("Failed to read model mass: {:?}", err));
        }

        // Convert volume to an array view
        let volume =
            vec_to_array_view2(&self.results.main.records.volume_liquid, nt, num_dimensions);

        // Calculate biomass concentration
        biomass_matrix = biomass_matrix * self.results.main.initial.initial_weight / volume;

        Ok(biomass_matrix)
    }

    /// Calculates the total growth in number
    ///
    /// # Returns
    /// * `Array1<f64>` - 1D array containing the summed growth numbers.
    fn get_growth_in_number(&self) -> Array1<f64> {
        self.results.total_particle_repetition.sum_axis(Axis(1))
    }

    /// Retrieves the 2D array of particle numbers.
    ///
    /// # Returns
    /// * `&Array2<f64>` - Reference to the 2D array containing particle numbers.
    fn get_number_particle(&self) -> &Array2<f64> {
        &self.results.total_particle_repetition
    }

    /// Fetches specific properties of the model at a given export index.
    ///
    /// # Arguments
    /// * `key` - The property key to fetch.
    /// * `i_export` - The index of the export to retrieve.
    ///
    /// # Returns
    /// * `Result<Array1<f64>, String>` - A 1D array of property values or an error message.
    fn get_properties(&self, key: &str, i_export: usize) -> Result<Array1<f64>, String> {
        if i_export >= self.results.main.records.time.len() {
            return Err(format!(
                "Index out of range: i_export ({}) exceeds available records ({}).",
                i_export,
                self.results.main.records.time.len()
            ));
        }

        match read_model_properties(key, self.results.get_files(), i_export) {
            Ok(res) => Ok(res),
            Err(e) => Err(format!("Failed to read model properties: {:?}", e)),
        }
    }

    /// Calculates the population mean over time for a given property key.
    ///
    /// # Arguments
    /// * `key` - The property key for which to calculate the mean.
    ///
    /// # Returns
    /// * `Result<Array1<f64>, String>` - A 1D array of mean values over time or an error message.
    fn get_time_population_mean(&self, key: &str) -> Result<Array1<f64>, String> {
        match read_avg_model_properties(key, self.results.get_files(), self.n_export()) {
            Ok(res) => Ok(res),
            Err(e) => Err(format!(
                "Failed to calculate time population mean for key '{}': {:?}",
                key, e
            )),
        }
    }

    fn get_histogram_array(
        &self,
        n_bins: usize,
        i_export: usize,
        key: &str,
    ) -> Result<(Array1<f64>, Array1<f64>), String> {
        let (b, c) = self.get_histogram(n_bins, i_export, key)?;
        let b = Array1::from_vec(b);
        let c = Array1::from_vec(c);
        Ok((b, c))
    }

    fn get_histogram(
        &self,
        n_bins: usize,
        i_export: usize,
        key: &str,
    ) -> Result<(Vec<f64>, Vec<f64>), String> {
        if i_export > self.n_export() {
            return Err("Out of range".to_owned());
        }

        // let np = n_bins;//*self.results.total_particle_repetition.sum_axis(Axis(1)).last().unwrap() as usize;
        let mut hist = Histogram::new(n_bins);

        let _ = make_histogram(self.results.get_files(), i_export, key, &mut hist);

        let b = hist.get_bins().to_vec();
        let c = hist.get_counts().to_vec();
        Ok((b, c))
    }

    fn get_population_mean(&self, key: &str, i_export: usize) -> Result<f64, String> {
        // Check if the index is out of range
        if i_export >= self.results.main.records.time.len() {
            return Err("Out of range".to_string());
        }

        match read_model_properties(key, self.results.get_files(), i_export) {
            Ok(res) => res.mean().ok_or("mean error".to_string()),
            Err(e) => Err(e.to_string()),
        }
    }
}

impl ConcatPostPrcess {
    pub fn new(folder: &[&str], root: Option<String>) -> Result<Self, String> {
        if folder.len() > 1 {
            let dataset: Vec<PostProcess> = folder
                .iter()
                .map(|f| PostProcess::new(f, root.clone()))
                .collect::<Result<Vec<_>, _>>()?; // Collect into a Result and propagate errors
            if dataset.is_empty()
            {
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

impl PostProcessUser for ConcatPostPrcess {
    fn time(&self) -> &[f64] {
        todo!()
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
    ) -> Result<Array1<f64>, String> {
        let mut concatenated = Array1::<f64>::default(0);
        for postprocess in &self.dataset {
            match postprocess.get_time_average_concentration(species, position, phase) {
                Ok(data) => concatenated.append(Axis(0), data.view()).unwrap(),
                Err(e) => return Err(e),
            }
        }
        Ok(concatenated)
    }

    fn get_biomass_concentration(&self) -> Result<Array2<f64>, String> {
        let mut concatenated = Array2::<f64>::default((0, 0));
        let mut init = false;
        for postprocess in &self.dataset {
            match postprocess.get_biomass_concentration() {
                Ok(data) => {
                    if !init {
                        concatenated = data;
                        init = true;
                    } else if let Err(err) = concatenated.append(Axis(0), data.view()) {
                        return Err(err.to_string());
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

    fn get_number_particle(&self) -> &Array2<f64> {
        todo!()
    }

    fn get_properties(&self, key: &str, i_export: usize) -> Result<Array1<f64>, String> {
        todo!()
    }

    fn get_time_population_mean(&self, key: &str) -> Result<Array1<f64>, String> {
        todo!()
    }

    fn get_histogram_array(
        &self,
        n_bins: usize,
        i_export: usize,
        key: &str,
    ) -> Result<(Array1<f64>, Array1<f64>), String> {
        todo!()
    }

    fn get_histogram(
        &self,
        n_bins: usize,
        i_export: usize,
        key: &str,
    ) -> Result<(Vec<f64>, Vec<f64>), String> {
        todo!()
    }

    fn get_population_mean(&self, key: &str, i_export: usize) -> Result<f64, String> {
        todo!()
    }
}
