use csv::Writer;
use ndarray::ArrayView2;
use serde::{Deserialize, Serialize};
use serde_json;
use super::vec_to_array_view2;
#[derive(Debug, Serialize, Deserialize)]
pub struct Tallies(pub Vec<f64>);

impl Tallies {
    pub fn validate(&self) -> bool {
        self.0.len() % 6 == 0
    }

    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(&self.0)
    }

    pub fn to_csv(&self) -> Result<String, String> {
        if !self.validate() {
            return Err(
                "Validation failed: The number of elements is not divisible by 6.".to_string(),
            );
        }

        let headers = vec![
            "NewParticle",
            "Death",
            "Move",
            "Exit",
            "Overflow",
            "ChangeWeight",
        ];
        let mut wtr = Writer::from_writer(vec![]);

        wtr.write_record(&headers).map_err(|e| e.to_string())?;

        for row in self.0.chunks(6) {
            wtr.serialize(row).map_err(|e| e.to_string())?;
        }
        let data = String::from_utf8(wtr.into_inner().map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
        Ok(data)
    }

    pub fn to_array(&self)->ArrayView2<f64>
    {
        vec_to_array_view2(&self.0, self.0.len()/6, 6)
    }
}