use crate::process::Histogram;

use super::main_file::{MainFInal, MainInitial, MainRecords, Misc};
use super::{Dim, ResultGroup};
use hdf5::Group;
use ndarray::{s, Array1, Array2, ArrayView1};
use std::collections::HashMap;

macro_rules! read_scalar {
    // Match the types f64, usize, or u64 and provide the correct default behavior
    ($group:expr, $name:expr, f64) => {
        $group.dataset($name)?.read_scalar::<f64>()?
    };
    ($group:expr, $name:expr, usize) => {
        $group.dataset($name)?.read_scalar::<usize>()?
    };
    ($group:expr, $name:expr, u64) => {
        $group.dataset($name)?.read_scalar::<u64>()?
    };
}

macro_rules! read_vec {
    // Match the types f64, usize, or u64 and provide the correct default behavior
    ($group:expr, $name:expr, f64) => {
        $group.dataset($name)?.read_raw::<f64>()?
    };
    ($group:expr, $name:expr, usize) => {
        $group.dataset($name)?.read_raw::<usize>()?
    };
    ($group:expr, $name:expr, u64) => {
        $group.dataset($name)?.read_raw::<u64>()?
    };
}

impl ResultGroup<Misc> for hdf5::Group {
    fn read_g(&self) -> hdf5::Result<Misc> {
        let mut fields: HashMap<String, u64> = HashMap::new();
        self.iter_visit_default(&mut fields, |group, name, _link_info, fields| {
            if let Ok(dataset) = group.dataset(name) {
                if let Ok(value) = dataset.read_scalar::<u64>() {
                    fields.insert(name.to_string(), value);
                }
            }
            // Continue the iteration
            true
        })
        .unwrap();

        Ok(Misc {
            n_node_thread: *fields.get("n_node_thread").unwrap_or(&0),
            n_rank: *fields.get("n_rank").unwrap_or(&0),
        })
    }
}

pub fn read_number_particle(filename: &str) -> hdf5::Result<Vec<f64>> {
    let file = hdf5::File::open_as(filename, hdf5::file::OpenMode::Read)?;
    let rec = file.group("/records")?;
    let v = read_vec!(rec, "number_particle", f64);
    Ok(v)
}

fn read_spatial_model_properties(
    key: &str,
    files: &[String],
    cx: &mut Array2<f64>,
    n_export: usize,
) -> hdf5::Result<()> {
    for filename in files.iter() {
        // Open the HDF5 file in read mode
        let file = hdf5::File::open_as(filename, hdf5::file::OpenMode::Read)?;

        // Access the "biological_model" group
        let group = file.group("biological_model")?;
        //TODO: use group.len() instead of n_export 
        for i_e in 0..n_export {
            // Read the data for the current export index
            let tmp: Vec<f64> = match group.dataset(&format!("{}/spatial/{}", i_e, key)) {
                Ok(dataset) => dataset.read_raw::<f64>()?, // Read the data directly as Vec<f64>
                Err(_) => continue,                        // Skip if the dataset doesn't exist
            };

            let tmp_array = ArrayView1::from_shape(tmp.len(), &tmp).map_err(|_| {
                hdf5::Error::Internal("Shape mismatch while creating ArrayView1".to_string())
            })?;

            let slice_shape = cx.slice(s![i_e, ..]).len();
            if tmp_array.len() == slice_shape {
                cx.slice_mut(s![i_e, ..])
                    .zip_mut_with(&tmp_array, |a, b| *a += b);
                } else {
                    eprintln!(
                        "Shape mismatch: cx[{}, ..].shape = {}, tmp.shape = {}",
                        i_e,
                        slice_shape,
                        tmp_array.len()
                    );
            }
        }
    }
    Ok(())
}

pub fn read_model_properties(
    key: &str,
    files: &[String],
    i_export: usize,
) -> hdf5::Result<Array1<f64>> {
    let mut total_size = 0;
    for filename in files.iter() {
        // Open the HDF5 file in read mode
        let file = hdf5::File::open_as(filename, hdf5::file::OpenMode::Read)?;

        // Access the "biological_model" group
        let group = file.group("biological_model")?;
        // Not all nodes have th=e same number of export so it happens that i_export is >
        // number_export for the current file
        if (group.len() as usize)>=i_export{
            let dataset = group.dataset(&format!("{}/{}", i_export, key))?;
            total_size += dataset.size();

        }
    }

    let mut result = Array1::zeros(total_size);
    let mut offset = 0;
    for filename in files.iter() {
        // Open the HDF5 file in read mode
        let file = hdf5::File::open_as(filename, hdf5::file::OpenMode::Read)?;

        // Access the "biological_model" group
        let group = file.group("biological_model")?;
        if (group.len() as usize)>=i_export{
            let dataset = group.dataset(&format!("{}/{}", i_export, key))?;

            // Read the dataset into a temporary array
            let temp_array: Vec<f64> = dataset.read_raw::<f64>()?;
            let tmp_array = ArrayView1::from_shape(temp_array.len(), &temp_array).map_err(|_| {
                hdf5::Error::Internal("Shape mismatch while creating ArrayView1".to_string())
            })?;

            // Copy the data into the result array
            result
                .slice_mut(s![offset..offset + temp_array.len()])
                .assign(&tmp_array);

            offset += temp_array.len();
        }
    }

    Ok(result)
}

