use std::time::SystemTime;

use fission_yields_data::prelude::Nuclide;
use oorandom::Rand64;
use uom::{ConstZero, si::{f64::*, time::millisecond}};

use crate::lagrangian_decay_simulator::StochasticDecayChain;
use crate::prelude::HalfLifeAndDecayEnergyInfo;

#[derive(Debug,Clone,PartialEq)]
pub struct SingleNuclideSimualtorMC {
    /// the current nuclide the simulator is simulating
    /// it can change over time
    pub current_nuclide: Nuclide,
    /// current half life information for current nuclide 
    pub current_half_life_info: HalfLifeAndDecayEnergyInfo,
    /// time passed in the simulation
    pub simulated_time: Time,
    /// time passed in real-life
    pub elapsed_time: Time,

    /// the decay chain in equation 
    stochastic_decay_chain: StochasticDecayChain,

    /// the time to live vector showing how long the subsequent nuclides 
    /// live
    ///
    /// basically, time remaining to next decay
    time_to_live_vec: Vec<Time>,
}

// basically the idea of this simulator is to take a current nuclide,
// then simulate the decay of it over time 
//
// so it will take the current nuclide and before simulation:
// 1. generate the decay chain of subsequent nuclides 
// 2. generate a vector of time remaining to the next nuclide 
// (this is stochastically determined based on half life)
//
// Now during simulation, the simulator will take in a timestep, and 
// based on that timestep given, proceed to the next nuclide
//
// Note: if concerns exist about computational expense, 
// we can always speed up simulation using flamegraph later.
//
//

impl SingleNuclideSimualtorMC {

    /// this obtains a time to live stochastically for the decay chain using 
    /// half life 
    /// From:
    /// N = N\_0 exp(-lambda * t)
    ///
    /// we get:
    /// t = Ln (N/N\_0) / (-lambda)
    /// t = Ln (N/N\_0) / (ln 2) * half life.
    ///
    /// N/N\_0 is a random number between 0 and 1
    pub fn get_time_to_decay_stochastic(rng: &mut Rand64, half_life: Time) 
        -> Time {

            let n_by_n0 = rng.rand_float();

            let half_life_coeff: f64 = n_by_n0.ln().abs() / (2.0_f64.ln());

            return half_life_coeff * half_life;

    }

    /// generate a new decay chain simulation 
    pub fn new_decay_chain_simulation(nuclide: Nuclide) -> Self {



        todo!()
    }
    /// generate a new decay chain simulation 
    /// based on a new nuclide, usually due to transmutation, 
    /// but keep the elapsed time and simulated time
    pub fn transmute_nuclide(&mut self, nuclide: Nuclide) -> Self {



        todo!()
    }

