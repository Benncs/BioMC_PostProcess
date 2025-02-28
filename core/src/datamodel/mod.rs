mod _impl;
mod main_file;
use crate::error::ApiError;
pub use _impl::{
    get_n_export_real, make_histogram, read_avg_model_properties, read_model_mass,
    read_model_properties,
};
pub use main_file::MainResult;
use ndarray::{Array2, ArrayView2, ArrayView3};
use std::path::PathBuf;

trait ResultGroup<T> {
    fn read_g(&self) -> hdf5::Result<T>;
}

#[derive(Debug)]
pub struct Dim(pub usize, pub usize);

#[derive(Debug)]
pub enum Weight {
    Single(f64),        // Represents a single f64 value
    Multiple(Vec<f64>), // Represents a vector of f64 values
}

#[derive(Debug)]
pub struct Results {
    pub main: MainResult,
    pub files: Vec<String>,
    pub total_particle_repetition: Array2<f64>,
    pub property_name: Vec<String>,
}

impl Results {
    pub fn new(fp: &str, root: &str, folder: &str) -> Result<Self, ApiError> {
        match MainResult::read(fp) {
            Ok(main) => {
                let files: Vec<String> = (0..main.misc.n_rank)
                    .map(|i| format!("{}/{}/{}_partial_{}.h5", root, folder, folder, i))
                    .collect();

                let nt = main.records.time.len();
                let shape = (nt, main.records.dim.0);
                let mut total_particle_repetition: Array2<f64> = Array2::zeros(shape);
                for i_f in &files {
                    let n_p = _impl::read_number_particle(i_f)?;
                    total_particle_repetition =
                        total_particle_repetition + Array2::from_shape_vec(shape, n_p).unwrap();
                }
                let property_name = Self::get_property_name(&files);
                Ok(Results {
                    main,
                    files,
                    total_particle_repetition,
                    property_name,
                })
            }
            Err(hdf5_error) => Err(ApiError::Io(hdf5_error)),
        }
    }

    fn get_property_name(files: &[String]) -> Vec<String> {
        if let Ok(file) = hdf5::File::open(files[0].clone()) {
            if let Ok(group) = file.group("biological_model/0") {
                let dataset_names: Vec<String> = group
                    .datasets()
                    .unwrap()
                    .into_iter()
                    .map(|d| {
                        PathBuf::from(d.name())
                            .file_name()
                            .and_then(|name| name.to_str())
                            .unwrap_or("")
                            .to_string()
                    }) // Extract the names of each dataset
                    .collect();
                return dataset_names;

                // .attr_names().unwrap()
                // .into_iter().map(|g| g)  // Extract the names of each group
                // .collect();
            }
        }
        todo!()
    }

    pub fn get_files(&self) -> &[String] {
        &self.files
    }
}

pub fn vec_to_array_view2(vec: &[f64], nr: usize, nc: usize) -> ArrayView2<'_, f64> {
    assert_eq!(vec.len(), nr * nc, "Vector size must match dimensions.");
    ArrayView2::from_shape((nr, nc), vec).expect("Failed to create ArrayView2")
}

pub fn vec_to_array_view3<'a>(vec: &'a [f64], dim: &'a Dim, nt: usize) -> ArrayView3<'a,f64> {
    assert_eq!(
        vec.len(),
        nt * dim.0 * dim.1,
        "Vector size must match dimensions."
    );
    ArrayView3::from_shape((nt, dim.0, dim.1), vec).expect("Failed to create ArrayView2")
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::{array, Array3};

    #[test]
    fn test_vec_to_array_view2_valid() {
        let vec = vec![1.0, 2.0, 3.0, 4.0]; // 2x2 matrix
        let nr = 2;
        let nc = 2;

        let view = vec_to_array_view2(&vec, nr, nc);

        assert_eq!(view.shape(), &[2, 2]);
        assert_eq!(view, array![[1.0, 2.0], [3.0, 4.0]]);
    }

    #[test]
    #[should_panic(expected = "Vector size must match dimensions.")]
    fn test_vec_to_array_view2_invalid_size() {
        let vec = vec![1.0, 2.0, 3.0]; // Incorrect size, should be 2x2
        let nr = 2;
        let nc = 2;

        vec_to_array_view2(&vec, nr, nc);
    }

    #[test]
    #[should_panic]
    fn test_vec_to_array_view2_empty_vector() {
        let vec: Vec<f64> = vec![];
        let nr = 2;
        let nc = 2;

        vec_to_array_view2(&vec, nr, nc);
    }
    #[test]
    fn test_vec_to_array_view3() {
        let vec = vec![1.0; 6]; // 2 * 3 dimensions
        let dim = &Dim { 0: 2, 1: 3 };
        let nt = 1;
        let vec_copy = vec.clone();
        let array_view = vec_to_array_view3(&vec, dim, nt);

        let expected_array = Array3::from_shape_vec((nt, dim.0, dim.1), vec_copy)
            .expect("Failed to create expected Array3");
        assert_eq!(array_view, expected_array.view());
    }

    #[test]
    #[should_panic]
    fn test_vec_to_array_view3_size_mismatch() {
        let vec = vec![1.0, 2.0, 3.0];
        let dim = &Dim { 0: 2, 1: 3 };
        let nt = 1;

        let _ = vec_to_array_view3(&vec, dim, nt);
    }

    #[test]
    #[should_panic]
    fn test_vec_to_array_view3_invalid_size() {
        let vec = vec![1.0, 2.0, 3.0]; // Incorrect size, should be 1x2x3
        let dim = Dim(1, 2);
        let nt = 1;

        vec_to_array_view3(&vec, &dim, nt);
    }

    #[test]
    #[should_panic]
    fn test_vec_to_array_view3_empty_vector() {
        let vec: Vec<f64> = vec![];
        let dim = Dim(1, 2);
        let nt = 1;

        vec_to_array_view3(&vec, &dim, nt);
    }
}