pub fn get_n_export_real(files:&[String])->hdf5::Result<usize>
{
    if files.is_empty()
    {
        panic!("FIXME: not enough files")
    }
    let file = hdf5::File::open_as(&files[0], hdf5::file::OpenMode::Read)?;
    let group = file.group("biological_model")?;
    let group_size = group.len() as usize; //We export n_export times properties but if there is no
                                           //particle we do not export. group_size <= n_export and for
    Ok(group_size) 
}


pub fn make_histogram(files:&[String],i_export:usize,key:&str,hist:&mut Histogram)->hdf5::Result<()>
{
    for filename in files.iter() {
        // Open the HDF5 file in read mode
        let file = hdf5::File::open_as(filename, hdf5::file::OpenMode::Read)?;
        let group = file.group("biological_model")?;
        if (group.len() as usize)>=i_export{
            let dataset = group.dataset(&format!("{}/{}", i_export, key))?;
            let temp_array: Vec<f64> = dataset.read_raw::<f64>()?;
            hist.add(temp_array);
        }
    }

    Ok(())
}

pub fn read_avg_model_properties(
    key: &str,
    files: &[String],
    n_export: usize,
) -> hdf5::Result<Array1<f64>> {
    let mut result = Array1::zeros(n_export);
    let mut tot_particle: Array1<f64> = Array1::zeros(n_export);

    for filename in files {
        let file = hdf5::File::open_as(filename, hdf5::file::OpenMode::Read)?;
        let group = file.group("biological_model")?;
        let group_size = group.len() as usize; //We export n_export times properties but if there is no
                                               //particle we do not export. group_size <= n_export and for
                                               //all i > group_size , value is set to 0   
        for i_e in 0..group_size{
            let dataset = group.dataset(&format!("{}/{}", i_e, key))?;
            let temp_array: Vec<f64> = dataset.read_raw::<f64>()?;
            result[i_e] += temp_array.iter().sum::<f64>();
            tot_particle[i_e] += temp_array.len() as f64;
        }
    }

    Ok(result / tot_particle)
}

pub fn read_model_mass(
    files: &[String],
    cx: &mut Array2<f64>,
    n_export: usize,
) -> hdf5::Result<()> {
    read_spatial_model_properties("mass", files, cx, n_export)
}

impl ResultGroup<MainInitial> for Group {
    fn read_g(&self) -> hdf5::Result<MainInitial> {
        let delta_time = read_scalar!(self, "delta_time", f64);
        let final_time = read_scalar!(self, "final_time", f64);
        let initial_biomass_concentration =
            read_scalar!(self, "initial_biomass_concentration", f64);
        let initial_weight = read_scalar!(self, "initial_weight", f64);
        let n_map = read_scalar!(self, "n_map", usize);
        let number_compartment = read_scalar!(self, "number_compartment", usize);
        let number_particles = read_scalar!(self, "number_particles", u64);
        let t_per_flow_map = read_scalar!(self, "t_per_flow_map", f64);
        // println!("{:?}",read_vec!(self,"particle_distribution",u64));

        Ok(MainInitial {
            delta_time,
            final_time,
            initial_biomass_concentration,
            initial_weight,
            n_map,
            number_compartment,
            number_particles,
            t_per_flow_map,
        })
    }
}

impl ResultGroup<MainRecords> for Group {
    fn read_g(&self) -> hdf5::Result<MainRecords> {
        let concentration_liquid = read_vec!(self, "concentration_liquid", f64);
        let volume_liquid = read_vec!(self, "volume_liquid", f64);
        let (concentration_gas, volume_gas) = match (
            self.dataset("concentration_gas"),
            self.dataset("volume_gas"),
        ) {
            (Ok(cg), Ok(vg)) => (Some(cg.read_raw::<f64>()?), Some(vg.read_raw::<f64>()?)),
            _ => (None, None),
        };

        let mtr = match self.dataset("mtr") 
        {
            Ok(_mtr) => Some(_mtr.read_raw::<f64>()?),
            _ => None,
        };

        let shape = self.dataset("concentration_liquid")?.shape();
        let dim = Dim(shape[1], shape[2]);
        let time = read_vec!(self, "time", f64);
        Ok(MainRecords {
            concentration_liquid,
            volume_liquid,
            concentration_gas,
            volume_gas,
            mtr,
            dim,
            time,
        })
    }
}

impl ResultGroup<MainFInal> for Group {
    fn read_g(&self) -> hdf5::Result<MainFInal> {
        let ds_events = self.group("events")?;
        let mut events: HashMap<String, u64> = HashMap::new();
        ds_events
            .iter_visit_default(&mut events, |group, name, _link_info, fields| {
                if let Ok(dataset) = group.dataset(name) {
                    if let Ok(value) = dataset.read_scalar::<u64>() {
                        fields.insert(name.to_string(), value);
                    }
                }
                true
            })
        .unwrap();
        let number_particles = read_scalar!(self, "number_particles", u64);

        Ok(MainFInal {
            events,
            number_particles,
        })
    }
}
