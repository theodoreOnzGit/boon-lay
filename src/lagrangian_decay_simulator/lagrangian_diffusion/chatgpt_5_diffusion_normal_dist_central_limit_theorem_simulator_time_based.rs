use rand::prelude::*;
use rand_distr::StandardNormal;

/// Diffusion coefficient for isotropic scattering: D = v * lambda / 3.
fn diffusion_coefficient(v: f64, lambda: f64) -> f64 {
    v * lambda / 3.0
}

/// Per-component variance for Gaussian displacement over time t: sigma2 = 2 D t.
fn per_component_variance_time(d: f64, t: f64) -> f64 {
    2.0 * d * t
}

/// Sample a unit direction uniformly on S^2 (isotropic).
fn sample_unit_vector<R: Rng + ?Sized>(rng: &mut R) -> [f64; 3] {
    let u: f64 = rng.gen_range(-1.0..=1.0); // cos(theta)
    let phi: f64 = rng.gen_range(0.0..(2.0 * std::f64::consts::PI));
    let rxy = (1.0 - u * u).sqrt();
    [rxy * phi.cos(), rxy * phi.sin(), u]
}

/// Sample a 3D Gaussian displacement vector X ~ N(0, sigma2 * I3).
fn sample_gaussian_vector<R: Rng + ?Sized>(rng: &mut R, sigma2: f64) -> [f64; 3] {
    let s = sigma2.sqrt();
    let x: f64 = rng.sample(StandardNormal);
    let y: f64 = rng.sample(StandardNormal);
    let z: f64 = rng.sample(StandardNormal);
    [s * x, s * y, s * z]
}

/// Sample net distance R for a 3D Gaussian displacement using Maxwell distribution with scale sigma.
/// Equivalent to ||N(0, sigma^2 I3)||.
fn sample_maxwell_radius<R: Rng + ?Sized>(rng: &mut R, sigma: f64) -> f64 {
    let x: f64 = rng.sample(StandardNormal);
    let y: f64 = rng.sample(StandardNormal);
    let z: f64 = rng.sample(StandardNormal);
    sigma * (x * x + y * y + z * z).sqrt()
}

/// Sample the net displacement over a single time horizon t, given speed v and mean free path lambda.
/// Returns both vector and (distance, direction) forms.
fn sample_displacement_over_time<R: Rng + ?Sized>(
    rng: &mut R,
    v: f64,
    lambda: f64,
    t: f64,
) -> ([f64; 3], (f64, [f64; 3])) {
    let d = diffusion_coefficient(v, lambda);
    let sigma2 = per_component_variance_time(d, t);

    // Vector method
    let x = sample_gaussian_vector(rng, sigma2);

    // Distance-direction method
    let sigma = sigma2.sqrt();
    let r = sample_maxwell_radius(rng, sigma);
    let u = sample_unit_vector(rng);

    (x, (r, u))
}

/// Generate a trajectory on a time grid using Gaussian increments with variance 2 D Δt per component.
/// - times: monotonically increasing times t0 < t1 < ... < tK
/// - start: initial position at t0
/// - v, lambda: speed and mean free path determining D
/// Returns positions at each time in `times` (same length).
fn simulate_trajectory_time_grid<R: Rng + ?Sized>(
    rng: &mut R,
    times: &[f64],
    start: [f64; 3],
    v: f64,
    lambda: f64,
) -> Result<Vec<[f64; 3]>, String> {
    if times.is_empty() {
        return Err("times must be non-empty".into());
    }
    // Validate monotonicity
    for w in times.windows(2) {
        if w[1] < w[0] {
            return Err("times must be monotonically non-decreasing".into());
        }
    }

    let d = diffusion_coefficient(v, lambda);
    let mut positions = Vec::with_capacity(times.len());
    let mut pos = start;
    positions.push(pos);

    for k in 1..times.len() {
        let dt = times[k] - times[k - 1];
        if dt < 0.0 {
            return Err("time grid must be non-decreasing".into());
        }
        let sigma2 = per_component_variance_time(d, dt);
        let inc = sample_gaussian_vector(rng, sigma2);
        pos = [pos[0] + inc[0], pos[1] + inc[1], pos[2] + inc[2]];
        positions.push(pos);
    }

    Ok(positions)
}

#[test]
fn isotropic_scattering_time_summation() -> Result<(), String> {
    // Example parameters
    let v = 1.0;        // speed (units: length/time)
    let lambda = 1.5;   // mean free path (units: length)
    let t = 10.0;       // total time horizon

    let mut rng = StdRng::seed_from_u64(42);

    // Single-horizon displacement sample (vector and distance-direction)
    let (x_vec, (r, u)) = sample_displacement_over_time(&mut rng, v, lambda, t);
    println!("Single-horizon sample over t = {:.3}:", t);
    println!("  Vector displacement: [{:.4}, {:.4}, {:.4}]", x_vec[0], x_vec[1], x_vec[2]);
    println!("  Distance-direction: R = {:.4}, u = [{:.4}, {:.4}, {:.4}]", r, u[0], u[1], u[2]);
    let x_recon = [r * u[0], r * u[1], r * u[2]];
    println!("  Recon vector from (R,u): [{:.4}, {:.4}, {:.4}]", x_recon[0], x_recon[1], x_recon[2]);

    // Trajectory over a time grid
    let start = [0.0, 0.0, 0.0];
    let times = vec![0.0, 2.0, 4.0, 6.0, 8.0, 10.0];
    let positions = simulate_trajectory_time_grid(&mut rng, &times, start, v, lambda)?;

    println!("\nTrajectory positions at specified times:");
    for (k, p) in positions.iter().enumerate() {
        println!("  t = {:>4.1}: [{:.4}, {:.4}, {:.4}]", times[k], p[0], p[1], p[2]);
    }

    // Optional: simulate multiple endpoints for statistics
    let num_samples = 10000;
    let d = diffusion_coefficient(v, lambda);
    let sigma2_total = per_component_variance_time(d, t);
    let mut sum = [0.0; 3];
    for _ in 0..num_samples {
        let x = sample_gaussian_vector(&mut rng, sigma2_total);
        sum[0] += x[0];
        sum[1] += x[1];
        sum[2] += x[2];
    }
    println!("\nSample mean of endpoints over {} runs (should be ~0): [{:.4}, {:.4}, {:.4}]",
             num_samples, sum[0] / num_samples as f64, sum[1] / num_samples as f64, sum[2] / num_samples as f64);

    Ok(())
}
