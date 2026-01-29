use rand::prelude::*;
use rand_distr::StandardNormal;

/// Compute per-component variance sigma2 for the Gaussian displacement after n isotropic steps.
/// General case: sigma2 = n * E[S^2] / 3.
/// For exponential step lengths with mean lambda, E[S^2] = 2 lambda^2 ⇒ sigma2 = n * 2 lambda^2 / 3.
fn per_component_variance_from_m2(n: usize, e_s2: f64) -> f64 {
    (n as f64) * e_s2 / 3.0
}

fn per_component_variance_exponential(n: usize, lambda: f64) -> f64 {
    let e_s2 = 2.0 * lambda * lambda;
    per_component_variance_from_m2(n, e_s2)
}

/// Sample a 3D Gaussian displacement vector X ~ N(0, sigma2 * I3).
fn sample_gaussian_vector<R: Rng + ?Sized>(rng: &mut R, sigma2: f64) -> [f64; 3] {
    let s = sigma2.sqrt();
    let x: f64 = rng.sample(StandardNormal);
    let y: f64 = rng.sample(StandardNormal);
    let z: f64 = rng.sample(StandardNormal);
    [s * x, s * y, s * z]
}

/// Sample a unit direction uniformly on S^2.
fn sample_unit_vector<R: Rng + ?Sized>(rng: &mut R) -> [f64; 3] {
    let u: f64 = rng.gen_range(-1.0..=1.0); // cos(theta)
    let phi: f64 = rng.gen_range(0.0..(2.0 * std::f64::consts::PI));
    let rxy = (1.0 - u * u).sqrt();
    [rxy * phi.cos(), rxy * phi.sin(), u]
}

/// Sample net distance R for the Gaussian displacement using the Maxwell distribution with scale sigma.
/// Equivalent to ||N(0, sigma^2 I3)||.
fn sample_maxwell_radius<R: Rng + ?Sized>(rng: &mut R, sigma: f64) -> f64 {
    // Efficient and numerically stable: norm of 3 standard normals times sigma.
    let x: f64 = rng.sample(StandardNormal);
    let y: f64 = rng.sample(StandardNormal);
    let z: f64 = rng.sample(StandardNormal);
    sigma * (x * x + y * y + z * z).sqrt()
}

/// Sample (distance, direction) for the Gaussian net displacement after n isotropic steps.
/// You can pass sigma2 directly (per-component variance) or compute it from n and E[S^2].
fn sample_distance_and_direction<R: Rng + ?Sized>(rng: &mut R, sigma2: f64) -> (f64, [f64; 3]) {
    let sigma = sigma2.sqrt();
    // Option A: independent sampling: R from Maxwell(sigma), u isotropic.
    // This yields exactly the same law as the vector method.
    let r = sample_maxwell_radius(rng, sigma);
    let u = sample_unit_vector(rng);
    (r, u)
}

#[test]
/// Produce a collection of Gaussian displacement samples (either as vectors,
/// or as distance-direction pairs) for given n and either E[S^2] or lambda.
fn diffusion_gaussian_sum() {
    let mut rng = StdRng::seed_from_u64(42);

    // Example settings:
    let n = 1000usize;

    // Case A: exponential step lengths with mean free path lambda
    let lambda = 1.0;
    let sigma2 = per_component_variance_exponential(n, lambda);

    // Case B (alternative): specify E[S^2] directly
    // let e_s2 = 0.5; // example
    // let sigma2 = per_component_variance_from_m2(n, e_s2);

    // Sample 5 Gaussian displacement vectors
    println!("Gaussian vectors X ~ N(0, sigma2 I3), with sigma2 = {:.6}", sigma2);
    for i in 0..5 {
        let x = sample_gaussian_vector(&mut rng, sigma2);
        println!("vec #{i}: [{:.4}, {:.4}, {:.4}]", x[0], x[1], x[2]);
    }

    // Sample 5 (distance, direction) pairs and reconstruct vectors
    println!("\nDistance–direction samples (Maxwell distance, isotropic direction):");
    for i in 0..5 {
        let (r, u) = sample_distance_and_direction(&mut rng, sigma2);
        let x = [r * u[0], r * u[1], r * u[2]];
        println!("pair #{i}: R = {:.4}, u = [{:.4}, {:.4}, {:.4}], X = [{:.4}, {:.4}, {:.4}]",
                 r, u[0], u[1], u[2], x[0], x[1], x[2]);
    }
}
