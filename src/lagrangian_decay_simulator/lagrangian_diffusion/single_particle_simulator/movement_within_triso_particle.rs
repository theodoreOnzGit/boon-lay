use crate::lagrangian_decay_simulator::lagrangian_diffusion::single_particle_simulator::{constructive_solid_geometry::TrisoCell, SingleParticleDiffusionSimulatorMC};
use crate::prelude::SingleNuclideSimulatorMC;
use fission_yields_data::prelude::Nuclide;
use uom::si::f64::*;
use uom::si::diffusion_coefficient::square_meter_per_second;
use uom::si::length::{angstrom, meter};
use uom::si::ratio::ratio;
use uom::ConstZero;
use uom::si::time::second;

impl SingleParticleDiffusionSimulatorMC {

    // this deals with movement within triso particles
    #[inline]
    pub fn scatter_within_triso_particle_gaussian(
        &mut self,
        triso_cell: TrisoCell,
        nuclide: Nuclide,
        timestep: Time,
    ) {
        // --- tolerances for (2) and (3) ---
        const T_EPS_S: f64 = 1e-15;
        let t_eps = Time::new::<second>(T_EPS_S);

        // nudge length: choose something tiny relative to geometry
        // (meter here; tune based on your smallest layer thickness)
        const R_EPS_M: f64 = 1e-12;
        let r_eps = Length::new::<meter>(R_EPS_M);

        #[inline]
        fn norm(p: [Length; 3]) -> Length {
            let x = p[0].get::<meter>();
            let y = p[1].get::<meter>();
            let z = p[2].get::<meter>();
            Length::new::<meter>((x * x + y * y + z * z).sqrt())
        }

        #[inline]
        fn unit_radial(p: [Length; 3]) -> [f64; 3] {
            // returns dimensionless unit vector
            let x = p[0].get::<meter>();
            let y = p[1].get::<meter>();
            let z = p[2].get::<meter>();
            let r = (x * x + y * y + z * z).sqrt().max(1e-30);
            [x / r, y / r, z / r]
        }

        let mut remaining_timestep = timestep;

        while remaining_timestep > Time::ZERO {
            // diffusion coeff at current position
            let (x, y, z) = self.position;
            let pos: [Length; 3] = [x, y, z];

            let diffusion_coeff = triso_cell
                .try_get_diffusion_coefficient(pos, nuclide)
                .unwrap_or_else(|| DiffusionCoefficient::new::<uom::si::diffusion_coefficient::square_meter_per_second>(1e-6));

            let jump_distance = Length::new::<uom::si::length::angstrom>(2.0);
            let collision_frequency: Frequency =
                diffusion_coeff * 6.0 / (jump_distance * jump_distance);

            let velocity = self.get_gaussian_velocity_vector(jump_distance, collision_frequency);

            let time_opt = triso_cell.get_time_to_sphere_boundary(pos, velocity);

            let Some(time_to_next_boundary) = time_opt else {
                // No boundary ahead in this direction; finish remaining time with this velocity
                let length_array: [Length; 3] = [
                    velocity[0] * remaining_timestep,
                    velocity[1] * remaining_timestep,
                    velocity[2] * remaining_timestep,
                ];
                self.move_particle_using_array(length_array);
                return;
            };

            // (2) Guard: if boundary time is tiny/invalid, don't loop forever.
            if time_to_next_boundary <= t_eps {
                // (3) Nudge across interface to escape boundary trap, then stop trying to "hit"
                // a boundary in zero time.
                let p_now = {
                    let (x, y, z) = self.position;
                    [x, y, z]
                };
                let n = unit_radial(p_now);

                // Decide nudge direction based on whether velocity points outward/inward
                let vdotn = velocity[0].get::<uom::si::velocity::meter_per_second>() * n[0]
                    + velocity[1].get::<uom::si::velocity::meter_per_second>() * n[1]
                    + velocity[2].get::<uom::si::velocity::meter_per_second>() * n[2];

                let s = if vdotn >= 0.0 { 1.0 } else { -1.0 };
                let nudge: [Length; 3] = [
                    Length::new::<meter>(s * r_eps.get::<meter>() * n[0]),
                    Length::new::<meter>(s * r_eps.get::<meter>() * n[1]),
                    Length::new::<meter>(s * r_eps.get::<meter>() * n[2]),
                ];
                self.move_particle_using_array(nudge);

                // After nudging, break out and do a final move for remaining time (prevents spin)
                break;
            }

            if time_to_next_boundary > remaining_timestep {
                break;
            }

            // Move exactly to boundary
            let length_array: [Length; 3] = [
                velocity[0] * time_to_next_boundary,
                velocity[1] * time_to_next_boundary,
                velocity[2] * time_to_next_boundary,
            ];
            self.move_particle_using_array(length_array);
            remaining_timestep -= time_to_next_boundary;

            // (3) Nudge across boundary so next iteration doesn't re-hit at t≈0
            let p_now = {
                let (x, y, z) = self.position;
                [x, y, z]
            };
            let n = unit_radial(p_now);
            let vdotn = velocity[0].get::<uom::si::velocity::meter_per_second>() * n[0]
                + velocity[1].get::<uom::si::velocity::meter_per_second>() * n[1]
                + velocity[2].get::<uom::si::velocity::meter_per_second>() * n[2];

            let s = if vdotn >= 0.0 { 1.0 } else { -1.0 };
            let nudge: [Length; 3] = [
                Length::new::<meter>(s * r_eps.get::<meter>() * n[0]),
                Length::new::<meter>(s * r_eps.get::<meter>() * n[1]),
                Length::new::<meter>(s * r_eps.get::<meter>() * n[2]),
            ];
            self.move_particle_using_array(nudge);
        }

        // Final move for whatever time remains
        if remaining_timestep > Time::ZERO {
            let (x, y, z) = self.position;
            let pos: [Length; 3] = [x, y, z];

            let diffusion_coeff = triso_cell
                .try_get_diffusion_coefficient(pos, nuclide)
                .unwrap_or_else(|| DiffusionCoefficient::new::<uom::si::diffusion_coefficient::square_meter_per_second>(1e-6));

            let jump_distance = Length::new::<uom::si::length::angstrom>(2.0);
            let collision_frequency: Frequency =
                diffusion_coeff * 6.0 / (jump_distance * jump_distance);

            let velocity = self.get_gaussian_velocity_vector(jump_distance, collision_frequency);

            let length_array: [Length; 3] = [
                velocity[0] * remaining_timestep,
                velocity[1] * remaining_timestep,
                velocity[2] * remaining_timestep,
            ];
            self.move_particle_using_array(length_array);
        }
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

        let no_of_collisions_f64: f64 
            = (collision_frequency * timestep).get::<ratio>();



        self.move_particle_gaussian_sampling_f64(jump_distance, 
            no_of_collisions_f64);


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



    // this deals with movement within triso particles
    //
    // This is for scattering within triso particles which is by brute 
    // force, that is one scatter at a time (I reckon this will be quite lengthy)
    #[inline]
    pub fn scatter_within_triso_particle_brute_force(
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

        let no_of_collisions_f64: f64 
            = (collision_frequency * timestep).get::<ratio>();



        self.move_particle_gaussian_sampling_f64(jump_distance, 
            no_of_collisions_f64);


    }
}
