use std::sync::{Arc, Mutex};

use boon_lay::{prelude::{decay_library::DecayLibrary, SingleNuclideSimulatorMC}, Nuclide};

use crate::decay_simulator_v1::backend::simulator_state::SimulatorState;

impl SimulatorState {

    /// this is code to update the plots 
    ///
    /// Now, here's the problem with plotting decay chains.
    ///
    /// Firstly, there is more than one route for decay chains,
    /// Secondly, if we change nuclides, then the nuclides we are interested 
    /// to plot will definitely differ as the whole decay chain will differ
    ///
    /// I want the code to be such that it will change the nuclides to plot 
    /// when we hit the change nuclide or reset button
    ///
    /// So, when we hit the reset button or change nuclide button,
    /// the decay paths and nuclides will be pre-determined. This shouldn't 
    /// be an issue
    ///
    /// 
    ///
    pub fn update_fractions_using_decay_sim_thread_ptrs(
        &mut self,
        decay_sim_plotting_thread_1_ptr: Arc<Mutex<(Vec<SingleNuclideSimulatorMC>, DecayLibrary)>>,
        decay_sim_plotting_thread_2_ptr: Arc<Mutex<(Vec<SingleNuclideSimulatorMC>, DecayLibrary)>>,
        decay_sim_plotting_thread_3_ptr: Arc<Mutex<(Vec<SingleNuclideSimulatorMC>, DecayLibrary)>>,
        decay_sim_plotting_thread_4_ptr: Arc<Mutex<(Vec<SingleNuclideSimulatorMC>, DecayLibrary)>>,
    ){

        // before anything, just obtain data by cloning and drop the reference

        let (nuclide_sim_vec_1, _decay_lib_1) 
            = decay_sim_plotting_thread_1_ptr.lock().unwrap().clone();
        let (nuclide_sim_vec_2, _decay_lib_2) 
            = decay_sim_plotting_thread_2_ptr.lock().unwrap().clone();
        let (nuclide_sim_vec_3, _decay_lib_3) 
            = decay_sim_plotting_thread_3_ptr.lock().unwrap().clone();
        let (nuclide_sim_vec_4, _decay_lib_4) 
            = decay_sim_plotting_thread_4_ptr.lock().unwrap().clone();

        // then we convert them all into a vector of nuclides 


        let full_nuclide_sim_vec: Vec<SingleNuclideSimulatorMC> = 
            nuclide_sim_vec_1.into_iter()
            .chain(nuclide_sim_vec_2)
            .chain(nuclide_sim_vec_3)
            .chain(nuclide_sim_vec_4)
            .collect();


        // we should combine the vectors using chain


        // now, for this, the first thing to check is if we should change 
        // the graph plotting.
        //
        if self.should_change_nuclide_to_graph_plot {

            // if we need to change the nuclide to plot, then
            // we should update this from the start

            let nuclides_to_plot: Vec<Nuclide> = 
                SingleNuclideSimulatorMC::all_chain_nuclides_unique_sorted(
                    &full_nuclide_sim_vec
                );

            self.nuclides_to_plot = nuclides_to_plot;


            // once update is done, then turn the change nuclide to plot button off

            self.turn_off_change_nuclide_to_plot_button();

        }

        // now we should have a list of nuclides to plot,
        let nuclides_to_plot: Vec<Nuclide> = self.nuclides_to_plot.clone();

        // then we should count them 

        let nuclide_count_vector: Vec<(Nuclide, u64)> = 
            SingleNuclideSimulatorMC::count_nuclides_in_sims_linear(
                &full_nuclide_sim_vec, 
                &nuclides_to_plot
            );



        // the graph plot is updated now

        

    }


}
