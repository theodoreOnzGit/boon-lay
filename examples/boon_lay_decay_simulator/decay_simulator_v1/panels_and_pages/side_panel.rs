use egui::Ui;
use uom::si::time::second;

use crate::decay_simulator_v1::{backend::simulator_state::SimulatorState, DecaySimApp};


impl DecaySimApp {

        pub fn side_panel(&mut self, ui: &mut Ui){

        ui.heading("Timestep and Time");
        
        // basically, get the simulator state first 
        ui.label(" ");

        let simulator_state_clone : SimulatorState
            = self.simulator_state.lock().unwrap().clone();

        // display elapsed time
        let elapsed_time = simulator_state_clone.get_elapsed_time();
        let mut elapsed_time_string: String = "Elapsed Time (seconds):".to_string();
        elapsed_time_string += &elapsed_time.get::<second>().to_string();

        ui.label(elapsed_time_string);
        ui.label(" ");

        // display simulated time

        ui.label(" ");
        let simulated_time = simulator_state_clone.get_simulated_time_seconds_2dp();
        let mut simulated_time_string: String = "Simulated Time (seconds):".to_string();
        simulated_time_string += &simulated_time.to_string();
        ui.label(simulated_time_string);
        ui.label(" ");

        // display timestep
        ui.label(" ");
        let timestep = simulator_state_clone.get_timestep();
        let mut timestep_string: String = "Timestep (seconds):".to_string();
        timestep_string += &timestep.get::<second>().to_string();
        ui.label(timestep_string);
        ui.label(" ");

    }

}

