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
    pub fn get_timestep(&self) -> Time {
        return self.user_selected_timestep;
    }


    pub fn set_elapsed_time(&mut self, elapsed_time: Time){
        self.elapsed_time = elapsed_time;
    }

    pub fn get_elapsed_time(&self) -> Time {
        self.elapsed_time
    }
}

