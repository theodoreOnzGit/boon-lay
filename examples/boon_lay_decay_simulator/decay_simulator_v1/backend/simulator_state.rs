use boon_lay::Nuclide;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SimulatorState {
    is_running: bool,
    restart_button_pressed: bool,
    change_nuclide_button_pressed: bool,
    user_selected_nuclide: Nuclide,


}

impl Default for SimulatorState {

    fn default() -> Self {

        let is_running = true;
        let restart_button_pressed = false;
        let change_nuclide_button_pressed = false;
        Self {
            is_running,
            restart_button_pressed,
            user_selected_nuclide: Nuclide::U238,
            change_nuclide_button_pressed,
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
}
