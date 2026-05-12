use std::time::SystemTime;
use std::time::Duration;
use std::thread;
use std::sync::{Arc, Barrier, Mutex};

use boon_lay::Nuclide;
use boon_lay::lagrangian_decay_simulator::lagrangian_diffusion::single_particle_simulator::cached_normals::DiffusionRandomCache;
use boon_lay::prelude::SingleNuclideSimulatorMC;
use boon_lay::prelude::decay_library::DecayLibrary;
use boon_lay::lagrangian_decay_simulator::lagrangian_diffusion::single_particle_simulator::SingleParticleDiffusionSimulatorMC;
use rand::SeedableRng;
use uom::si::time::second;
use uom::si::f64::Time;
use uom::si::time::millisecond;

use crate::triso_simulator_v1::TRISOSimApp;
use crate::triso_simulator_v1::front_end::triso_particle::TrisoParticleUi;
use crate::triso_simulator_v1::backend::simulator_state::SimulatorState;
use boon_lay::lagrangian_decay_simulator::lagrangian_diffusion::central_limit_theorem::oorandom_rng::OoRng64;


impl TRISOSimApp {

    /// at each simulation, the simulator will run in the background 
    ///
    /// now, challenge is, each simulator may run too fast, or slow 
    /// depending on thread speed, relative to other simulations
    ///
    ///
    /// the way to do it, according to ChatGPT5, is to use Arc barrier
    /// 
    pub fn run_decay_chain_simulation(
        thread_ptr: Arc<Mutex<(Vec<SingleNuclideSimulatorMC>,DecayLibrary)>>,
        simulator_state_ptr: Arc<Mutex<SimulatorState>>,
        thread_number: u8,
        barrier: Arc<Barrier>,
        ){

        let loop_time = SystemTime::now();

        // now i create the the SingleParticleDiffusionSimulatorMC 
        let mut particle_simulator_rng: OoRng64 = 
            OoRng64::from_seed([thread_number * 7_u8 ; 16]);

        let mut diffusion_simulator = 
            SingleParticleDiffusionSimulatorMC::new_from_rng(
                &mut particle_simulator_rng
            );

        let new_triso_particle_ui = TrisoParticleUi::default();

        let cached_normals = DiffusionRandomCache::new(1e5 as usize);

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

            // get the triso cell 
            let triso_cell = simulator_state_clone.triso_cell;


            // check if the restart button is pressed 

            if simulator_state_clone.is_restart_button_pressed() {

                // if restart button is pressed, then reset the simulator 
                // with the nuclide supplied by the user but all times set to zero 

                let (mut simulation_vector, mut decay_library): 
                    (Vec<SingleNuclideSimulatorMC>, DecayLibrary) = 
                     thread_ptr.lock().unwrap().clone();

                // let me get the nuclide of interest first 

                let user_set_nuclide: Nuclide = 
                    simulator_state_ptr.lock().unwrap().get_user_selected_nuclide();

                // get new position based on triso particle
                let _buffer_radius = new_triso_particle_ui.get_diameter_after_buffer() * 0.5;
                let _ipyc_radius = new_triso_particle_ui.get_diameter_after_ipyc() * 0.5;
                let fuel_radius = new_triso_particle_ui.get_diameter_after_fuel() * 0.5;
                let opyc_radius = new_triso_particle_ui.get_diameter_after_opyc() * 0.5;
                let mut rng_for_position = OoRng64::from_u64(thread_number as u64 *4);

                // this pre-simulates all the decay trajectories
                for simulation in simulation_vector.iter_mut() {

                    let mut new_simulation 
                        = SingleNuclideSimulatorMC::new_decay_chain_simulation(
                            user_set_nuclide, &mut decay_library
                        );

                    let coordinate = Self::random_point_in_triso(
                        fuel_radius, 
                        opyc_radius, 
                        &mut decay_library.random_number_generator,
                        &mut rng_for_position
                    );
                    new_simulation.position = coordinate;

                    *simulation = new_simulation;

                }


                // set timestep to 7s because diffusion is very fast
                let timestep_based_on_diffusion: Time = 
                    Time::new::<second>(1.0);


                // make sure all threads in sync 
                barrier.wait();

                // once done 
                *thread_ptr.lock().unwrap() = 
                    (simulation_vector, decay_library);
                if thread_number == 1 {

                    simulator_state_ptr.lock().unwrap().turn_off_restart_button();
                    simulator_state_ptr.lock().unwrap().turn_off_change_nuclide_button();
                    simulator_state_ptr.lock().unwrap().reset_simulated_time();
                    simulator_state_ptr.lock().unwrap().set_timestep(timestep_based_on_diffusion);

                    

                    // upon restarting, we must toggle a flag to replot 
                    // the nuclides 
                    simulator_state_ptr.lock().unwrap().turn_on_change_nuclide_to_plot_button();

                }

                // make sure all threads in sync 
                barrier.wait();
                
            }

            if simulator_state_clone.is_change_nuclide_button_pressed() {
                // check if change_nuclide button is pressed,
                // if so, change the nuclide button, but do not reset 
                // all the time to zero

                // first if this is thread 1, then we change 
                // all the restart and change nuclide off

                let (mut simulation_vector, mut decay_library): 
                    (Vec<SingleNuclideSimulatorMC>, DecayLibrary) = 
                     thread_ptr.lock().unwrap().clone();

                // let me get the nuclide of interest first 

                let user_set_nuclide: Nuclide = 
                    simulator_state_ptr.lock().unwrap().get_user_selected_nuclide();

                barrier.wait();
                // this pre-simulates all the decay trajectories
                //
                // change nuclide does not reset position
                for simulation in simulation_vector.iter_mut() {
                    simulation.transmute_nuclide(user_set_nuclide, &mut decay_library);

                }
                // once done 
                *thread_ptr.lock().unwrap() = 
                    (simulation_vector, decay_library.clone());



                // set timestep to 0.1% half life 
                let timestep_based_on_diffusion: Time = 
                    Time::new::<second>(1.0);
                // make sure all threads in sync 
                barrier.wait();

                if thread_number == 1 {

                    simulator_state_ptr.lock().unwrap().turn_off_restart_button();
                    simulator_state_ptr.lock().unwrap().turn_off_change_nuclide_button();
                    simulator_state_ptr.lock().unwrap().set_timestep(timestep_based_on_diffusion);

                    // upon changing nuclide, we must toggle a flag to replot 
                    // the nuclides 
                    simulator_state_ptr.lock().unwrap().turn_on_change_nuclide_to_plot_button();

                }

                // make sure all threads in sync 
                barrier.wait();
            }
            
            // so all the conditions for running are met 
            // now we just run

            // firstly, we lock the pointer 
            // making a clone of the simulation vector and library

            let (mut simulation_vector, decay_library): 
                (Vec<SingleNuclideSimulatorMC>, DecayLibrary) = 
                 thread_ptr.lock().unwrap().clone();

            // technically decay libraries are not needed here

            let timestep = simulator_state_clone.get_timestep();
            
            // all we are doing here is to advance_timestep
            for decay_simulation in simulation_vector.iter_mut() {
                // advance the decay portion
                decay_simulation.advance_timestep(timestep);
                // then I want to move the particle 

                //diffusion_simulator.move_single_decaying_particle_within_triso_based_on_fourier_no_cached(
                //    decay_simulation, 
                //    triso_cell, 
                //    timestep,
                //    &cached_normals
                //);

                diffusion_simulator.move_single_decaying_particle_within_triso_based_on_fourier_no(
                    decay_simulation, 
                    triso_cell, 
                    timestep,
                );

            };
            // once the decay simulation is complete, lock the thread ptr 
            // and return the simulation vector
            *thread_ptr.lock().unwrap() = (simulation_vector, decay_library);

            // now let's keep things in time 
            // this is for real-time simulation

            let realtime = false; 

            if realtime {
                let loop_time_end = loop_time.elapsed().unwrap();
                let time_taken_for_calculation_loop_milliseconds: f64 = 
                    (loop_time_end - loop_time_start)
                    .as_millis() as f64;

                let time_to_sleep_milliseconds: u64 = 
                    (timestep.get::<millisecond>() - 
                     time_taken_for_calculation_loop_milliseconds)
                    .round().abs() as u64;

                let time_to_sleep_realtime: Duration = 
                    Duration::from_millis(time_to_sleep_milliseconds - 1);
                // time to sleep for real-time (default)
                thread::sleep(time_to_sleep_realtime);
            } else {
                let time_to_sleep_milliseconds: u64 = 
                    5;
                let time_to_sleep_non_realtime: Duration = 
                    Duration::from_millis(time_to_sleep_milliseconds);
                thread::sleep(time_to_sleep_non_realtime);
            }

            barrier.wait();

            // again, only thread 1 is responsible to timekeeping
            // other threads don't touch

            if thread_number == 1 {

                simulator_state_ptr.lock().unwrap().add_to_simulated_time(timestep);

                let elapsed_time_seconds = 
                    (loop_time.elapsed().unwrap().as_secs_f64() * 100.0).round()/100.0;
                let elapsed_time = Time::new::<second>(elapsed_time_seconds);

                // count fraction remaining (approx)
                let (simulation_vector, _decay_library): 
                    (Vec<SingleNuclideSimulatorMC>, DecayLibrary) = 
                     thread_ptr.lock().unwrap().clone();

                let user_set_nuclide: Nuclide = 
                    simulator_state_ptr.lock().unwrap().get_user_selected_nuclide();

                let mut surviving_nuclide_counter = 0;

                for nuclide_simulation in simulation_vector.iter() {
                    if nuclide_simulation.check_if_current_nuclide_matches(user_set_nuclide){
                        surviving_nuclide_counter += 1;
                    }
                }


                let nuclide_fraction_remaining = 
                    surviving_nuclide_counter as f64 / 
                    simulation_vector.clone().len() as f64 ;

                simulator_state_ptr.lock().unwrap().set_nuclide_fraction(
                    nuclide_fraction_remaining
                );

                simulator_state_ptr.lock().unwrap().set_elapsed_time(elapsed_time);

            }


            // thread 2 will be responsible for constructing the nuclide fraction 
            // vector 
            if thread_number == 2 {


                let (simulation_vector, _decay_library): 
                    (Vec<SingleNuclideSimulatorMC>, DecayLibrary) = 
                     thread_ptr.lock().unwrap().clone();


                let mut nuclide_vector: Vec<Nuclide> = vec![];

                for simulation in &simulation_vector {
                    let nuclide = simulation.get_current_nuclide();
                    nuclide_vector.push(nuclide);
                }

                let nuclide_fraction_vector: Vec<(Nuclide, f64)> = 
                    TRISOSimApp::fractions_vec_map(&nuclide_vector);


                simulator_state_ptr.lock().unwrap().set_nuclide_fraction_vector(
                    nuclide_fraction_vector);




            }

            // thread 3 is responsible for debugging  triso particles 
            let debug = false;
            if thread_number == 3 && debug {


                let (simulation_vector, _decay_library): 
                    (Vec<SingleNuclideSimulatorMC>, DecayLibrary) = 
                     thread_ptr.lock().unwrap().clone();

                let decay_sim = simulation_vector[0].clone();

                let pos = decay_sim.position;
                let elapsed_time_seconds = 
                    (loop_time.elapsed().unwrap().as_secs_f64() * 100.0).round()/100.0;
                let elapsed_time = Time::new::<second>(elapsed_time_seconds);
                let nuclide = decay_sim.get_current_nuclide();

                dbg!(&(elapsed_time,nuclide,pos));

            }
            barrier.wait();



        };



    }


    
}
