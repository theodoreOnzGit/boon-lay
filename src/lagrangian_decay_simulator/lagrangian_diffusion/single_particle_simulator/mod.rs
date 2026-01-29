use rand::RngCore;
use uom::{si::f64::*, ConstZero};

use crate::lagrangian_decay_simulator::lagrangian_diffusion::central_limit_theorem::oorandom_rng::OoRng64;

#[derive(Debug,Clone,Copy,PartialEq)]
pub struct SingleParticleDiffusionSimulatorMC {
    pub coordinates: (Length, Length, Length),
    /// random number generator
    pub rng: OoRng64,
}

impl SingleParticleDiffusionSimulatorMC {

    pub fn new_from_rng(outside_rng: &mut OoRng64) -> Self {

        let zero_length = Length::ZERO;
        let coordinates = (zero_length, zero_length, zero_length);

        // I will seed a new rng
        let rng = OoRng64::from_u64(outside_rng.next_u64());

        return Self { 
            coordinates, 
            rng,
        }

    }

}
