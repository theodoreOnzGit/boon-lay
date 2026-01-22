use std::sync::{Arc, Mutex};

use boon_lay::{prelude::{decay_library::DecayLibrary, SingleNuclideSimualtorMC}, Nuclide};

use crate::decay_simulator_v1::{backend::simulator_state::SimulatorState, DecaySimApp};


impl DecaySimApp {

    /// at each simulation, the simulator will run in the background 
    pub fn run_decay_chain_simulation(
        thread_ptr: Arc<Mutex<(Vec<SingleNuclideSimualtorMC>,DecayLibrary)>>,
        simulator_state_ptr: Arc<Mutex<SimulatorState>>,
        thread_number: u8,
        ){

        // this is the main loop
        loop {
            // firstly, we obtain the simulator state 
            let simulator_state_clone: SimulatorState 
                = simulator_state_ptr.lock().unwrap().clone();

            // check if the pause button is on 
            // if pause button on, skip all contents in current iteration
            if simulator_state_clone.is_paused() {
                continue;
            }



            // check if the restart button is pressed 

            if simulator_state_clone.is_restart_button_pressed() {

                // if restart button is pressed, then reset the simulator 
                // with the nuclide supplied by the user but all times set to zero 

                // I will deal with this part later
                
            }

            if simulator_state_clone.is_change_nuclide_button_pressed() {
                // check if change_nuclide button is pressed,
                // if so, change the nuclide button, but do not reset 
                // all the time to zero

                // I will deal with this part later
            }
            







        };



    }


    
}
