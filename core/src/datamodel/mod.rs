mod _impl;
mod main_file;
use std::collections::HashMap;
pub trait ResultGroup<T> {
    fn read_g(&self) -> hdf5::Result<T>;
}

#[derive(Debug)]
pub struct Dim(pub usize, pub usize);

use hdf5::Result;
pub use main_file::{MainFInal, MainInitial, MainRecords, MainResult, Misc};
use ndarray::{Array2, ArrayView2, ArrayView3, Shape};
pub use _impl::{read_model_mass,read_model_properties,read_avg_model_properties};

#[derive(Debug)]
pub struct Results(pub MainResult,pub Vec<String>,pub Array2<f64>);

impl Results
{
    pub fn new(fp:&str,root:&str,folder:&str)->Option<Self>
    {
        if let Ok(main) = {MainResult::read(fp)}
        {
            let list:Vec<String> = (0..main.misc.n_rank).map(|i| format!("{}/{}/{}_partial_{}.h5",root,folder,folder,i)).collect();
            let nt = main.records.time.len();
            let shape = (nt, main.records.dim.0);
            let mut total_particle_repetition: Array2<f64> = Array2::zeros(shape);
            for i_f in &list {
                match _impl::read_number_particle(i_f) {
                    Ok(result) => {
                        total_particle_repetition = total_particle_repetition + Array2::from_shape_vec(shape, result).unwrap();  
                    }
                    Err(e) => {
                        panic!("Error reading number of particles: {:?}", e);
                    }
                };
            }
      
        
            return Some(Results{0:main,1:list,2:total_particle_repetition});
        }
        return None;
        
    }


}

pub fn vec_to_array_view2<'a>(vec: &'a Vec<f64>, nr:usize,nc:usize) -> ArrayView2<'a, f64> {
    assert_eq!(vec.len(), nr * nc, "Vector size must match dimensions.");
    ArrayView2::from_shape((nr, nc), vec).expect("Failed to create ArrayView2")
}

pub fn vec_to_array_view3<'a>(vec: &'a Vec<f64>, dim:&'a Dim,nt:usize,) -> ArrayView3<'a, f64> {
    assert_eq!(vec.len(), nt*dim.0 * dim.1, "Vector size must match dimensions.");
    ArrayView3::from_shape((nt,dim.0.clone(), dim.1.clone()), vec).expect("Failed to create ArrayView2")
}