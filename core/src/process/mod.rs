use ndarray::{Array1, Array2, ArrayView1, ArrayView2, Axis};

pub fn spatial_average_concentration(
    concentration_record: &ArrayView2<f64>,
    full_volume: &ArrayView2<f64>,
) -> Array1<f64> {
    let weighted_concentration = concentration_record * full_volume;
    let numerator = weighted_concentration.sum_axis(Axis(1));
    let denominator = full_volume.sum_axis(Axis(1));
    numerator / denominator
}

pub fn normalize_concentration(
    concentration_record: &ArrayView2<f64>,
    mean_concentration: &ArrayView1<f64>,
) -> Array2<f64> {
    concentration_record / mean_concentration
}

pub fn variance_concentration(
    mean_concentration: &ArrayView1<f64>,
    concentration_record: &ArrayView2<f64>,
    full_volume: &ArrayView2<f64>,
) -> Array1<f64> {
    let mean_concentration = spatial_average_concentration(concentration_record, full_volume);
    ((concentration_record - mean_concentration).pow2() * full_volume).sum_axis(Axis(1))
}

// def normalize_concentration(
//     raw_concentration: np.ndarray, volumes: np.ndarray
// ) -> Tuple[np.ndarray, float, float]:
//     vtot = np.sum(volumes, axis=1)
//     mean_concentration = np.sum(raw_concentration * volumes, axis=1) / vtot
//     mean_concentration = mean_concentration.reshape(-1, 1)
//     variance = (
//         np.sum(np.power(raw_concentration - mean_concentration, 2) * volumes, axis=1)
//         / vtot
//     )
//     return raw_concentration / mean_concentration, mean_concentration, variance

pub struct Histogram {
    bins: Vec<f64>,
    counts: Vec<f64>,
    bin_counts: usize,
}

impl Histogram {
    pub fn new(bin_counts: usize) -> Self {
        Histogram {
            bins: Vec::new(),
            counts: Vec::new(),
            bin_counts,
        }
    }

    pub fn add(&mut self, values: Vec<f64>) {
        if values.is_empty() {
            return;
        }

        // Determine the range of the data
        let min = values.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let bin_width = (max - min) / self.bin_counts as f64;

        // Initialize bins if not already done
        if self.bins.is_empty() {
            self.bins = (0..self.bin_counts)
                .map(|i| min + i as f64 * bin_width)
                .collect();
            self.counts = vec![0.0; self.bin_counts];
        }

        // Add values to the appropriate bins
        for value in values {
            let bin_index = ((value - min) / bin_width).floor() as usize;
            let bin_index = bin_index.min(self.bin_counts - 1); // Clamp to the last bin if necessary
            self.counts[bin_index] += 1.0;
        }
    }

    pub fn get_bins(&self) -> &[f64] {
        &self.bins
    }

    pub fn get_counts(&self) -> &[f64] {
        &self.counts
    }
}

fn direct_growth_rate() {}