    // move the simulation forward by some time supplied by the user
    // also provides the nuclide of interest currently
    pub fn step_forward_simulation(&mut self, timestep: Time) -> 
        (Nuclide, HalfLifeAndDecayEnergyInfo){
        // we do loop timing
        let loop_time = SystemTime::now();
        let loop_time_start = loop_time.elapsed().unwrap();

        // main calculation loop
        //
        // given a timestep, the job is to then find the next nuclide
        {
            // first, add the timestep 
            self.simulated_time += timestep;
            // this is for multiple nuclides remaining
            // if timestep overshoots several nuclide decays, this is 
            // helpful
            let mut timestep_remaining = timestep;


            for (i, (nuclide,half_life_info)) in 
                self.stochastic_decay_chain.iter().enumerate() {

                    let time_to_next_nuclide = self.time_to_live_vec[i];

                    // in the case timestep is less than the decay to next 
                    // nuclide, deduct the time to live for the time 
                    // to next nuclide
                    //
                    // the current nuclide has not decayed yet
                    if timestep_remaining < time_to_next_nuclide {
                        self.time_to_live_vec[i] -= timestep;
                        break;
                    };

                    // in the case timestep is equal to decay of next nuclide 
                    // we have a single decay
                    if timestep_remaining == time_to_next_nuclide {

                        // time to live for next nuclide is zero
                        self.time_to_live_vec[i] = Time::ZERO;
                        self.current_nuclide = *nuclide;
                        self.current_half_life_info = half_life_info.clone();
                        // break out of the loop
                        break;

                    }

                    // in case timestep is more than time to next nuclide,
                    if timestep_remaining > time_to_next_nuclide {

                        // subtract the time to live vector from the timestep 
                        // remaining
                        timestep_remaining -= self.time_to_live_vec[i];
                        self.time_to_live_vec[i] = Time::ZERO;
                        self.current_nuclide = *nuclide;
                        self.current_half_life_info = half_life_info.clone();

                        // once done, continue to the next timestep

                    }



            }

        }

        // now let's tidy up
        // first the stochastic decay chain vector 

        for time_to_live in self.time_to_live_vec.iter(){

            if *time_to_live == Time::ZERO {

                // basically we keep removing if we have time to live = 0 
                // then we remove the first element of 
                // the vector in the decay chain

                self.stochastic_decay_chain.nuclides_and_decay_data_vec.remove(0);


            } else if *time_to_live > Time::ZERO {

                // basically this is this is the case if the time to live is 
                // greater than 0, then we stop doing this 
                // this is important because if the rng gives 
                //
                // time to live
                // [
                // 10s,
                // 0s,
                // 30s 
                // ] 
                //
                // I don't want to remove the 0s time to live just yet, 
                // that will mess up the logic 
                //
                // save that for the next time
                // 
                
                break;

            }

        }

        // next i will clean up the time to live 
        // this will clear up the leading zeroes

        // this was advised by ChatGPT5
        //


        let first_non_zero = 
            self.time_to_live_vec.iter()
            .position(|&x| x != Time::ZERO)
            .unwrap_or(self.time_to_live_vec.len());
        self.time_to_live_vec.drain(0..first_non_zero);

        // now that we've cleared up the vectors, we can time the simulation




        let loop_time_end = loop_time.elapsed().unwrap();
        let time_taken_for_calculation_loop_milliseconds: f64 = 
            (loop_time_end - loop_time_start)
            .as_millis() as f64;

        self.elapsed_time += Time::new::<millisecond>(
            time_taken_for_calculation_loop_milliseconds
        );

        return (self.current_nuclide, self.current_half_life_info.clone());

    }

    /// as function name implies, get time to next decay
    /// unless the radionuclide is already stable
    #[inline]
    pub fn get_time_to_next_decay(&self) -> Option<Time> {

        match self.current_half_life_info {
            HalfLifeAndDecayEnergyInfo::Stable => return None,
            HalfLifeAndDecayEnergyInfo::Unstable(_, _) => {

            },
        }

        // if we have decays, 

        return Some(*self.time_to_live_vec.first().unwrap());


    }
    /// as function name implies, get nuclide in next decay
    /// unless the radionuclide is already stable
    /// then returns None
    #[inline]
    pub fn get_next_decay_nuclide(&self) -> Option<Nuclide> {

        match self.current_half_life_info {
            HalfLifeAndDecayEnergyInfo::Stable => return None,
            HalfLifeAndDecayEnergyInfo::Unstable(_, _) => {

            },
        }

        // if we have decays, the next nuclide is the first 
        // in the vector
        let (next_nuclide, _half_life_info) = self
            .stochastic_decay_chain
            .nuclides_and_decay_data_vec.first().unwrap();

        return Some(*next_nuclide);


    }
    /// as function name implies, get the time to live vector
    #[inline]
    pub fn get_time_to_live_vec(&self) -> Vec<Time> {

        return self.time_to_live_vec.clone();
    }
    #[inline]
    pub fn get_decay_chain_vec(&self) -> Vec<Nuclide> {
        let mut decay_chain_vec: Vec<Nuclide> = vec![];

        for (nuclide,_half_life_info) in &self.stochastic_decay_chain {
            decay_chain_vec.push(*nuclide);
        }


        return decay_chain_vec;
    }

}


#[cfg(test)]
pub mod tests;
