use uom::si::diffusion_coefficient::square_meter_per_second;
use uom::si::f64::*;

use crate::lagrangian_decay_simulator::lagrangian_diffusion::single_particle_simulator::constructive_solid_geometry::{TrisoCell, TrisoRegion};
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

        self.move_particle_gaussian_sampling_u64(mean_free_path, no_of_collisions);

        single_particle_sim.position = self.position;
    }

    /// moves the particle in the SingleNuclideSimulatorMC 
    /// in a Gaussian direction
    /// providing the mean free path and number of collisions 
    ///
    /// I want to have it done through 100 collisions rather than
    /// one single collision in every timestep
    ///
    pub fn move_single_decaying_particle_within_triso(
        &mut self,
        single_particle_sim: &mut SingleNuclideSimulatorMC,
        triso_cell: TrisoCell,
        timestep: Time,
    ){

        self.position = single_particle_sim.position;
        let nuclide = single_particle_sim.get_current_nuclide();
        // I would like to scale timestep appropriately, based on diffusion 
        // coeff and lengthscales 
        //
        // it would seem that diff_coeff = m^2/s
        // then timestep is  delta_t = diff_coeff/(length_scale of layer)
        // let's try that

        // first get diffusion coeff
        let (x, y, z) = self.position;
        let pos: [Length; 3] = [x, y, z];
        let diffusion_coeff = triso_cell
            .try_get_diffusion_coefficient(pos, nuclide)
            .unwrap_or_else(|| DiffusionCoefficient::new::<square_meter_per_second>(1e-6));

        // next get lengthscale
        let region = triso_cell.get_triso_region(pos);

        let lengthscale = match region {
            TrisoRegion::Fuel => triso_cell.get_fuel_radius(),
            TrisoRegion::Buffer => triso_cell.get_buffer_radius() - triso_cell.get_fuel_radius(),
            TrisoRegion::IPyC => triso_cell.get_ipyc_radius() - triso_cell.get_buffer_radius(),
            TrisoRegion::SiC => triso_cell.get_sic_radius() - triso_cell.get_ipyc_radius(),
            TrisoRegion::OPyC => triso_cell.get_opyc_radius() - triso_cell.get_sic_radius(),

            // For the 'Outside' region, there is no containing shell. A reasonable
            // default is the radius of the entire particle, representing the boundary
            // that was just crossed. Another option could be Length::ZERO if this
            // state should be handled specially, but using the particle radius is safer
            // to avoid potential division-by-zero errors later.
            TrisoRegion::Outside => triso_cell.get_opyc_radius(),
        };

        


        self.scatter_within_triso_particle_gaussian(triso_cell, nuclide, timestep);

        single_particle_sim.position = self.position;
    }

}
