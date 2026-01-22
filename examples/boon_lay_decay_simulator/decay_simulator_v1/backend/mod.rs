use std::sync::Arc;

use boon_lay::prelude::SingleNuclideSimualtorMC;
use egui::mutex::Mutex;

use crate::decay_simulator_v1::DecaySimApp;

impl DecaySimApp {

    pub fn construct_new_nuclide_vector(num_of_nuclides: u64) -> 
        Arc<Mutex<Vec<SingleNuclideSimualtorMC>>>{

            todo!();
    }

    
}
