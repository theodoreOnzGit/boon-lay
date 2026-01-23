use boon_lay::Nuclide;
use uom::{si::{f64::Time, time::second}, ConstZero};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SimulatorState {
    is_running: bool,
    restart_button_pressed: bool,
    change_nuclide_button_pressed: bool,
    user_selected_nuclide: Nuclide,
    user_selected_timestep: Time,
    elapsed_time: Time, 
    simulated_time: Time,


}

impl Default for SimulatorState {

    fn default() -> Self {

        let is_running = true;
        let restart_button_pressed = false;
        let change_nuclide_button_pressed = false;
        let user_selected_timestep = Time::new::<second>(1.0);
        let elapsed_time = Time::ZERO;
        let simulated_time = Time::ZERO;
        Self {
            is_running,
            restart_button_pressed,
            user_selected_nuclide: Nuclide::U238,
            change_nuclide_button_pressed,
            user_selected_timestep,
            elapsed_time,
            simulated_time,
        }
    }
}

impl SimulatorState {

    pub fn is_paused(&self) -> bool {
        return !self.is_running;
    }

    pub fn is_restart_button_pressed(&self) -> bool {
        return self.restart_button_pressed;
    }
    pub fn is_change_nuclide_button_pressed(&self) -> bool {
        return self.change_nuclide_button_pressed;
    }

    // timestep settings
    pub fn get_timestep(&self) -> Time {
        return self.user_selected_timestep;
    }
    pub fn set_timestep(&mut self, timestep: Time){
        self.user_selected_timestep = timestep;
    }

    // these are for elapsed time

    pub fn set_elapsed_time(&mut self, elapsed_time: Time){
        self.elapsed_time = elapsed_time;
    }

    pub fn get_elapsed_time(&self) -> Time {
        self.elapsed_time
    }


    // these are for simulated time

    pub fn add_to_simulated_time(&mut self, timestep: Time){
        self.simulated_time += timestep;
    }
    pub fn get_simulated_time(&self) -> Time {
        self.simulated_time
    }
    pub fn get_simulated_time_seconds_2dp(&self) -> f64 {
        return (self.get_simulated_time().get::<second>()*100_f64).round()/100.0;
    }
    pub fn reset_simulated_time(&mut self){
        self.simulated_time = Time::ZERO;
    }


    // for getting and setting nuclide 
    pub fn set_user_selected_nuclide(&mut self, user_selected_nuclide: Nuclide){
        self.user_selected_nuclide = user_selected_nuclide;
    }

    pub fn get_user_selected_nuclide(&self) -> Nuclide {
        self.user_selected_nuclide
    }


}

