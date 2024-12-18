use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyList, PyString};
use numpy::{IntoPyArray, PyArray1, PyArray2, PyArrayLike2};
use bcore::PostProcess;

#[derive(Debug)]
#[pyclass(name = "PostProcess")]
struct PythonPostProcess {
    inner:PostProcess
}

#[derive(Clone,PartialEq)]
#[pyclass(eq, eq_int)]
pub enum Phase
{
    Liquid,
    Gas
}

fn convert_phase(p:Phase)->bcore::Phase
{
    return match p 
    {
        Phase::Liquid=>
        {
            bcore::Phase::Liquid
        }
        Phase::Gas=>
        {
            bcore::Phase::Gas
        }
    }
}


#[pymethods]
impl PythonPostProcess {
    #[new]
    #[pyo3(signature = (folder, root=None))]
    fn new(folder: &str, root: Option<String>) -> PyResult<Self> {

        if let Ok(pp) = PostProcess::new(folder,root)
        {
            return Ok(Self{inner:pp});
        }

        Err(PyValueError::new_err("Error creating object"))
    }

    #[getter]
    fn time(&self) -> PyResult<&[f64]> {
        Ok(self.inner.time())
    }

    fn get_spatial_average_concentration(&self,py: Python<'_>, species: usize, phase: Phase) -> Py<PyArray1<f64>> {
        let e = self.inner.get_spatial_average_concentration(species,convert_phase(phase));
        
        return PyArray1::from_owned_array(py, e).unbind();
        
    
    }

    fn get_time_average_concentration(&self, py: Python<'_>,species: usize, position: usize, phase: Phase) -> Py<PyArray1<f64>> {
        if let Ok(e) = self.inner.get_time_average_concentration(species,position,convert_phase(phase))
        {
            return PyArray1::from_owned_array(py, e).unbind();
        }
        panic!("")
    }

    fn get_property_list(&self) -> PyResult<Vec<String>> {
        // Placeholder signature
        Ok(vec![])
    }

    fn get_spatial_property(&self, name: &str) -> PyResult<PyObject> {
        // Placeholder signature
        Python::with_gil(|py| Ok(PyList::empty(py).to_object(py)))
    }

    fn get_biomass_concentration(&self,py: Python<'_>) -> Py<PyArray2<f64>> {

        let e = self.inner.get_biomass_concentration();
        
        return PyArray2::from_owned_array(py, e).unbind();

    }

    fn get_growth_in_number(&self,py: Python<'_>) -> Py<PyArray1<f64>> {

        let e = self.inner.get_growth_in_number();
        
        return PyArray1::from_owned_array(py, e).unbind();
    }

    fn get_number_particle<'py>(&self,py: Python<'_>) -> Py<PyArray2<f64>>{

        let e = self.inner.get_number_particle().to_owned();
        
        return PyArray2::from_owned_array(py, e).unbind();

    }

    fn get_rtd(&self, flow: f64, step: f64, is_str: Option<bool>) -> PyResult<PyObject> {
        // Placeholder signature
        Python::with_gil(|py| Ok(py.None()))
    }
}



#[pymodule]
mod biomc_pp
{
    #[pymodule_export]
    use super::PythonPostProcess;
    #[pymodule_export]
    use super::Phase;
   
}
