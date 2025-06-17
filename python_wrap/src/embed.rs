use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::ffi::CString;
use std::fs;
use super::biomc_pp as embed_biomc_pp;
use pyo3::ffi::c_str;
// pub fn run_embed(script_path: &str) -> Result<(), String> {
//     let r = Python::with_gil(|py| -> PyResult<()> {
//         let module = PyModule::new(py, "embed_biomc_pp")?;

//         let sys_modules = py.import("sys")?.getattr("modules")?;

//         super::my_extension(&module);

//         sys_modules.set_item("embed_biomc_pp", module)?;

//         let args ="pathtoresults";

//         let script = fs::read_to_string(script_path)?;

//         let script = CString::new(script)?;
//         py.run(&script, None, None)?;

//         Ok(())
//     });

//     match r {
//         Ok(_) => Ok(()),
//         Err(e) => {

//             Err(format!("Error: {:?}", e))
//         }
//     }

// }

pub fn run_embed(script_path: &str, name: &str, root: &str) -> Result<(), String> {
    pyo3::append_to_inittab!(embed_biomc_pp);
    let r = Python::with_gil(|py| -> PyResult<()> {
        let raw_script = fs::read_to_string(script_path)?;

        let complete_script = format!(
            r#"
__root_pp = "{}"
__name_pp = "{}"
{}
if __name__ == "__main__":
    __udf_entry_point(__name_pp,__root_pp)

    "#,
            root, name, raw_script
        );

        let script = CString::new(complete_script)?;

        Python::run(py, &script, None, None)?;

        Ok(())
    });

    match r {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Error: {:?}", e)),
    }
}
