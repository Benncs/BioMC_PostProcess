
use std::collections::HashMap;

use super::{Dim, ResultGroup,Weight};


#[derive(Debug)]
pub struct Misc {
    pub n_node_thread: u64,
    pub n_rank: u64,
}



#[derive(Debug)]
pub struct MainRecords {
    pub concentration_liquid: Vec<f64>,
    pub volume_liquid: Vec<f64>,
    pub concentration_gas: Option<Vec<f64>>,
    pub volume_gas: Option<Vec<f64>>,
    pub mtr: Option<Vec<f64>>,
    pub dim: Dim,
    pub time:Vec<f64>,
}

#[derive(Debug)]
pub struct MainInitial {
    pub delta_time: f64,
    pub final_time: f64,
    pub initial_biomass_concentration: f64,
    pub initial_weight: f64,
    pub n_map: usize,
    pub number_compartment: usize,
    pub number_particles: u64,
    pub t_per_flow_map: f64,
}

#[derive(Debug)]
pub struct MainFInal {
    pub events: HashMap<String, u64>,
    pub number_particles: u64,
}

#[derive(Debug)]
pub struct MainResult {
    pub records: MainRecords,
    pub initial: MainInitial,
    pub cfinal: MainFInal,
    pub misc: Misc,
    pub weight: Weight
}

impl MainResult {
    pub fn read(name: &str) -> hdf5::Result<MainResult> {
        let file = hdf5::File::open_as(name, hdf5::file::OpenMode::Read)?;

        let m_ds = file.group("initial_parameters")?;
        let initial = ResultGroup::<MainInitial>::read_g(&m_ds)?;

        let m_ds = file.group("misc")?;
        let misc = ResultGroup::<Misc>::read_g(&m_ds)?;
        
        let m_ds = file.group("records")?;
        let records = ResultGroup::<MainRecords>::read_g(&m_ds)?;

        let m_ds = file.group("final_result")?;
        let cfinal = ResultGroup::<MainFInal>::read_g(&m_ds)?;


        let weight = Weight::Single(initial.initial_weight); //TODO switch between single and multi weight

        Ok(MainResult {
            records,
            initial,
            cfinal,
            misc,
            weight
        })
    }

    pub fn time(&self)->&[f64]
    {
        &self.records.time
    }
}