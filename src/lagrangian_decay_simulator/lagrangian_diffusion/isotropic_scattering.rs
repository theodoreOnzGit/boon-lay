use std::f64::consts::PI;

use rand::Rng;
use rand_distr::{Distribution, Exp};
use uom::si::{f64::Ratio, ratio::ratio};

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn add(&self, other: Vec3) -> Vec3 {
        Vec3 { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
    }

    pub fn scale(&self, s: f64) -> Vec3 {
        Vec3 { x: self.x * s, y: self.y * s, z: self.z * s }
    }

    pub fn norm(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&self) -> Vec3 {
        let n = self.norm();
        if n == 0.0 {
            // Avoid division by zero; return some default
            Vec3 { x: 1.0, y: 0.0, z: 0.0 }
        } else {
            self.scale(1.0 / n)
        }
    }
}

/// Sample an isotropic direction on the unit sphere.
/// Method: mu = cos(theta) ~ U[-1, 1], phi ~ U[0, 2π]
#[inline]
fn _sample_isotropic_direction<R: Rng>(rng: &mut R) -> Vec3 {
    let mu :f64 = rng.gen_range(-1.0..=1.0);
    let phi: f64 = rng.gen_range(0.0..(2.0 * PI));
    let sin_theta: f64 = (1.0_f64 - mu * mu).sqrt();
    Vec3 {
        x: sin_theta * phi.cos(),
        y: sin_theta * phi.sin(),
        z: mu,
    }
}

/// Sample an isotropic direction on the unit sphere.
/// Method: mu = cos(theta) ~ U[-1, 1], phi ~ U[0, 2π]
#[inline]
pub(crate) fn sample_isotropic_direction_into_array<R: Rng>(rng: &mut R) -> [Ratio; 3] {
    let mu :f64 = rng.gen_range(-1.0..=1.0);
    let phi: f64 = rng.gen_range(0.0..(2.0 * PI));
    let sin_theta: f64 = (1.0_f64 - mu * mu).sqrt();
    let vec = Vec3 {
        x: sin_theta * phi.cos(),
        y: sin_theta * phi.sin(),
        z: mu,
    };

    return [Ratio::new::<ratio>(vec.x),
    Ratio::new::<ratio>(vec.y),
    Ratio::new::<ratio>(vec.z),];

}

#[inline]
pub (crate) fn sample_free_path<R: Rng>(rng: &mut R, sigma_s: f64) -> f64 {
    let exp = Exp::new(sigma_s).expect("Σ_s must be > 0");
    exp.sample(rng)
}

