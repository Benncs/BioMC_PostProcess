use pyo3::prelude::*;
use std::ffi::CString;
use std::fs;

pub fn run_embed(script_path: &str) -> Result<(), String> {
    let r = Python::with_gil(|py| -> PyResult<()> {
        let module = PyModule::new(py, "embed_biomc_pp")?;

        let sys_modules = py.import("sys")?.getattr("modules")?;

        super::my_extension(&module);

        sys_modules.set_item("embed_biomc_pp", module)?;

        let script = fs::read_to_string(script_path)?;

        let script = CString::new(script)?;
        py.run(&script, None, None)?;

        Ok(())
    });

    match r {
        Ok(_) => Ok(()),
        Err(e) => {
            
            Err(format!("Error: {:?}", e))
        }
    }

}
