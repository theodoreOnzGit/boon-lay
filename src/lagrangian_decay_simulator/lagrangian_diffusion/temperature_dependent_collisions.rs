use std::f64::consts::PI;

use fission_yields_data::prelude::Nuclide;
use uom::si::areal_number_density::per_square_meter;
use uom::si::diffusion_coefficient::square_meter_per_second;
use uom::si::f64::*;
use uom::si::energy::joule;
use uom::si::molar_energy::kilojoule_per_mole;
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


// diffusion coefficient 
// from Jiang 2023
// Jiang, W., Toptan, A., Hales, J. D., Spencer, B. W., & 
// Novascone, S. R. (2023). Fission product transport in TRISO particles 
// and pebbles (No. INL/EXT-21-63549-Rev001). Idaho National Lab.(INL), 
// Idaho Falls, ID (United States).
//
// D = D1 exp (-Q1/RT) + D2 exp (-Q2/RT)
pub fn diffusion_coeff_jiang(
    triso_layer: TrisoLayerMaterial,
    nuclide: Nuclide,
    temperature: ThermodynamicTemperature,
    ) -> Option<DiffusionCoefficient> {

    let (z,_a) = nuclide.get_z_a();

    let d1: Option<DiffusionCoefficient> = match z {
        // Silver 
        _ => None
    };

    todo!()

}

// from Jiang 2023
// Jiang, W., Toptan, A., Hales, J. D., Spencer, B. W., & 
// Novascone, S. R. (2023). Fission product transport in TRISO particles 
// and pebbles (No. INL/EXT-21-63549-Rev001). Idaho National Lab.(INL), 
// Idaho Falls, ID (United States).
//
// table on page 13 of 105
#[inline]
pub fn get_d1_for_ag(triso_layer: TrisoLayerMaterial,) -> DiffusionCoefficient{

    let coeff_m2_per_s: f64 = match triso_layer {
        TrisoLayerMaterial::Kernel => 6.7e-9,
        TrisoLayerMaterial::PyC => 5.3e-9,
        TrisoLayerMaterial::SiC => 3.6e-9,
    };

    return DiffusionCoefficient::new::<square_meter_per_second>(
        coeff_m2_per_s
    );


}

// from Jiang 2023
// Jiang, W., Toptan, A., Hales, J. D., Spencer, B. W., & 
// Novascone, S. R. (2023). Fission product transport in TRISO particles 
// and pebbles (No. INL/EXT-21-63549-Rev001). Idaho National Lab.(INL), 
// Idaho Falls, ID (United States).
//
// table on page 13 of 105
#[inline]
pub fn get_q1_for_ag(triso_layer: TrisoLayerMaterial,) -> MolarEnergy {

    let coeff_kj_per_mol: f64 = match triso_layer {
        TrisoLayerMaterial::Kernel => 165.0,
        TrisoLayerMaterial::PyC => 154.0,
        TrisoLayerMaterial::SiC => 215.0,
    };

    return MolarEnergy::new::<kilojoule_per_mole>(coeff_kj_per_mol);


}

// from Jiang 2023
// Jiang, W., Toptan, A., Hales, J. D., Spencer, B. W., & 
// Novascone, S. R. (2023). Fission product transport in TRISO particles 
// and pebbles (No. INL/EXT-21-63549-Rev001). Idaho National Lab.(INL), 
// Idaho Falls, ID (United States).
//
// table on page 13 of 105
#[inline]
pub fn get_d1_for_cs(triso_layer: TrisoLayerMaterial,
    gamma_neutron_fluence: ArealNumberDensity) -> DiffusionCoefficient{

    let coeff_m2_per_s: f64 = match triso_layer {
        TrisoLayerMaterial::Kernel => 5.6e-8,
        TrisoLayerMaterial::PyC => 6.3e-8,
        TrisoLayerMaterial::SiC => {
            let gamma_neutron_fluence_neutrons_per_sqm = 
                gamma_neutron_fluence.get::<per_square_meter>();

            let exponential_factor = 
                (gamma_neutron_fluence_neutrons_per_sqm * 1.1/5.0).exp();

            5.5e-14 * exponential_factor
        },
    };

    return DiffusionCoefficient::new::<square_meter_per_second>(
        coeff_m2_per_s
    );


}

// from Jiang 2023
// Jiang, W., Toptan, A., Hales, J. D., Spencer, B. W., & 
// Novascone, S. R. (2023). Fission product transport in TRISO particles 
// and pebbles (No. INL/EXT-21-63549-Rev001). Idaho National Lab.(INL), 
// Idaho Falls, ID (United States).
//
// table on page 13 of 105
#[inline]
pub fn get_q1_for_cs(triso_layer: TrisoLayerMaterial,) -> MolarEnergy {

    let coeff_kj_per_mol: f64 = match triso_layer {
        TrisoLayerMaterial::Kernel => 209.0,
        TrisoLayerMaterial::PyC => 222.0,
        TrisoLayerMaterial::SiC => 125.0,
    };

    return MolarEnergy::new::<kilojoule_per_mole>(coeff_kj_per_mol);


}


// from Jiang 2023
// Jiang, W., Toptan, A., Hales, J. D., Spencer, B. W., & 
// Novascone, S. R. (2023). Fission product transport in TRISO particles 
// and pebbles (No. INL/EXT-21-63549-Rev001). Idaho National Lab.(INL), 
// Idaho Falls, ID (United States).
//
// table on page 13 of 105
#[inline]
pub fn get_d2_for_cs(triso_layer: TrisoLayerMaterial) -> DiffusionCoefficient{

    let coeff_m2_per_s: f64 = match triso_layer {
        TrisoLayerMaterial::Kernel => 5.2e-4,
        TrisoLayerMaterial::PyC => 0.0,
        TrisoLayerMaterial::SiC => {
            1.6e-2
        },
    };

    return DiffusionCoefficient::new::<square_meter_per_second>(
        coeff_m2_per_s
    );


}

// from Jiang 2023
// Jiang, W., Toptan, A., Hales, J. D., Spencer, B. W., & 
// Novascone, S. R. (2023). Fission product transport in TRISO particles 
// and pebbles (No. INL/EXT-21-63549-Rev001). Idaho National Lab.(INL), 
// Idaho Falls, ID (United States).
//
// table on page 13 of 105
#[inline]
pub fn get_q2_for_cs(triso_layer: TrisoLayerMaterial,) -> MolarEnergy {

    let coeff_kj_per_mol: f64 = match triso_layer {
        TrisoLayerMaterial::Kernel => 362.0,
        TrisoLayerMaterial::PyC => 0.0,
        TrisoLayerMaterial::SiC => 514.0,
    };

    return MolarEnergy::new::<kilojoule_per_mole>(coeff_kj_per_mol);


}





/// triso layer for diffusion
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TrisoLayerMaterial {
    Kernel,
    PyC,
    SiC,
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
