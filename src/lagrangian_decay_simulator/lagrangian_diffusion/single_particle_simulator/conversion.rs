use uom::si::f64::*;

use crate::prelude::SingleNuclideSimulatorMC;
use crate::lagrangian_decay_simulator::lagrangian_diffusion::single_particle_simulator::SingleParticleDiffusionSimulatorMC;

impl SingleParticleDiffusionSimulatorMC {

    /// moves the particle in the SingleNuclideSimulatorMC 
    /// in a random walk direction 
    /// you'll need to define a linear number density (ie 
    /// macroscopic cross section)
    pub fn move_single_decaying_particle_isotropically(
        &mut self,
        single_particle_sim: &mut SingleNuclideSimulatorMC,
        sigma_s: LinearNumberDensity){

        self.position = single_particle_sim.position;

        self.scatter_isotropically_using_macro_xs(sigma_s);

        single_particle_sim.position = self.position;

    }

    /// moves the particle in the SingleNuclideSimulatorMC 
    /// in a Gaussian direction
    /// providing the mean free path and number of collisions 
    pub fn move_single_decaying_particle_gaussian_mfp_and_no_of_collisions(
        &mut self,
        single_particle_sim: &mut SingleNuclideSimulatorMC,
        mean_free_path: Length,
        no_of_collisions: u64,
    ){

        self.position = single_particle_sim.position;

        self.move_particle_gaussian_sampling(mean_free_path, no_of_collisions);

        single_particle_sim.position = self.position;
    }

}
