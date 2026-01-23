use std::sync::{Arc, Mutex};

use boon_lay::prelude::{SingleNuclideSimulatorMC, decay_library::DecayLibrary};

use crate::decay_simulator_v1::backend::simulator_state::SimulatorState;

impl SimulatorState {

    pub fn update_fractions_using_decay_sim_thread_ptrs(
        &mut self,
        decay_sim_plotting_thread_1_ptr: Arc<Mutex<(Vec<SingleNuclideSimulatorMC>, DecayLibrary)>>,
        decay_sim_plotting_thread_2_ptr: Arc<Mutex<(Vec<SingleNuclideSimulatorMC>, DecayLibrary)>>,
        decay_sim_plotting_thread_3_ptr: Arc<Mutex<(Vec<SingleNuclideSimulatorMC>, DecayLibrary)>>,
        decay_sim_plotting_thread_4_ptr: Arc<Mutex<(Vec<SingleNuclideSimulatorMC>, DecayLibrary)>>,
    ){
        // code to update plotting will be here

    }


}
