use egui::Ui;

use crate::decay_simulator_v1::DecaySimApp;

impl DecaySimApp {

    pub fn graph_page(&self, ui: &mut Ui){


        let simulator_state_clone = 
            self.simulator_state.lock().unwrap().clone();

        let nuclides_to_plot = simulator_state_clone.get_nuclides_to_plot();
        let nuclide_fractions_over_time = simulator_state_clone.get_nuclides_fractions_over_time();


    }
}
