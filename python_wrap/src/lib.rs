use bcore::api::{ModelEstimator};
use bcore::{PostProcess, PostProcessReader};
use numpy::PyArray2;
use numpy::{PyArray1, PyArray3};
use pyo3::exceptions::{PyRuntimeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyList;
use bcore::Weight;
/// A struct that wraps the `PostProcess` type for Python bindings.
///
/// The `PythonPostProcess` struct is designed to provide a Python interface for the
/// `PostProcess` struct, enabling the interaction with Rust's `PostProcess` type in Python.
///
/// # Fields
///
/// - `inner`: A reference to the inner `PostProcess` object, which holds the actual
///   post-processing logic that can be used in Python.
///
/// # Example
///
/// ```python
/// # Example usage in Python
/// post_process = PostProcess(name,root)
/// ```
#[derive(Debug)]
#[pyclass(name = "PostProcess")]
struct PythonPostProcess {
    inner: PostProcess,
}

/// An enum representing different phases .
/// # Example
/// ```python
/// phase = Phase.Liquid
/// assert phase == Phase.Liquid
/// ```
#[derive(Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum Phase {
    Liquid,
    Gas,
}

impl From<Phase> for bcore::Phase {
    fn from(val: Phase) -> Self {
        match val {
            Phase::Liquid => bcore::Phase::Liquid,
            Phase::Gas => bcore::Phase::Gas,
        }
    }
}

impl From<bcore::Phase> for Phase {
    fn from(phase: bcore::Phase) -> Self {
        match phase {
            bcore::Phase::Liquid => Phase::Liquid,
            bcore::Phase::Gas => Phase::Gas,
        }
    }
}


/// An enum representing different phases .
/// # Example
/// ```python
/// phase = Phase.Liquid
/// assert phase == Phase.Liquid
/// ```
#[derive(Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum Estimator {
    MonteCarlo,
    Weighted,
}

impl From<Estimator> for bcore::api::Estimator {
    fn from(val: Estimator) -> Self {
        match val {
            Estimator::MonteCarlo => bcore::api::Estimator::MonteCarlo,
            Estimator::Weighted => bcore::api::Estimator::Weighted,
        }
    }
}

impl From<bcore::api::Estimator> for Estimator {
    fn from(phase: bcore::api::Estimator) -> Self {
        match phase {
            bcore::api::Estimator::MonteCarlo => Estimator::MonteCarlo,
            bcore::api::Estimator::Weighted => Estimator::Weighted,
        }
    }
}

#[pymethods]
impl PythonPostProcess {
    /// Creates a new instance of `PythonPostProcess`.
    ///
    /// This function serves as a constructor for creating a`PostProcess` struct for use in Python. It takes a folder path
    /// and an optional root string as arguments to initialize the `PostProcess` object.
    ///
    /// # Arguments
    ///
    /// * `folder` (`str`): A path to the folder where the post-processing files or resources are located.
    /// * `root` (`Option<String>`): An optional root path used for additional processing logic, or `None` if not provided.
    ///
    /// # Returns
    ///
    /// * `PyResult<Self>`: On success, returns a `PythonPostProcess` instance wrapped in a `PyResult`.
    ///   On failure, raises a `PyValueError` with an error message.
    ///
    /// # Example
    ///
    /// ```python
    /// post_process = PostProcess("path/to/folder", "optional/root")
    /// ```
    ///
    /// # Errors
    ///
    /// If the `PostProcess::new` function fails (e.g., due to invalid paths or other internal errors),
    /// this function returns a `PyValueError` with the message `"Error creating object"`.
    #[new]
    #[pyo3(signature = (folder, root=None))]
    fn new(folder: &str, root: Option<String>) -> PyResult<Self> {
        if let Ok(pp) = PostProcess::new(folder, root) {
            return Ok(Self { inner: pp });
        }

        Err(PyValueError::new_err("Error creating object"))
    }

    fn get_property_names(&self) -> PyResult<Vec<String>> {
        Ok(self.inner.get_property_names())
    }

    /// Gets the time data
    ///
    /// This function provides access to the time data stored within the `PostProcess` object.
    /// It returns a slice of `f64` values representing the time series used in the post-processing.
    ///
    /// # Returns
    ///
    /// * `PyResult<&[f64]>`: A reference to a slice of `f64` values representing the time series.
    ///
    /// # Example
    ///
    /// ```python
    /// post_process = PostProcess("path/to/folder")
    /// time_data = post_process.time
    /// ```
    #[getter]
    fn time(&self) -> PyResult<&[f64]> {
        Ok(self.inner.time())
    }

    /// Gets the number of exports
    ///
    /// This function retrieves the number of export events.
    ///
    /// # Returns
    ///
    /// * `PyResult<usize>`: The number of exports, represented as a `usize` value.
    ///
    /// # Example
    ///
    /// ```python
    /// post_process = PostProcess("path/to/folder")
    /// num_exports = post_process.n_export()
    /// ```
    #[getter]
    fn n_export(&self) -> PyResult<usize> {
        Ok(self.inner.n_export())
    }

    #[getter]
    fn max_n_export_bio(&self) -> usize {
        self.inner.get_max_n_export_bio()
    }

    #[getter]
    fn weight(&self,py: Python<'_>) -> Py<PyArray1<f64>> {
        match self.inner.weight()
        {
            Weight::Single(sw)=>{PyArray1::from_vec(py, vec![*sw]).unbind()},
            Weight::Multiple(mw)=>{PyArray1::from_vec(py, mw.clone()).unbind()}
        }

        
    }

