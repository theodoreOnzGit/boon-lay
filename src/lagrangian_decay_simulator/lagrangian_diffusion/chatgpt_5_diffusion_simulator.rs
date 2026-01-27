// Cargo.toml dependencies:
// [dependencies]
// rand = "0.8"
// rand_distr = "0.4"
// csv = "1.3"      # optional, used if you enable CSV output

use rand::Rng;
use rand::thread_rng;
use rand_distr::{Distribution, Exp};
use std::f64::consts::PI;

#[derive(Debug, Clone, Copy)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    fn add(&self, other: Vec3) -> Vec3 {
        Vec3 { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
    }

    fn scale(&self, s: f64) -> Vec3 {
        Vec3 { x: self.x * s, y: self.y * s, z: self.z * s }
    }

    fn norm(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    fn normalize(&self) -> Vec3 {
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
fn sample_isotropic_direction<R: Rng>(rng: &mut R) -> Vec3 {
    let mu :f64 = rng.gen_range(-1.0..=1.0);
    let phi = rng.gen_range(0.0..(2.0 * PI));
    let sin_theta: f64 = (1.0_f64 - mu * mu).sqrt();
    Vec3 {
        x: sin_theta * phi.cos(),
        y: sin_theta * phi.sin(),
        z: mu,
    }
}

/// Sample a free path length ℓ from exponential distribution with rate Σ_s:
/// p(ℓ) = Σ_s e^{-Σ_s ℓ}, ℓ ≥ 0
fn sample_free_path<R: Rng>(rng: &mut R, sigma_s: f64) -> f64 {
    let exp = Exp::new(sigma_s).expect("Σ_s must be > 0");
    exp.sample(rng)
}

#[derive(Debug)]
struct Particle {
    pos: Vec3,
    dir: Vec3,
}

impl Particle {
    fn new(initial_pos: Vec3, initial_dir: Option<Vec3>) -> Self {
        let mut rng = thread_rng();
        let dir = match initial_dir {
            Some(d) => d.normalize(),
            None => sample_isotropic_direction(&mut rng),
        };
        Particle { pos: initial_pos, dir }
    }
}

/// Run a simple collision-driven random walk:
/// - Start at (x0, y0, z0)
/// - For each collision:
///   1) Sample free path ℓ ~ Exp(Σ_s)
///   2) Move pos += dir * ℓ
///   3) Sample new isotropic dir
fn simulate(
    sigma_s: f64,
    n_collisions: usize,
    initial_pos: Vec3,
    initial_dir: Option<Vec3>,
) -> Vec<Vec3> {
    let mut rng = thread_rng();
    let mut particle = Particle::new(initial_pos, initial_dir);
    let mut trajectory = Vec::with_capacity(n_collisions + 1);

    // Store initial position
    trajectory.push(particle.pos);

    for _ in 0..n_collisions {
        // Step 1: sample free path
        let ell = sample_free_path(&mut rng, sigma_s);

        // Step 2: move along current direction
        particle.pos = particle.pos.add(particle.dir.scale(ell));
        trajectory.push(particle.pos);

        // Step 3: sample new isotropic direction
        particle.dir = sample_isotropic_direction(&mut rng);
    }

    trajectory
}

#[test]
fn chat_gpt_sim() {
    // Example parameters
    // Σ_s (macroscopic scattering cross section) in 1/length units (e.g., cm^-1)
    // D = 1 / (3 Σ_s) for isotropic scattering; not directly used in the walk.
    let sigma_s = 0.5_f64; // adjust as needed
    let n_collisions = 1000;

    // Initial conditions
    let initial_pos = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
    let initial_dir = None; // None => start with an isotropic direction

    let trajectory = simulate(sigma_s, n_collisions, initial_pos, initial_dir);

    // Print a few positions to stdout
    for (i, p) in trajectory.iter().enumerate().take(10) {
        println!("step {:4}: ({:.6}, {:.6}, {:.6})", i, p.x, p.y, p.z);
    }

    // Optional: write full trajectory to CSV
    // Uncomment to enable
    //let mut wtr = csv::Writer::from_path("trajectory.csv").expect("cannot create CSV");
    //wtr.write_record(&["step", "x", "y", "z"]).unwrap();
    //for (i, p) in trajectory.iter().enumerate() {
    //    wtr.write_record(&[i.to_string(), p.x.to_string(), p.y.to_string(), p.z.to_string()]).unwrap();
    //}
    //wtr.flush().unwrap();
    //println!("Wrote trajectory.csv");
}
