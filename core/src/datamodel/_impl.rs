use hdf5::Group;
use ndarray::{s, Array2, ArrayView1, ArrayView2};
use std::collections::HashMap;
use super::main_file::{Misc,MainFInal,MainRecords,MainInitial};
use super::{ResultGroup,Dim};

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

// macro_rules! read_scalar_or_default {
//     ($group:expr, $name:expr, $type:ty) => {
//         $group
//             .dataset($name)?.read_scalar::<$type>()?
//     };
// }

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

pub fn read_number_particle(filename:&str)->hdf5::Result<Vec<f64>>
{
    let file = hdf5::File::open_as(filename, hdf5::file::OpenMode::Read)?;
    let rec = file.group("/records")?;
    let v = read_vec!(rec,"number_particle",f64);
    Ok(v)
}


pub fn read_model_mass(files: &[String], cx: &mut Array2<f64>, n_export: usize) -> hdf5::Result<()> {
    for (i_file, filename) in files.iter().enumerate() {
        // Open the HDF5 file in read mode
        let file = hdf5::File::open_as(filename, hdf5::file::OpenMode::Read)?;
        
        // Access the "biological_model" group
        let group = file.group("biological_model")?;
        
        for i_e in 0..n_export {
            // Read the data for the current export index
            let tmp: Vec<f64> = match group.dataset(&format!("{}/spatial/mass", i_e)) {
                Ok(dataset) => dataset.read_raw::<f64>()?, // Read the data directly as Vec<f64>
                Err(_) => continue, // Skip if the dataset doesn't exist
            };

            let tmp_array = ArrayView1::from_shape(tmp.len(), &tmp)
                .map_err(|_| hdf5::Error::Internal("Shape mismatch while creating ArrayView1".to_string()))?;
            
            let slice_shape = cx.slice(s![i_e, ..]).len();
            if tmp_array.len() == slice_shape {
                cx.slice_mut(s![i_e, ..]).zip_mut_with(&tmp_array, |a, b| *a += b);
            } else {
                eprintln!("Shape mismatch: cx[{}, ..].shape = {}, tmp.shape = {}", i_e, slice_shape, tmp_array.len());
            }
        }
    }
    Ok(())
}



impl ResultGroup<MainInitial> for Group {
    fn read_g(&self) -> hdf5::Result<MainInitial> {
        let delta_time = read_scalar!(self, "delta_time", f64);
        let final_time = read_scalar!(self, "final_time", f64);
        let initial_biomass_concentration = read_scalar!(self, "initial_biomass_concentration", f64);
        let initial_weight = read_scalar!(self, "initial_weight", f64);
        let n_map = read_scalar!(self, "n_map", usize);
        let number_compartment = read_scalar!(self, "number_compartment", usize);
        let number_particles = read_scalar!(self, "number_particles", u64);
        let t_per_flow_map = read_scalar!(self, "t_per_flow_map", f64);
        // println!("{:?}",read_vec!(self,"particle_distribution",u64));

        Ok(MainInitial{delta_time,final_time,initial_biomass_concentration,initial_weight,n_map,number_compartment,number_particles,t_per_flow_map})
    }
}

impl ResultGroup<MainRecords> for Group {
    fn read_g(&self) -> hdf5::Result<MainRecords> {
        let concentration_liquid =  read_vec!(self,"concentration_liquid",f64);
        let volume_liquid =  read_vec!(self,"volume_liquid",f64);
        let (concentration_gas, volume_gas) = match (self.dataset("concentration_gas"), self.dataset("volume_gas")) {
            (Ok(cg), Ok(vg)) => (
                Some(cg.read_raw::<f64>()?), 
                Some(vg.read_raw::<f64>()?)
            ),
            _ => (None, None),
        };
        let shape = self.dataset("concentration_liquid")?.shape();
        println!("{:?}",shape);
        let dim = Dim(shape[1],shape[2]);
        let time = read_vec!(self,"time",f64);
        Ok(MainRecords{concentration_liquid,volume_liquid,concentration_gas,volume_gas,dim,time})
    }
}

impl ResultGroup<MainFInal> for Group {
    fn read_g(&self) -> hdf5::Result<MainFInal> {
        let ds_events = self.group("events")?;
        let mut events: HashMap<String, u64> = HashMap::new();
        ds_events.iter_visit_default(&mut events, |group, name, _link_info, fields| {
            if let Ok(dataset) = group.dataset(name) {
                if let Ok(value) = dataset.read_scalar::<u64>() {
                    fields.insert(name.to_string(), value);
                }
            }
            true
        })
        .unwrap();
        let number_particles = read_scalar!(self, "number_particles", u64);

        Ok(MainFInal{events,number_particles})
    }
}

