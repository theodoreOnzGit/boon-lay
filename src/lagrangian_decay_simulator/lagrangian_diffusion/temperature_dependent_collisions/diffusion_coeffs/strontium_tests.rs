use crate::lagrangian_decay_simulator::lagrangian_diffusion::temperature_dependent_collisions::{TrisoPebbleLayerMaterial, diffusion_coeff_jiang};


// If you already use the `approx` crate elsewhere, this is the nicest way:
// approx = "0.5"
use approx::assert_relative_eq;

use fission_yields_data::prelude::Nuclide;
// If you use `uom`, these are common imports. Adjust to match your project.
use uom::si::f64::*;
use uom::si::thermodynamic_temperature::kelvin;


#[test]
fn test_diffusion_coeff_jiang_matches_tabulated_sr_in_sic() {
    // GIVEN
    let triso_layer = TrisoPebbleLayerMaterial::SiC;
    let nuclide = Nuclide::Sr90; // adjust if your enum uses a different Sr nuclide name

    // neutron fluence: 5.5e25 n/m^2
    let fluence =
        ArealNumberDensity::new::<uom::si::areal_number_density::per_square_meter>(5.5e25);
    let gamma_neutron_fluence = Some(fluence);

    // (T [K], log10(D [m^2/s])) for Sr in SiC
    let data: &[(f64, f64)] = &[
        (626.0623, -26.0144),
        (668.3942, -24.9596),
        (713.5482, -23.9047),
        (769.9907, -22.8499),
        (832.0775, -21.7448),
        (902.6306, -20.7402),
        (984.4723, -19.7356),
        (1074.7803, -18.8314),
        (1187.6653, -17.9273),
        (1303.3724, -17.1738),
        (1399.3247, -16.4706),
        (1506.5655, -16.0185),
        (1599.6956, -15.5664),
        (1704.1143, -15.1646),
        (1797.2444, -14.9134),
        (1881.9082, -14.5116),
        (1969.3940, -14.2604),
        (2051.2357, -14.0093),
    ];

    // THEN
    let rtol = 0.02;

    for &(t_k, log10_d) in data {
        let temperature = ThermodynamicTemperature::new::<kelvin>(t_k);

        let got = diffusion_coeff_jiang(
            triso_layer,
            nuclide,
            temperature,
            gamma_neutron_fluence,
        )
        .unwrap_or_else(|| panic!("Expected Some(D) at T={t_k} K, got None"));

        let expected_d_m2_s = 10f64.powf(log10_d);
        let got_d_m2_s = got.get::<uom::si::diffusion_coefficient::square_meter_per_second>();

        assert_relative_eq!(
            got_d_m2_s,
            expected_d_m2_s,
            max_relative = rtol,
        );
    }
}
