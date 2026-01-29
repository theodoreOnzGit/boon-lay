use rand::Rng;
use rand_distr::StandardNormal;
use uom::si::f64::*;


/// Compute per-component variance sigma2 for the Gaussian displacement after n isotropic steps.
/// General case: sigma2 = n * E[S^2] / 3.
/// For exponential step lengths with mean lambda, E[S^2] = 2 lambda^2 ⇒ sigma2 = n * 2 lambda^2 / 3.
///
pub fn per_component_variance_from_second_moment(
    no_of_collisions: u64, e_s2: Area
) -> Area {
    (no_of_collisions as f64) * e_s2 / 3.0
}

/// this obtains the variance given n random collisions 
/// and a mean free path length
///
/// denoted as lambda
///
/// this is meant for 3d vector
pub fn per_component_variance_exponential_for_3d_vector(no_of_collisions: u64, lambda: Length) -> Area {
    let e_s2: Area = 2.0 * lambda * lambda;
    per_component_variance_from_second_moment(no_of_collisions, e_s2)
}

/// Sample a 3D Gaussian displacement vector X ~ N(0, sigma2 * I3).
pub fn sample_dimensioned_gaussian_vector<R: Rng + ?Sized>(rng: &mut R, variance: Area) -> [Length; 3] {
    let std_deviation = variance.sqrt();
    let x: f64 = rng.sample(StandardNormal);
    let y: f64 = rng.sample(StandardNormal);
    let z: f64 = rng.sample(StandardNormal);
    [std_deviation * x, std_deviation * y, std_deviation * z]
}


/// this is a local type
/// that implements Rng for oorandom
mod oorandom_rng;

