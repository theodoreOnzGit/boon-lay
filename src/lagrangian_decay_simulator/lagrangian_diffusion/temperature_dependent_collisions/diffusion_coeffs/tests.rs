#[cfg(test)]
mod tests {
    use crate::lagrangian_decay_simulator::lagrangian_diffusion::temperature_dependent_collisions::{TrisoPebbleLayerMaterial, diffusion_coeff_jiang};


    // If you already use the `approx` crate elsewhere, this is the nicest way:
    // approx = "0.5"
    use approx::assert_relative_eq;

    use fission_yields_data::prelude::Nuclide;
    // If you use `uom`, these are common imports. Adjust to match your project.
    use uom::si::f64::*;
    use uom::si::thermodynamic_temperature::kelvin;

    // Adjust these to your actual constructors / enums.
    // The test assumes:
    // - you want Cs in SiC
    // - neutron fluence is supplied as ArealNumberDensity in n/m^2
    #[test]
    fn test_diffusion_coeff_jiang_matches_tabulated_cs_in_sic() {
        // GIVEN
        let triso_layer = TrisoPebbleLayerMaterial::SiC;
        let nuclide = Nuclide::Cs137;

        // neutron fluence: 5.5e25 n/m^2
        // Adjust constructor/units to your ArealNumberDensity type.
        let fluence = ArealNumberDensity::new::<uom::si::areal_number_density::per_square_meter>(5.5e25);
        let gamma_neutron_fluence = Some(fluence);

        // (T [K], log10(D [m^2/s]))
        let data: &[(f64, f64)] = &[
            (608.9744, -23.4219),
            (649.3590, -22.7841),
            (696.4744, -22.1860),
            (743.5897, -21.5880),
            (795.1923, -20.8704),
            (869.2308, -20.1927),
            (952.2436, -19.5150),
            (1048.7179, -18.9967),
            (1147.4359, -18.3987),
            (1259.6154, -17.8804),
            (1324.6795, -17.6013),
            (1403.2051, -17.4419),
            (1470.5128, -17.1229),
            (1535.5769, -17.0033),
            (1584.9359, -16.8837),
            (1688.1410, -16.6445),
            (1744.2308, -16.3654),
            (1811.5385, -16.1262),
            (1883.3333, -15.8472),
            (1950.6410, -15.4086),
            // note that for these values, a larger error 
            // bound is required as the logscale gets bigger
            (2017.9487, -14.9302),
            (2087.5000, -14.6910),
        ];

        // THEN
        // Tolerance: choose something appropriate for your implementation
        // (e.g., regression fit, interpolation, or piecewise model).
        // Here we allow 2% relative error.
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

            dbg!(&temperature);
            // expected D in m^2/s
            let expected_d_m2_s = 10f64.powf(log10_d);

            // Convert `got` to f64 in m^2/s. Adjust accessor to match your type.
            // Common patterns:
            // - got.get::<diffusion_coefficient::square_meter_per_second>()
            // - got.value
            // - f64::from(got)
            let got_d_m2_s = got.get::<uom::si::diffusion_coefficient::square_meter_per_second>();


            assert_relative_eq!(
                got_d_m2_s,
                expected_d_m2_s,
                max_relative=rtol,
            );
        }
    }
}
