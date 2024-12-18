use ndarray::{Array1,Array2, Axis,ArrayView3,ArrayView2};

pub fn spatial_average_concentration(
    concentration_record: &ArrayView2<f64>,
    full_volume: &ArrayView2<f64>,
) -> Array1<f64> {

    let weighted_concentration = concentration_record * full_volume;
    let numerator = weighted_concentration.sum_axis(Axis(1));
    let denominator = full_volume.sum_axis(Axis(1));
    numerator / denominator
}

