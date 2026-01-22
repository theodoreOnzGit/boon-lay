use std::sync::{Arc, Mutex};

use boon_lay::{prelude::{decay_library::DecayLibrary, SingleNuclideSimualtorMC}, Nuclide};

use crate::decay_simulator_v1::DecaySimApp;

impl DecaySimApp {

    /// this basically constructs a simulation for a single thread to run 
    pub fn construct_new_single_thread_multi_particle_simulation(num_of_nuclides: u64,
        nuclide: Nuclide)-> 
        Arc<Mutex<(Vec<SingleNuclideSimualtorMC>, DecayLibrary)>>{

            let mut decay_library = DecayLibrary::new();

            let new_simulation 
                = SingleNuclideSimualtorMC::new_decay_chain_simulation(
                    nuclide, &mut decay_library
                );
            let v: Vec<SingleNuclideSimualtorMC> = vec![
                new_simulation; num_of_nuclides.try_into().unwrap()
            ];


            return Arc::new(Mutex::new(
                    (v,decay_library)
            ));
    }

    
}
