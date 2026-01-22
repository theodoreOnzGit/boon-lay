use std::{sync::{Arc, Barrier, Mutex}, thread, time::{Duration, SystemTime}};

use boon_lay::{prelude::{decay_library::DecayLibrary, SingleNuclideSimualtorMC}, Nuclide};
use uom::si::{f64::Time, time::second};

use crate::decay_simulator_v1::{backend::simulator_state::SimulatorState, DecaySimApp};


impl DecaySimApp {

    /// at each simulation, the simulator will run in the background 
    ///
    /// now, challenge is, each simulator may run too fast, or slow 
    /// depending on thread speed, relative to other simulations
    ///
    ///
    /// the way to do it, according to ChatGPT5, is to use Arc barrier
    /// 
    pub fn run_decay_chain_simulation(
        thread_ptr: Arc<Mutex<(Vec<SingleNuclideSimualtorMC>,DecayLibrary)>>,
        simulator_state_ptr: Arc<Mutex<SimulatorState>>,
        thread_number: u8,
        barrier: Arc<Barrier>,
        ){

        let loop_time = SystemTime::now();


        // this is the main loop
        loop {


            let loop_time_start = loop_time.elapsed().unwrap();
            // firstly, we obtain the simulator state 
            let simulator_state_clone: SimulatorState 
                = simulator_state_ptr.lock().unwrap().clone();
            // now, for timekeeping, only thread 1 is responsible, 
            // no other thread is important in this regard
            // other barriers will stay in sync because of the barrier 
            // code


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
            
            // so all the conditions for running are met 
            // now we just run

            // firstly, we lock the pointer 
            // making a clone of the simulation vector and library

            let (mut simulation_vector, decay_library): 
                (Vec<SingleNuclideSimualtorMC>, DecayLibrary) = 
                 thread_ptr.lock().unwrap().clone();

            // technically decay libraries are not needed here

            let timestep = simulator_state_clone.get_timestep();
            
            // all we are doing here is to advance_timestep
            for decay_simulation in simulation_vector.iter_mut() {
                decay_simulation.advance_timestep(timestep);
            };

            // once the decay simulation is complete, lock the thread ptr 
            // and return the simulation vector
            *thread_ptr.lock().unwrap() = (simulation_vector, decay_library);

            // now let's keep things in time


            // just sleep for 50 ms each time, default
            let time_to_sleep: Duration = 
                Duration::from_millis(500);

            thread::sleep(time_to_sleep);

            barrier.wait();

            // again, only thread 1 is responsible to timekeeping
            // other threads don't touch

            if thread_number == 1 {

                let elapsed_time_seconds = 
                    (loop_time.elapsed().unwrap().as_secs_f64() * 100.0).round()/100.0;
                let elapsed_time = Time::new::<second>(elapsed_time_seconds);

                simulator_state_ptr.lock().unwrap().set_elapsed_time(elapsed_time);

            }


        };



    }


    
}
