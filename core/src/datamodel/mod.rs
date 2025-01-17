mod _impl;
mod main_file;

pub use _impl::{
    get_n_export_real, read_avg_model_properties, read_model_mass, read_model_properties,make_histogram
};
pub use main_file::MainResult;

pub(crate) use ndarray::{Array2, ArrayView2, ArrayView3};


pub trait ResultGroup<T> {
    fn read_g(&self) -> hdf5::Result<T>;
}

#[derive(Debug)]
pub struct Dim(pub usize, pub usize);


#[derive(Debug)]
pub struct Results{
    pub main:MainResult, pub files:Vec<String>, pub total_particle_repetition:Array2<f64>}

impl Results {
    pub fn new(fp: &str, root: &str, folder: &str) -> Result<Self, String> {
        match MainResult::read(fp) {
            Ok(main) => {
                let files: Vec<String> = (0..main.misc.n_rank)
                    .map(|i| format!("{}/{}/{}_partial_{}.h5", root, folder, folder, i))
                    .collect();
                let nt = main.records.time.len();
                let shape = (nt, main.records.dim.0);
                let mut total_particle_repetition: Array2<f64> = Array2::zeros(shape);
                for i_f in &files {
                    match _impl::read_number_particle(i_f) {
                        Ok(result) => {
                            total_particle_repetition = total_particle_repetition
                                + Array2::from_shape_vec(shape, result).unwrap();
                        }
                        Err(e) => {
                            panic!("Error reading number of particles: {:?}", e);
                        }
                    };
                }
                Ok(Results {
                    main,
                    files,
                    total_particle_repetition,
                })
            }
            Err(hdf5_error) => Err(hdf5_error.to_string()),
        }

    }
}

pub fn vec_to_array_view2<'a>(vec: &'a Vec<f64>, nr: usize, nc: usize) -> ArrayView2<'a, f64> {
    assert_eq!(vec.len(), nr * nc, "Vector size must match dimensions.");
    ArrayView2::from_shape((nr, nc), vec).expect("Failed to create ArrayView2")
}

pub fn vec_to_array_view3<'a>(vec: &'a Vec<f64>, dim: &'a Dim, nt: usize) -> ArrayView3<'a, f64> {
    assert_eq!(
        vec.len(),
        nt * dim.0 * dim.1,
        "Vector size must match dimensions."
    );
    ArrayView3::from_shape((nt, dim.0, dim.1), vec)
        .expect("Failed to create ArrayView2")
}
