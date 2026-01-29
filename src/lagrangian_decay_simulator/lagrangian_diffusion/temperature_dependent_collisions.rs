use std::f64::consts::PI;

use uom::si::f64::{
    Energy, Length, Mass, ThermodynamicTemperature, Time, Velocity,
};
use uom::si::energy::joule;
use uom::si::length::meter;
use uom::si::mass::kilogram;
use uom::si::thermodynamic_temperature::kelvin;
use uom::si::time::second;
use uom::si::velocity::meter_per_second;

/// Boltzmann constant k_B in SI (J/K).
/// Define as Energy per Temperature so we can multiply by T to get Energy.
fn boltzmann_constant() -> Energy {
    // Represent k_B as Energy per Kelvin by taking 1 K in the denominator.
    // k_B = 1.380649e-23 J/K.
    Energy::new::<joule>(1.380_649e-23)
}

/// Mean speed (Maxwell–Boltzmann) at temperature T for a particle of mass m:
/// v_mean = sqrt(8 k_B T / (pi m))
fn mean_speed(T: ThermodynamicTemperature, m: Mass) -> Velocity {
    // k_B * T has dimension of energy
    let k_b_T: Energy = boltzmann_constant() * (T / ThermodynamicTemperature::new::<kelvin>(1.0));
    // specific energy (m^2/s^2)
    let specific = (8.0 * k_b_T) / (PI * m);
    // sqrt to get velocity
    specific.sqrt()
}

/// Expected number of collisions in time t with mean free path ℓ:
/// E[N(t)] = (t / ℓ) * E[v]
/// Returns a dimensionless count (f64).
fn expected_collisions(
    T: ThermodynamicTemperature,
    m: Mass,
    ell: Length,
    t: Time,
) -> f64 {
    let v_mean = mean_speed(T, m);
    // Compute in scalar form to avoid needing reciprocal-velocity quantity:
    // (t/ell) has units s/m, v_mean has m/s -> dimensionless.
    (t.get::<second>() / ell.get::<meter>()) * v_mean.get::<meter_per_second>()
}



/// boltzmann collision test
#[test]
fn boltzmann_test() {
    // Example: nitrogen molecule at room temperature
    // Temperature T = 300 K
    let T = ThermodynamicTemperature::new::<kelvin>(300.0);

    // Mass m: take N2 with molar mass ~28 g/mol -> per molecule m = 28e-3 kg / N_A
    // For demonstration, use m ≈ 4.65e-26 kg (approx for N2).
    let m = Mass::new::<kilogram>(4.65e-26);

    // Mean free path ℓ (example): 70 nm in air at STP (order of magnitude), or any value you need.
    let ell = Length::new::<meter>(70e-9);

    // Time horizon t: 1 microsecond
    let t = Time::new::<second>(1e-6);

    let v_mean = mean_speed(T, m);
    let n_expected = expected_collisions(T, m, ell, t);

    println!("Inputs:");
    println!("  T = {:.3} K", T.get::<kelvin>());
    println!("  m = {:.3e} kg", m.get::<kilogram>());
    println!("  ℓ = {:.3e} m", ell.get::<meter>());
    println!("  t = {:.3e} s", t.get::<second>());

    println!("\nResults:");
    println!(
        "  Mean speed (Maxwell–Boltzmann): {:.3} m/s",
        v_mean.get::<meter_per_second>()
    );
    println!(
        "  Expected collisions in time t: {:.6} (dimensionless count)",
        n_expected
    );
}
