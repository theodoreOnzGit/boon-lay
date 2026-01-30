use rand::RngCore;
use uom::{si::{f64::*, length::meter, linear_number_density::per_meter}, ConstZero};

use crate::lagrangian_decay_simulator::lagrangian_diffusion::{central_limit_theorem::{oorandom_rng::OoRng64, per_component_variance_exponential_for_3d_vector, sample_dimensioned_gaussian_vector}, isotropic_scattering::{sample_free_path, sample_isotropic_direction_into_array}};

#[derive(Debug,Clone,Copy,PartialEq)]
pub struct SingleParticleDiffusionSimulatorMC {
    pub position: (Length, Length, Length),
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
            position: coordinates, 
            rng,
        }

    }


    /// this moves the particle by an array
    pub fn move_particle_using_array(&mut self, length_array: [Length; 3]){
        let dx = length_array[0];
        let dy = length_array[1];
        let dz = length_array[2];

        let (x, y, z) = self.position;

        self.position = (x+dx, y+dy, z+dz);

    }

    /// this moves the particle by an tuple
    pub fn move_particle_using_tuple(&mut self, length_tuple: (Length, Length, Length)){
        let (dx, dy, dz) = length_tuple;

        let (x, y, z) = self.position;

        self.position = (x+dx, y+dy, z+dz);

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

    /// samples isotropic direction 
    pub fn sample_isotropic_direction(&mut self) -> [Ratio;3] {
        sample_isotropic_direction_into_array(&mut self.rng)
    }

    /// samples distance travelled given a mean free path
    /// given a macroscopic scattering cross section
    pub fn sample_mean_free_path_given_sigma_s(&mut self, sigma_s: LinearNumberDensity) 
        -> Length {

            let sigma_s_per_meter: f64 = sigma_s.get::<per_meter>();
            let distance_travelled_randomised_meters = 
                sample_free_path(&mut self.rng, sigma_s_per_meter);

            return Length::new::<meter>(distance_travelled_randomised_meters);

    }

    // if we have a scattering marcoscopic cross section,
    // we can scatter the particle isotropically
    #[inline] 
    pub fn scatter_isotropically_using_macro_xs(
        &mut self, 
        sigma_s: LinearNumberDensity,
    ){
        let [dx_unit, dy_unit, dz_unit] = self.sample_isotropic_direction();
        let randomly_sampled_length = self.sample_mean_free_path_given_sigma_s(sigma_s);

        let dx = dx_unit * randomly_sampled_length;
        let dy = dy_unit * randomly_sampled_length;
        let dz = dz_unit * randomly_sampled_length;

        let length_array = [dx,dy,dz];
        self.move_particle_using_array(length_array);

    }



    // now, for challenge with scattering is that we want to 
    // is that we want to have them precalculated.
    //
    // That isn't easy, but I'll probably do this another day
}

/// implements conversion and interaction with the 
/// SingleNuclideSimulatorMC
pub mod conversion;
