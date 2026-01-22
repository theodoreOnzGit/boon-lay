use boon_lay::Nuclide;
use egui::Ui;
use uom::si::{f64::Time, time::{day, millisecond, second, year}};

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


        let simulated_time = simulator_state_clone.get_simulated_time();
        let mut simulated_time_string: String = "Simulated Time (years):".to_string();
        simulated_time_string += &simulated_time.get::<year>().to_string();
        ui.label(simulated_time_string);
        ui.label(" ");

        let simulated_time = simulator_state_clone.get_simulated_time();
        let mut simulated_time_string: String = "Simulated Time (billion years):".to_string();
        simulated_time_string += &(simulated_time.get::<year>()/1e9 as f64).to_string();
        ui.label(simulated_time_string);
        ui.label(" ");


        // display timestep
        ui.label(" ");
        let timestep = simulator_state_clone.get_timestep();
        let mut timestep_string: String = "Timestep (seconds):".to_string();
        timestep_string += &timestep.get::<second>().to_string();
        ui.label(timestep_string);
        ui.label(" ");

        // I also want to display this in milliseconds, days, years
        let mut timestep_string: String = "Timestep (milliseconds):".to_string();
        timestep_string += &timestep.get::<millisecond>().to_string();
        ui.label(timestep_string);
        ui.label(" ");

        let mut timestep_string: String = "Timestep (days):".to_string();
        timestep_string += &timestep.get::<day>().to_string();
        ui.label(timestep_string);
        ui.label(" ");
        let mut timestep_string: String = "Timestep (years):".to_string();
        timestep_string += &timestep.get::<year>().to_string();
        ui.label(timestep_string);
        ui.label(" ");

        ui.separator();

        // timestep settings

        let mut user_set_timestep_seconds 
            = simulator_state_clone.get_timestep().get::<second>();

        let timestep_slider_seconds = egui::Slider::new(
            &mut user_set_timestep_seconds, 
            0.00001..=1e20
        ) .logarithmic(true) .text("Timestep Control (s)") .drag_value_speed(0.001);

        // set timestep 
        ui.add(timestep_slider_seconds);
        let timestep = Time::new::<second>(user_set_timestep_seconds);
        self.simulator_state.lock().unwrap().set_timestep(timestep);
        ui.separator();


        ui.label("Select nuclide :");

        let mut nuclide = simulator_state_clone.get_user_selected_nuclide();

        egui::ComboBox::from_label("User Selected Nuclide")
            .selected_text(format!("{:?}", nuclide))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut nuclide, Nuclide::U238, "U238");
                ui.selectable_value(&mut nuclide, Nuclide::U235, "U235");
                ui.selectable_value(&mut nuclide, Nuclide::Cs137, "Cs137");
                ui.selectable_value(&mut nuclide, Nuclide::I131, "I131");
            });

        ui.label(format!("User Selected Nuclide: {:?}", nuclide));
        ui.separator();

        self.simulator_state.lock().unwrap().set_user_selected_nuclide(
            nuclide
        );




    }

}

