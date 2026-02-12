use crate::lagrangian_decay_simulator::lagrangian_diffusion::single_particle_simulator::constructive_solid_geometry::TrisoRegion;
use crate::lagrangian_decay_simulator::lagrangian_diffusion::single_particle_simulator::{constructive_solid_geometry::TrisoCell, SingleParticleDiffusionSimulatorMC};
use crate::prelude::SingleNuclideSimulatorMC;
use fission_yields_data::prelude::Nuclide;
use uom::si::f64::*;
use uom::si::diffusion_coefficient::square_meter_per_second;
use uom::si::length::angstrom;
use uom::si::ratio::ratio;
use uom::si::time::millisecond;
use uom::ConstZero;

impl SingleParticleDiffusionSimulatorMC {

    // this deals with movement within triso particles
    #[inline]
    pub fn scatter_within_triso_particle_gaussian(
        &mut self, 
        triso_cell: TrisoCell,
        nuclide: Nuclide,
        timestep: Time,
        ){

        // first find the diffusion coeff 
        let (x,y,z) = self.position;
        let initial_pos_array: [Length;3] = [x,y,z];

        let diffusion_coeff_option = 
            triso_cell.try_get_diffusion_coefficient(initial_pos_array, nuclide);

        let diffusion_coeff: DiffusionCoefficient = match diffusion_coeff_option {
            Some(coeff) => coeff,
            // the default diffusion coefficient is same as a cracked layer 
            // unless otherwise stated
            None => DiffusionCoefficient::new::<square_meter_per_second>(1e-6),
        };

        // now when having diffusion coeff, I need a mean free path and number 
        // of collisions
        // I'm going to use a jump distance of 2 angstroms 
        let jump_distance = Length::new::<angstrom>(2.0);

        // using D = 1/6 lambda^2 * nu 

        let collision_frequency: Frequency 
            = diffusion_coeff * 6.0 / (jump_distance * jump_distance);

        let mut remaining_timestep = timestep;

        while remaining_timestep > Time::ZERO {

            // let's get the velocity vector
            let velocity = self.get_gaussian_velocity_vector(
                jump_distance, 
                collision_frequency
            );

            let position = self.position;

            let time_opt: Option<Time> = triso_cell.get_time_to_sphere_boundary(
                position.into(), 
                velocity
            );

            let time_to_next_boundary = match time_opt {
                Some(time) => time,
                None => {
                    // if no time to next boundary, just 
                    // use the naive version of the code
                    return self.scatter_within_triso_particle_gaussian_simple(
                        triso_cell, 
                        nuclide, 
                        remaining_timestep);
                },
            };

            if time_to_next_boundary > remaining_timestep {

                // if time to next boundary is bigger than remaining timestep,
                // scatter using remaining timestep
                // break out of the loop

                break;

            };

            // if we get time_to_next_boundary, then use the simple scattering
            //
            self.scatter_within_triso_particle_gaussian_simple(
                triso_cell, 
                nuclide, 
                time_to_next_boundary
            );





            remaining_timestep -= time_to_next_boundary;
        };


        return self.scatter_within_triso_particle_gaussian_simple(
            triso_cell, 
            nuclide, 
            remaining_timestep
        );


    }


    /// this deals with movement within triso particles
    ///
    /// assuming there is no change in diffusion coefficient along the 
    /// travel path of the particle
    #[inline]
    pub fn scatter_within_triso_particle_gaussian_simple(
        &mut self, 
        triso_cell: TrisoCell,
        nuclide: Nuclide,
        timestep: Time,
        ){

        // first find the diffusion coeff 
        let (x,y,z) = self.position;
        let pos_array: [Length;3] = [x,y,z];

        let diffusion_coeff_option = 
            triso_cell.try_get_diffusion_coefficient(pos_array, nuclide);

        let diffusion_coeff: DiffusionCoefficient = match diffusion_coeff_option {
            Some(coeff) => coeff,
            // the default diffusion coefficient is same as a cracked layer 
            // unless otherwise stated
            None => DiffusionCoefficient::new::<square_meter_per_second>(1e-6),
        };

        // now when having diffusion coeff, I need a mean free path and number 
        // of collisions
        // I'm going to use a jump distance of 2 angstroms 
        let jump_distance = Length::new::<angstrom>(2.0);

        // using D = 1/6 lambda^2 * nu 

        let collision_frequency: Frequency 
            = diffusion_coeff * 6.0 / (jump_distance * jump_distance);

        let no_of_collisions_f64: f64 = (collision_frequency * timestep).get::<ratio>();
        let no_of_collisions: u64 = no_of_collisions_f64 as u64;


        self.move_particle_gaussian_sampling(jump_distance, no_of_collisions);


    }

    /// moves the particle in the SingleNuclideSimulatorMC 
    /// in a Gaussian direction
    /// within a triso particle
    pub fn move_single_decaying_particle_gaussian_triso_particle(
        &mut self,
        single_particle_sim: &mut SingleNuclideSimulatorMC,
        triso_cell: TrisoCell,
        timestep: Time,
    ){

        self.position = single_particle_sim.position;
        let nuclide = single_particle_sim.get_current_nuclide();

        self.scatter_within_triso_particle_gaussian(triso_cell, nuclide, timestep);

        single_particle_sim.position = self.position;
    }
}
