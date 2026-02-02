use std::f64::consts::PI;

use uom::si::f64::{
    Energy, Length, Mass, ThermodynamicTemperature, Time, Velocity,
};
use uom::si::energy::joule;
use uom::si::ratio::ratio;
use uom::si::thermodynamic_temperature::kelvin;

/// Boltzmann constant k_B in SI (J/K).
/// Define as Energy per Temperature so we can multiply by T to get Energy.
pub fn boltzmann_constant() -> Energy {
    // Represent k_B as Energy per Kelvin by taking 1 K in the denominator.
    // k_B = 1.380649e-23 J/K.
    Energy::new::<joule>(1.380_649e-23)
}

/// Mean speed (Maxwell–Boltzmann) at temperature T for a particle of mass m:
/// v_mean = sqrt(8 k_B T / (pi m))
pub fn mean_speed(medium_temperature: ThermodynamicTemperature, particle_mass: Mass) -> Velocity {
    // k_B * T has dimension of energy
    let k_b_t: Energy = boltzmann_constant() * (medium_temperature / ThermodynamicTemperature::new::<kelvin>(1.0));
    // specific energy (m^2/s^2)
    let specific = (8.0 * k_b_t) / (PI * particle_mass);
    // sqrt to get velocity
    specific.sqrt()
}

/// Expected number of collisions in time t with mean free path ℓ:
/// E[N(t)] = (t / ℓ) * E[v]
/// Returns a dimensionless count (f64).
///
///
/// now this is not quite atomic jumps as chatGPT suggested,
/// D = 1/6 a^2 * nu
///
/// However, atomic jumps assume diffusion is only within monocrystalline 
/// material without defects. 
///
/// In reality, there are defects, grain boundaries, dislocations etc.
/// Therefore, we need an effective diffusion coefficient to consider 
/// this
pub fn expected_collisions_atomic_jumps(
    medium_temperature: ThermodynamicTemperature,
    particle_mass: Mass,
    mean_free_path: Length,
    t: Time,
) -> f64 {
    let v_mean = mean_speed(medium_temperature, particle_mass);
    // Compute in scalar form to avoid needing reciprocal-velocity quantity:
    // (t/ell) has units s/m, v_mean has m/s -> dimensionless.

    return (t/mean_free_path * v_mean).get::<ratio>();
}



/// boltzmann collision test
#[test]
fn boltzmann_test() {
    use uom::si::mass::kilogram;
    use uom::si::time::second;
    use uom::si::velocity::meter_per_second;
    use uom::si::length::meter;
    // Example: nitrogen molecule at room temperature
    // Temperature T = 300 K
    let room_temp = ThermodynamicTemperature::new::<kelvin>(300.0);

    // Mass m: take N2 with molar mass ~28 g/mol -> per molecule m = 28e-3 kg / N_A
    // For demonstration, use m ≈ 4.65e-26 kg (approx for N2).
    let m = Mass::new::<kilogram>(4.65e-26);

    // Mean free path ℓ (example): 70 nm in air at STP (order of magnitude), or any value you need.
    let ell = Length::new::<meter>(70e-9);

    // Time horizon t: 1 microsecond
    let t = Time::new::<second>(1e-6);

    let v_mean = mean_speed(room_temp, m);
    let n_expected = expected_collisions_atomic_jumps(room_temp, m, ell, t);

    println!("Inputs:");
    println!("  T = {:.3} K", room_temp.get::<kelvin>());
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
