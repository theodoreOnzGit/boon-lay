use rand::RngCore;
use uom::{si::f64::*, ConstZero};

use crate::lagrangian_decay_simulator::lagrangian_diffusion::central_limit_theorem::{oorandom_rng::OoRng64, per_component_variance_exponential_for_3d_vector, sample_dimensioned_gaussian_vector};

#[derive(Debug,Clone,Copy,PartialEq)]
pub struct SingleParticleDiffusionSimulatorMC {
    pub coordinates: (Length, Length, Length),
    /// random number generator
    pub rng: OoRng64,
}

impl SingleParticleDiffusionSimulatorMC {

    /// constructor for new diffusion simulator
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


    /// this moves the particle by an array
    pub fn move_particle_using_array(&mut self, length_array: [Length; 3]){
        let dx = length_array[0];
        let dy = length_array[1];
        let dz = length_array[2];

        let (x, y, z) = self.coordinates;

        self.coordinates = (x+dx, y+dy, z+dz);

    }

    /// this moves the particle by an tuple
    pub fn move_particle_using_tuple(&mut self, length_tuple: (Length, Length, Length)){
        let (dx, dy, dz) = length_tuple;

        let (x, y, z) = self.coordinates;

        self.coordinates = (x+dx, y+dy, z+dz);

    }


    /// move particle assuming normal distribution 
    /// given mean free path and number of collisions 
    pub fn move_particle_gaussian_sampling(&mut self,
        mean_free_path: Length,
        no_of_collisions: u64){

        let per_component_variance = 
            per_component_variance_exponential_for_3d_vector(
                no_of_collisions, mean_free_path);

        let gaussian_length_array = 
            sample_dimensioned_gaussian_vector(
                &mut self.rng, 
                per_component_variance,
            );

        self.move_particle_using_array(gaussian_length_array);

    }

}
