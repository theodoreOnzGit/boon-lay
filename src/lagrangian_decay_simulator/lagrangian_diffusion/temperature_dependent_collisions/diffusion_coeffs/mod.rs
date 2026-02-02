use crate::lagrangian_decay_simulator::lagrangian_diffusion::temperature_dependent_collisions::TrisoLayerMaterial;
use uom::si::areal_number_density::per_square_meter;
use uom::si::f64::*;
use uom::si::diffusion_coefficient::square_meter_per_second;
use uom::si::molar_energy::kilojoule_per_mole;

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


// from Jiang 2023
// Jiang, W., Toptan, A., Hales, J. D., Spencer, B. W., & 
// Novascone, S. R. (2023). Fission product transport in TRISO particles 
// and pebbles (No. INL/EXT-21-63549-Rev001). Idaho National Lab.(INL), 
// Idaho Falls, ID (United States).
//
// table on page 13 of 105
#[inline]
pub fn get_d1_for_sr(triso_layer: TrisoLayerMaterial,
    gamma_neutron_fluence: ArealNumberDensity) -> DiffusionCoefficient{

    let coeff_m2_per_s: f64 = match triso_layer {
        TrisoLayerMaterial::Kernel => 2.2e-3,
        TrisoLayerMaterial::PyC => 2.3e-6,
        TrisoLayerMaterial::SiC => {
            1.2e-9
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
pub fn get_q1_for_sr(triso_layer: TrisoLayerMaterial,) -> MolarEnergy {

    let coeff_kj_per_mol: f64 = match triso_layer {
        TrisoLayerMaterial::Kernel => 488.0,
        TrisoLayerMaterial::PyC => 197.0,
        TrisoLayerMaterial::SiC => 205.0,
    };

    return MolarEnergy::new::<kilojoule_per_mole>(coeff_kj_per_mol);


}
