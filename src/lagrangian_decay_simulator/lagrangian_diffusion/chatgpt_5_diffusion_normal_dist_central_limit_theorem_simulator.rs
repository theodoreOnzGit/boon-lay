use rand::prelude::*;
use rand_distr::StandardNormal;

/// Compute the Cholesky decomposition L of a symmetric positive-definite 3x3 matrix Sigma,
/// such that Sigma = L * L^T. Returns None if the matrix is not SPD (numerically).
fn cholesky_3x3(sigma: [[f64; 3]; 3]) -> Option<[[f64; 3]; 3]> {
    // Ensure symmetry (not strictly necessary if caller guarantees it)
    let s = sigma;

    // L lower-triangular
    let mut l = [[0.0f64; 3]; 3];

    // l00
    let mut v = s[0][0];
    if v <= 0.0 { return None; }
    l[0][0] = v.sqrt();

    // l10, l11
    l[1][0] = s[1][0] / l[0][0];
    v = s[1][1] - l[1][0] * l[1][0];
    if v <= 0.0 { return None; }
    l[1][1] = v.sqrt();

    // l20, l21, l22
    l[2][0] = s[2][0] / l[0][0];
    l[2][1] = (s[2][1] - l[2][0] * l[1][0]) / l[1][1];
    v = s[2][2] - l[2][0] * l[2][0] - l[2][1] * l[2][1];
    if v <= 0.0 { return None; }
    l[2][2] = v.sqrt();

    // Enforce lower-triangular zeros in the upper part
    l[0][1] = 0.0; l[0][2] = 0.0;
    l[1][2] = 0.0;

    Some(l)
}

/// Sample a 3D multivariate normal: mu + L z where z ~ N(0, I), L from Cholesky(Sigma)
fn sample_multivariate_normal<R: Rng + ?Sized>(
    rng: &mut R,
    mu: [f64; 3],
    l: [[f64; 3]; 3],
) -> [f64; 3] {
    let z0: f64 = rng.sample(StandardNormal);
    let z1: f64 = rng.sample(StandardNormal);
    let z2: f64 = rng.sample(StandardNormal);

    // y = L z
    let y0 = l[0][0] * z0;
    let y1 = l[1][0] * z0 + l[1][1] * z1;
    let y2 = l[2][0] * z0 + l[2][1] * z1 + l[2][2] * z2;

    [mu[0] + y0, mu[1] + y1, mu[2] + y2]
}

/// Simulate multiple 3D random walk paths.
/// - num_paths: number of Monte Carlo paths
/// - num_steps: steps per path
/// - start: starting position [x, y, z]
/// - mu: mean step vector
/// - sigma: 3x3 covariance matrix for step distribution
/// Returns Vec of paths; each path is Vec of positions length (num_steps + 1),
/// including the initial position.
fn simulate_random_walks(
    num_paths: usize,
    num_steps: usize,
    start: [f64; 3],
    mu: [f64; 3],
    sigma: [[f64; 3]; 3],
    seed: Option<u64>,
) -> Result<Vec<Vec<[f64; 3]>>, String> {
    let l = cholesky_3x3(sigma).ok_or_else(|| "Covariance matrix is not SPD (Cholesky failed)".to_string())?;

    let mut rng: StdRng = match seed {
        Some(s) => StdRng::seed_from_u64(s),
        None => StdRng::from_entropy(),
    };

    let mut paths = Vec::with_capacity(num_paths);
    for _ in 0..num_paths {
        let mut path = Vec::with_capacity(num_steps + 1);
        let mut pos = start;
        path.push(pos);

        for _ in 0..num_steps {
            let step = sample_multivariate_normal(&mut rng, mu, l);
            pos = [pos[0] + step[0], pos[1] + step[1], pos[2] + step[2]];
            path.push(pos);
        }

        paths.push(path);
    }

    Ok(paths)
}

#[test]
fn clt_norm_dist_random_walk() -> Result<(), String> {
    // Example parameters
    let num_paths = 10;
    let num_steps = 1000;
    let start = [0.0, 0.0, 0.0];

    // Mean step per time increment (e.g., drift)
    let mu = [0.0, 0.0, 0.0];

    // Symmetric positive-definite covariance for step distribution
    // Example: correlated axes
    let sigma = [
        [1.0, 0.3, 0.2],
        [0.3, 1.5, 0.4],
        [0.2, 0.4, 0.8],
    ];

    // Optional reproducibility
    let seed = Some(42);

    let paths = simulate_random_walks(num_paths, num_steps, start, mu, sigma, seed)?;

    // Example: print the final position of each path
    for (i, path) in paths.iter().enumerate() {
        let end = path.last().unwrap();
        println!("Path {i} end: [{:.4}, {:.4}, {:.4}]", end[0], end[1], end[2]);
    }

    // Example: compute sample mean of endpoints
    let mut sum = [0.0; 3];
    for path in &paths {
        let end = path.last().unwrap();
        sum[0] += end[0];
        sum[1] += end[1];
        sum[2] += end[2];
    }
    let n = paths.len() as f64;
    let mean_end = [sum[0] / n, sum[1] / n, sum[2] / n];
    println!(
        "Sample mean endpoint: [{:.4}, {:.4}, {:.4}]",
        mean_end[0], mean_end[1], mean_end[2]
    );

    //todo!();
    Ok(())
        
}
