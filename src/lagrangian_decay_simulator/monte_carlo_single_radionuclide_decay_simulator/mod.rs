use fission_yields_data::prelude::Nuclide;
use oorandom::Rand64;
use uom::si::f64::*;

#[derive(Debug,Clone,PartialEq)]
pub struct SingleNuclideSimualtorMC {
    /// the current nuclide the simulator is simulating
    /// it can change over time
    pub current_nuclide: Nuclide,
    /// time passed in the simulation
    pub simulated_time: Time,
    /// time passed in real-life
    pub elapsed_time: Time,
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

            let half_life_coeff: f64 = n_by_n0.ln() / (2.0_f64.ln());

            return half_life_coeff * half_life;

    }
}

