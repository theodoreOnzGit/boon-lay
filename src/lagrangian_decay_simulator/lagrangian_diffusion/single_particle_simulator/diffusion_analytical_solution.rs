// Add these imports to your project's `Cargo.toml` if not already present:
// [dependencies]
// uom = { version = "0.35", features = ["f64", "si"] }

// Add these `use` statements at the top of your relevant file (e.g., a new verification module)
use uom::si::f64::{DiffusionCoefficient, Length, Time};
use uom::si::diffusion_coefficient::square_meter_per_second;
use uom::si::length::meter;
use uom::si::time::second;
use std::f64::consts::PI;

/// Calculates the analytical fraction of material released from a sphere over time.
///
/// This solution is for diffusion from a sphere of radius `radius`
/// with a constant diffusion coefficient `diffusion_coefficient`,
/// assuming a uniform initial concentration within the sphere
/// and a perfect sink (zero concentration) at the surface.
///
/// # Arguments
/// * `diffusion_coefficient` - The constant diffusion coefficient (e.g., in m²/s).
/// * `radius` - The radius of the sphere (e.g., in m).
/// * `time` - The elapsed time (e.g., in s).
/// * `num_terms` - The number of terms to use in the infinite series summation.
///                 More terms provide higher accuracy, but 10-20 are usually sufficient.
///
/// # Returns
/// A `f64` representing the fraction of material released (between 0.0 and 1.0).
///
/// # Panics
/// Panics if `radius` is zero or `time` is negative.
pub fn calculate_analytical_fraction_released(
    diffusion_coefficient: DiffusionCoefficient,
    radius: Length,
    time: Time,
    num_terms: usize,
) -> f64 {
    // Extract raw f64 values with consistent units
    let d_val = diffusion_coefficient.get::<square_meter_per_second>();
    let r_val = radius.get::<meter>();
    let t_val = time.get::<second>();

    // Basic validation
    if r_val <= 0.0 {
        panic!("Radius must be positive for analytical solution.");
    }
    if t_val < 0.0 {
        panic!("Time cannot be negative for analytical solution.");
    }
    if d_val < 0.0 {
        panic!("Diffusion coefficient cannot be negative.");
    }

    // Handle t=0 case explicitly to avoid exp(0) issues with very small D*t/R^2
    if t_val == 0.0 {
        return 0.0; // No release at t=0
    }

    let mut sum_terms = 0.0;

    // Sum the infinite series for fraction remaining
    for n in 1..=num_terms {
        let n_f64 = n as f64;
        let term_exponent = -d_val * n_f64.powi(2) * PI.powi(2) * t_val / r_val.powi(2);
        let term_coefficient = 6.0 / (n_f64.powi(2) * PI.powi(2));
        sum_terms += term_coefficient * term_exponent.exp();
    }

    let fraction_remaining = sum_terms;

    // The fraction released is 1 - fraction_remaining
    1.0 - fraction_remaining
}

// Example usage (you can put this in a test or a temporary main function)
#[cfg(test)]
mod tests {
    use super::*;
    use uom::si::diffusion_coefficient::square_meter_per_second;
    use uom::si::length::millimeter;
    use uom::si::time::hour;

    #[test]
    fn test_analytical_fraction_released() {
        let d = DiffusionCoefficient::new::<square_meter_per_second>(1.0e-12); // 1e-12 m²/s
        let r = Length::new::<millimeter>(250.0); // 250 mm = 0.25 m
        let t = Time::new::<hour>(1000.0); // 1000 hours

        // For this specific set of parameters, let's calculate the expected value
        // You'd typically compare against a known value from a reference or another tool.
        // For demonstration, let's pick a time where some release has occurred.
        let fraction_released = calculate_analytical_fraction_released(d, r, t, 100); // Use 100 terms for good accuracy

        println!("Diffusion Coefficient: {:?}", d);
        println!("Radius: {:?}", r);
        println!("Time: {:?}", t);
        println!("Analytical Fraction Released: {}", fraction_released);

        // Assert that the fraction is within a reasonable range (0 to 1)
        assert!(fraction_released >= 0.0);
        assert!(fraction_released <= 1.0);

        // Add more specific assertions if you have known values for D, R, T
        // For example, if you know at a certain time, the release should be ~0.5:
        // assert!((fraction_released - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_analytical_fraction_released_at_t_zero() {
        let d = DiffusionCoefficient::new::<square_meter_per_second>(1.0e-12);
        let r = Length::new::<millimeter>(250.0);
        let t = Time::new::<second>(0.0);
        let fraction_released = calculate_analytical_fraction_released(d, r, t, 10);
        assert_eq!(fraction_released, 0.0);
    }

    #[test]
    #[should_panic(expected = "Radius must be positive for analytical solution.")]
    fn test_analytical_fraction_released_zero_radius_panics() {
        let d = DiffusionCoefficient::new::<square_meter_per_second>(1.0e-12);
        let r = Length::new::<meter>(0.0);
        let t = Time::new::<second>(10.0);
        calculate_analytical_fraction_released(d, r, t, 10);
    }
}