    fn get_spatial_average_concentration(
        &self,
        py: Python<'_>,
        species: usize,
        phase: Phase,
    ) -> Py<PyArray1<f64>> {
        let e = self
            .inner
            .get_spatial_average_concentration(species, phase.into());

        PyArray1::from_owned_array(py, e).unbind()
    }

    fn get_spatial_average_mtr(&self, py: Python<'_>, species: usize) -> Py<PyArray1<f64>> {
        match self.inner.get_spatial_average_mtr(species) {
            Ok(e) => PyArray1::from_owned_array(py, e).unbind(),
            Err(e) => panic!("{}", e),
        }
    }

    fn get_concentrations(&self, py: Python<'_>, phase: Phase) -> Py<PyArray3<f64>> {
        let e = self.inner.get_concentrations(phase.into());

        PyArray3::from_owned_array(py, e.to_owned()).unbind()
    }

    fn get_time_average_concentration(
        &self,
        py: Python<'_>,
        species: usize,
        position: usize,
        phase: Phase,
    ) -> Py<PyArray1<f64>> {
        if let Ok(e) =
            self.inner
                .get_time_average_concentration(species, position, phase.into())
        {
            return PyArray1::from_owned_array(py, e).unbind();
        }
        panic!("")
    }

    fn get_spatial_property(&self, name: &str) -> PyResult<PyObject> {
        // TODO
        Python::with_gil(|py| Ok(PyList::empty(py).to_object(py)))
    }

    fn get_biomass_concentration(&self, py: Python<'_>) -> Py<PyArray2<f64>> {
        match self.inner.get_biomass_concentration() {
            Ok(e) => PyArray2::from_owned_array(py, e).unbind(),
            Err(e) => panic!("{}", e),
        }
    }

    fn get_growth_in_number(&self, py: Python<'_>) -> Py<PyArray1<f64>> {
        let e = self.inner.get_growth_in_number();

        PyArray1::from_owned_array(py, e).unbind()
    }

    fn get_number_particle<'py>(&self, py: Python<'_>) -> Py<PyArray2<f64>> {
        let e = self.inner.get_number_particle().to_owned();

        PyArray2::from_owned_array(py, e).unbind()
    }

    fn get_rtd(&self, flow: f64, step: f64, is_str: Option<bool>) -> PyResult<PyObject> {
        // todo
        Python::with_gil(|py| Ok(py.None()))
    }

    fn get_properties(&self, py: Python<'_>, key: &str, i_export: usize) -> Py<PyArray1<f64>> {
        let e = self.inner.get_properties(key, i_export);

        PyArray1::from_owned_array(py, e.unwrap()).unbind() //TODO
    }

    fn get_population_mean(&self, key: &str, i_export: usize) -> PyResult<f64> {
        if let Ok(o) = self.inner.get_population_mean(key, i_export) {
            Ok(o)
        } else {
            Err(PyValueError::new_err("Error get_population_mean"))
        }
    }

    fn get_time_population_mean(&self, py: Python<'_>, key: &str) -> Py<PyArray1<f64>> {
        let e = self.inner.get_time_population_mean(key);

        PyArray1::from_owned_array(py, e.unwrap()).unbind() //TODO
    }

    pub fn get_histogram(
        &self,
        py: Python<'_>,
        n_bins: usize,
        i_export: usize,
        key: &str,
    ) -> (Py<PyArray1<f64>>, Py<PyArray1<f64>>) {
        let e = self.inner.get_histogram(n_bins, i_export, key);

        match e {
            Ok((nbins, counts)) => (
                PyArray1::from_owned_array(py, nbins.into()).unbind(),
                PyArray1::from_owned_array(py, counts.into()).unbind(),
            ),
            Err(e) => {
                panic!("histogram {}", e);
            }
        }
    }

    pub fn mu_direct(&self, py: Python<'_>) -> PyResult<Py<PyArray1<f64>>> {
        match self.inner.mu_direct() {
            Ok(e) => Ok(PyArray1::from_owned_array(py, e).unbind()),
            Err(e) => Err(PyErr::new::<PyRuntimeError, _>(format!("mu_direct error: {}", e))),
        }
    }

    pub fn estimate(&self, etype: Estimator, key: &str, i_export: usize) -> PyResult<f64> 
    {
        match self.inner.estimate(etype.into(),key,i_export) {
            Ok(e) => Ok(e),
            Err(e) => Err(PyErr::new::<PyRuntimeError, _>(format!("mu_direct error: {}", e))),
        }
    }
    
    fn get_spatial_average_biomass_concentration(&self,py: Python<'_>) -> PyResult<Py<PyArray1<f64>>>
    {
        match self.inner.get_spatial_average_biomass_concentration() {
            Ok(e) => Ok(PyArray1::from_owned_array(py, e).unbind()),
            Err(e) => Err(PyErr::new::<PyRuntimeError, _>(format!("get_spatial_average_biomass_concentration: {}", e))),
        }
    }

    pub fn estimate_time(&self, py: Python<'_>,etype: Estimator, key: &str) -> PyResult<Py<PyArray1<f64>>>
    {
        match self.inner.estimate_time(etype.into(),key) {
            Ok(e) => Ok(PyArray1::from_owned_array(py, e).unbind()),
            Err(e) => Err(PyErr::new::<PyRuntimeError, _>(format!("mu_direct error: {}", e))),
        }
    }
}

#[pymodule]
mod biomc_pp {
    #[pymodule_export]
    use super::Phase;
    #[pymodule_export]
    use super::Estimator;
    #[pymodule_export]
    use super::PythonPostProcess;
}
