use ndarray::{Array1, Array2, ArrayView2, ArrayView3, Axis};

pub fn spatial_average_concentration(
    concentration_record: &ArrayView2<f64>,
    full_volume: &ArrayView2<f64>,
) -> Array1<f64> {
    let weighted_concentration = concentration_record * full_volume;
    let numerator = weighted_concentration.sum_axis(Axis(1));
    let denominator = full_volume.sum_axis(Axis(1));
    numerator / denominator
}

pub struct Histogram {
    bins: Vec<f64>,
    counts: Vec<f64>,
    bin_width: f64,
}

impl Histogram {
    pub fn new(bin_width: f64) -> Self {
        Histogram {
            bins: Vec::new(),
            counts: Vec::new(),
            bin_width,
        }
    }

    pub fn add(&mut self, values: Vec<f64>) {
        for value in values {
            let bin_index = self.find_or_create_bin(value);
            self.counts[bin_index] += 1.0;
        }
    }

    fn find_or_create_bin(&mut self, value: f64) -> usize {
        let start = (value / self.bin_width).floor() * self.bin_width;

        if let Some(index) = self.bins.iter().position(|&bin| bin == start) {
            return index;
        }
        self.bins.push(start);
        self.counts.push(0.0);

        let mut indices: Vec<usize> = (0..self.bins.len()).collect();
        indices.sort_by(|&i, &j| self.bins[i].partial_cmp(&self.bins[j]).unwrap());

        self.bins = indices.iter().map(|&i| self.bins[i]).collect();
        self.counts = indices.iter().map(|&i| self.counts[i]).collect();

        self.bins.iter().position(|&bin| bin == start).unwrap()
    }

    pub fn get_bins(&self) -> &[f64] {
        &self.bins
    }

    pub fn get_counts(&self) -> &[f64] {
        &self.counts
    }
}
