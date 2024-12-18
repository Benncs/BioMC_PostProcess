mod datamodel;
mod process;


use datamodel::{vec_to_array_view2, vec_to_array_view3, Dim, MainResult, Results};
use ndarray::{s, Array1, Array2, Axis};
use process::spatial_average_concentration;

#[derive(Debug)]

pub struct PostProcess {
    folder: String,
    root: String,
    dest: String,
    results:Results
}

#[derive(Clone,PartialEq)]
pub enum Phase
{
    Liquid,
    Gas
}

impl PostProcess
{
    pub fn new(folder: &str, root: Option<String>) -> Result<Self,()>
    {
        let _root  =root.unwrap_or_else(|| "./results/".to_string());
        let result_path = format!("{}/{}/{}.h5",_root,folder,folder);
        let main = Results::new(&result_path,&_root,folder);
        if main.is_none()
        {
            panic!("todo")
        }

        Ok(Self {
            folder:folder.to_string(),
            root: _root,
            dest: String::new(), 
            results: main.unwrap() 
        })
    }


    pub fn time(&self) -> &[f64]
    {
        self.results.0.time()
    }

    pub fn get_spatial_average_concentration(&self, species: usize, phase: Phase) -> Array1<f64> {
        // Helper 
        fn process_phase(
            concentration: &Vec<f64>,
            volume: &Vec<f64>,
            nt: usize,
            dim: &Dim,
            species: usize,
        ) -> Array1<f64> {
          
                let cl = vec_to_array_view3(concentration, &dim, nt);
                let vol = vec_to_array_view2(volume, nt, dim.0);
                let res = spatial_average_concentration(&cl.slice(s![.., .., species]), &vol);
    
                res
            
        }
    
        let r = &self.results.0.records;
        let nt = r.time.len();
        let dim = &r.dim;
    
        match phase {
            Phase::Gas => {
                if let (Some(c), Some(v)) = (&r.concentration_gas ,&r.volume_gas)
                {
                    return process_phase(c, v, nt, &dim, species)
                }
              
                panic!("Gas is not present");
            }
            Phase::Liquid => process_phase(&r.concentration_liquid, &r.volume_liquid, nt, &dim, species),
        }
    }

    pub fn get_time_average_concentration(&self, species: usize, position: usize, phase: Phase) -> Result<Array1<f64>,String> {
        let r = &self.results.0.records;
        let nt = r.time.len();
        let dim = &r.dim;

        let callback = |c:&Vec<f64>|{
                let cl = vec_to_array_view3(&c, &dim, nt);
                
                cl.slice(s![.., .., species]).mean_axis(Axis(0)).unwrap()
        };


        return match phase {
            Phase::Liquid => 
            {
                
                Ok(callback(&r.concentration_liquid))
            
            }
            Phase::Gas=>{
                if let Some(c) = &r.concentration_gas
                {
                     return Ok(callback(c));
                }
              
                return Err("Gas is not present".to_string());
            }
        }
        
    }

    pub fn get_biomass_concentration(&self) -> Array2<f64> {

        let nt = self.results.0.records.time.len();
        let mut cx = Array2::zeros((nt,self.results.0.records.dim.0));

        
        let err = datamodel::read_model_mass(&self.results.1,&mut cx,nt);
        if !err.is_ok()
        {
            panic!("Error read mass");
        }

        let vol = vec_to_array_view2(&self.results.0.records.volume_liquid, nt, self.results.0.records.dim.0);
        cx=cx*self.results.0.initial.initial_weight/vol;

        cx

    }

    pub fn get_growth_in_number(&self) -> Array1<f64> {
        self.results.2.sum_axis(Axis(1))
    }

    pub fn get_number_particle<'py>(&self) -> &Array2<f64> {
        &self.results.2
    }

}