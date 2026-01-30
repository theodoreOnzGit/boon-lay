use rand::prelude::*;
use std::f64::consts::PI;

/// Returns (x, y, z) uniformly distributed in the volume of the spherical shell
/// between radii r_in and r_out (0 <= r_in < r_out).
fn random_point_in_spherical_shell<R: Rng + ?Sized>(
    r_in: f64,
    r_out: f64,
    rng: &mut R,
) -> (f64, f64, f64) {
    assert!(r_in >= 0.0 && r_in < r_out, "Require 0 <= r_in < r_out");

    // Sample radius with correct volume weighting: r ~ proportional to r^2
    let u: f64 = rng.r#gen(); // U in [0,1)
    let r_in3 = r_in * r_in * r_in;
    let r_out3 = r_out * r_out * r_out;
    let rho = (u * (r_out3 - r_in3) + r_in3).cbrt();

    // Sample direction uniformly on the sphere.
    // z uniform in [-1,1], phi uniform in [0, 2π)
    let z: f64 = rng.gen_range(-1.0..=1.0);
    let phi: f64 = rng.gen_range(0.0..(2.0 * PI));
    let t = (1.0 - z * z).max(0.0).sqrt(); // sin(theta)
    let x = t * phi.cos();
    let y = t * phi.sin();

    (rho * x, rho * y, rho * z)
}

#[test]
fn rng_test() {
    let mut rng = rand::thread_rng();

    // Example usage: full sphere of radius r
    let r = 1.0;
    let r_in = 0.0;
    let r_out = r;

    for _ in 0..5 {
        let (x, y, z) = random_point_in_spherical_shell(r_in, r_out, &mut rng);
        println!("{x:.6}, {y:.6}, {z:.6}");
    }

    // Example usage: thin shell between 0.6*r and r
    let r_in2 = 0.6 * r;
    let r_out2 = r;
    for _ in 0..5 {
        let (x, y, z) = random_point_in_spherical_shell(r_in2, r_out2, &mut rng);
        println!("shell: {x:.6}, {y:.6}, {z:.6}");
    }
}