use crate::api::{ModelEstimator, PostProcessReader};
use crate::datamodel::{
    get_n_export_real, read_avg_model_properties, read_model_mass, read_model_properties,
    vec_to_array_view2, vec_to_array_view3, Dim, tallies::Tallies, Weight,
};
use crate::datamodel::{f_get_probes, make_histogram, read_spatial_model_properties, Results};
use crate::process::{spatial_average_concentration, Histogram};
use crate::{api::Estimator, api::Phase, error::ApiError};
use ndarray::{s, Array1, Array2, ArrayView3, Axis};

/// The `PostProcess` struct handles post-processing of simulation results.
///
/// It contains the path to the results folder, the root directory, and the processed results.
///
/// The struct uses the `Results` struct internally, which is expected to contain time and other data.
#[derive(Debug)]
pub struct PostProcess {
    results: Results, // The results of the simulation, which will be accessed for time and other data.
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
    pub fn new(folder: &str, root: Option<String>) -> Result<Self, ApiError> {
        let _root = root.unwrap_or_else(|| "./results/".to_string());
        let result_path = format!("{}/{}/{}.h5", _root, folder, folder);
        let main = Results::new(&result_path, &_root, folder)?;
        Ok(Self { results: main })
    }
}

impl PostProcessReader for PostProcess {
    fn time(&self) -> &[f64] {
        self.results.main.time()
    }

    fn weight(&self) -> &Weight {
        &self.results.main.weight
    }

    fn get_property_names(&self) -> Vec<String> {
        self.results.property_name.clone()
    }

    fn get_spatial_average_mtr(&self, species: usize) -> Result<Array1<f64>, ApiError> {
        let r = &self.results.main.records;
        let nt = r.time.len();
        let dim = &r.dim;

        if let Some(mtr) = &r.mtr {
            let mtr = vec_to_array_view3(mtr, dim, nt);

            return match mtr.slice(s![.., .., species]).mean_axis(Axis(1)) {
                Some(avg) => Ok(avg),
                None => Err(ApiError::ShapeError),
            };
        }

        Err(ApiError::RecordsError("mtr".to_string()))

        // let mtr = vec_to_array_view3(self.results., &dim, nt);
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

    fn get_concentrations(&self, phase: Phase) -> ArrayView3<f64> {
        let records = &self.results.main.records;
        let nt = records.time.len();
        let dim = &records.dim;

        match phase {
            Phase::Gas => {
                if let (Some(c), Some(_v)) = (&records.concentration_gas, &records.volume_gas) {
                    return vec_to_array_view3(c, dim, nt);
                }

                panic!("Gas is not present");
            }
            Phase::Liquid => vec_to_array_view3(&records.concentration_liquid, dim, nt),
        }
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
            let cl = vec_to_array_view3(concentration, dim, nt);
            let vol = vec_to_array_view2(volume, nt, dim.0);
            let res = spatial_average_concentration(&cl.slice(s![.., .., species]), &vol);

            res
        }

        let records = &self.results.main.records;
        let nt = records.time.len();
        let dim = &records.dim;

        match phase {
            Phase::Gas => {
                if let (Some(c), Some(v)) = (&records.concentration_gas, &records.volume_gas) {
                    return process_phase(c, v, nt, dim, species);
                }

                panic!("Gas is not present");
            }
            Phase::Liquid => process_phase(
                &records.concentration_liquid,
                &records.volume_liquid,
                nt,
                dim,
                species,
            ),
        }
    }

    fn get_spatial_average_biomass_concentration(&self) -> Result<Array1<f64>, ApiError> {
        let num_dimensions = self.results.main.records.dim.0;
        let nt = self.results.main.records.time.len();
        let volume =
            vec_to_array_view2(&self.results.main.records.volume_liquid, nt, num_dimensions);
        let vtot = volume.sum_axis(Axis(1));

        let mut biomass_matrix = Array1::zeros(nt);

        for i in 0..nt {
            let m = self.get_properties("mass", i)?;
            biomass_matrix[i] = m.sum() / vtot[i];
        }

        Ok(self.results.main.initial.initial_weight * biomass_matrix)
    }

    fn get_time_average_concentration(
        &self,
        species: usize,
        position: usize, 
        phase: Phase,
    ) -> Result<Array1<f64>, ApiError> {
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

                Err(ApiError::RecordsError("Gas".to_string()))
            }
        }
    }

    //Actually compute spatial average property cannot be use to follow distribution  
    fn get_spatial_average_property(&self, key:&str) ->  Result<Array2<f64>, ApiError>
    {
        let nt: usize = self.results.main.records.time.len(); // Number of time steps
        let num_dimensions = self.results.main.records.dim.0; // Dimensionality

        // Initialize the biomass matrix
        let mut biomass_matrix = Array2::zeros((nt, num_dimensions));

        if !self.results.property_name.iter().any(|x| x == key) {
            return Err(ApiError::KeyError(key.to_string()));
        }

        read_spatial_model_properties(key,self.results.get_files(), &mut biomass_matrix, nt) ?;
     


        // Calculate biomass concentration
        biomass_matrix /= self.get_number_particle();

        Ok(biomass_matrix)
    }

    fn get_probes(&self) -> Result<Array1<f64>, ApiError>
    {
        f_get_probes(&self.results.files)
    }

    /// Calculates the biomass concentration over time for the simulation.
    ///
    /// # Returns
    /// * `Result<Array2<f64>, String>` - A 2D array with biomass concentrations or an error message.
    fn get_biomass_concentration(&self) -> Result<Array2<f64>, ApiError> {
        let nt: usize = self.results.main.records.time.len(); // Number of time steps
        let num_dimensions = self.results.main.records.dim.0; // Dimensionality

        // Initialize the biomass matrix
        let mut biomass_matrix = Array2::zeros((nt, num_dimensions));

        if !self.results.property_name.iter().any(|x| x == "mass") {
            return Err(ApiError::KeyError("mass".to_string()));
        }

        // Attempt to read model mass
        read_model_mass(self.results.get_files(), &mut biomass_matrix, nt)?;

        // Convert volume to an array view
        let volume =
            vec_to_array_view2(&self.results.main.records.volume_liquid, nt, num_dimensions);

        // Calculate biomass concentration
        biomass_matrix = self.results.main.initial.initial_weight * (biomass_matrix / volume);

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
    fn get_properties(&self, key: &str, i_export: usize) -> Result<Array1<f64>, ApiError> {
        if i_export >= self.results.main.records.time.len() {
            return Err(ApiError::OutOfRange(
                i_export,
                self.results.main.records.time.len(),
            ));
        }

        if !self.results.property_name.iter().any(|x| x == key) {
            return Err(ApiError::KeyError(key.to_string()));
        }

        match read_model_properties(key, self.results.get_files(), i_export) {
            Ok(res) => Ok(res),
            // Err(e) => Err(format!("Failed to read model properties: {:?}", e)),
            Err(e) => Err(ApiError::Io(e)),
        }
    }

    /// Calculates the population mean over time for a given property key.
    ///
    /// # Arguments
    /// * `key` - The property key for which to calculate the mean.
    ///
    /// # Returns
    /// * `Result<Array1<f64>, String>` - A 1D array of mean values over time or an error message.
    fn get_time_population_mean(&self, key: &str) -> Result<Array1<f64>, ApiError> {
        if !self.results.property_name.iter().any(|x| x == key) {
            return Err(ApiError::KeyError(key.to_string()));
        }

        match read_avg_model_properties(key, self.results.get_files(), self.n_export()) {
            Ok(res) => Ok(res),
            Err(e) => Err(ApiError::Io(e)),
        }
    }

    fn get_histogram_array(
        &self,
        n_bins: usize,
        i_export: usize,
        key: &str,
    ) -> Result<(Array1<f64>, Array1<f64>), ApiError> {
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
    ) -> Result<(Vec<f64>, Vec<f64>), ApiError> {
        if i_export > self.n_export() {
            return Err(ApiError::OutOfRange(i_export, self.n_export()));
        }
        if !self.results.property_name.iter().any(|x| x == key) {
            return Err(ApiError::KeyError(key.to_string()));
        }
        // let np = n_bins;//*self.results.total_particle_repetition.sum_axis(Axis(1)).last().unwrap() as usize;
        let mut hist = Histogram::new(n_bins);

        if let Err(e) = make_histogram(self.results.get_files(), i_export, key, &mut hist) {
            return Err(ApiError::Io(e));
        }

        let b = hist.get_bins().to_vec();
        let c = hist.get_counts().to_vec();
        Ok((b, c))
    }

    fn get_population_mean(&self, key: &str, i_export: usize) -> Result<f64, ApiError> {
        // Check if the index is out of range
        if i_export >= self.results.main.records.time.len() {
            return Err(ApiError::OutOfRange(
                i_export,
                self.results.main.records.time.len(),
            ));
        }
        if !self.results.property_name.iter().any(|x| x == key) {
            return Err(ApiError::KeyError(key.to_string()));
        }

        match read_model_properties(key, self.results.get_files(), i_export) {
            Ok(res) => res
                .mean()
                .ok_or(ApiError::Default("get_population_mean".to_string())),
            Err(e) => Err(ApiError::Io(e)),
        }
    }
    fn tallies(&self) -> Option<&Tallies> {
        self.results.main.records.tallies.as_ref()
    }
}

impl ModelEstimator for PostProcess {
    fn mu_direct(&self) -> Result<Array1<f64>, ApiError> {
        match self.weight() {
            Weight::Single(_) => {
                let nt = self.results.main.records.time.len();
                let time = self.time();

                let mu_functor = |i: usize, j: usize, im: usize| -> Result<f64, ApiError> {
                    let mass_i = self.get_properties("mass", i)?;
                    let mass_mi = self.get_properties("mass", j)?;
                    let mass_tot_i = self.get_properties("mass", im)?.sum();
                    let dm: f64 = mass_i.sum() - mass_mi.sum();
                    let dt = time[i] - time[j];
                    Ok(dm / dt / mass_tot_i)
                };

                let mut mu = Array1::zeros(nt);

                mu[0] = mu_functor(1, 0, 0)?; //Forward

                for i in 1..nt - 1 {
                    mu[i] = mu_functor(i + 1, i - 1, i)?; //Center
                }
                mu[nt - 1] = mu_functor(nt - 1, nt - 2, nt - 1)?; //Backward

                Ok(mu)
            }
            Weight::Multiple(_) => todo!("Mu with different weight"),
        }
    }

    fn estimate(&self, etype: Estimator, key: &str, i_export: usize) -> Result<f64, ApiError> {
        crate::process::estimate(etype, self.weight(), &self.get_properties(key, i_export)?)
    }

    fn estimate_time(&self, etype: Estimator, key: &str) -> Result<Array1<f64>, ApiError> {
        let nt = self.results.main.records.time.len();
        let mut estimator = Array1::<f64>::zeros(nt);
        for i in 0..nt {
            estimator[i] = self.estimate(etype, key, i)?;
        }
        Ok(estimator)
    }
}
