use fission_yields_data::prelude::{parse_nuclide_allow_underscore_isomer, Nuclide};

use crate::decay_xml_info_serde::SerdeNuclideData;


#[derive(Debug, PartialEq)]
pub struct NuclideReactionAndDecayData {
    // contains the nuclide of interest
    pub nuclide: Nuclide,
}


impl From<SerdeNuclideData> for NuclideReactionAndDecayData {
    fn from(value: SerdeNuclideData) -> Self {


        // first convert nuclide name to string 

        let mut nuclide_string: String = value.name;

        {
            // this part modifies the isomer nuclides 
            // in the string from OpenMC style to my Nuclide style 
            //
            // I denote the first isomer as m 
            // OpenMC denotes it as m1 
            //
            // likewise, 
            //
            // OpenMC calls the second isomer m2 
            //
            // OpenMC => fission-yields-data library
            // m1 => m 
            // m2 => m1 
            // m3 => m2
            //
            //

            // first, if i detect m1, change it to m 

            if let Some(stripped) = nuclide_string.strip_suffix("m1") {
                nuclide_string = format!("{stripped}m");
            }

            // second, if i detect m2, change it to m1
            if let Some(stripped) = nuclide_string.strip_suffix("m2") {
                nuclide_string = format!("{stripped}m1");
            }

            // third, if i detect m3, change it to m2
            if let Some(stripped) = nuclide_string.strip_suffix("m3") {
                nuclide_string = format!("{stripped}m2");
            }

            // third, if i detect m4, change it to m3
            if let Some(stripped) = nuclide_string.strip_suffix("m4") {
                nuclide_string = format!("{stripped}m3");
            }

            // thus, openmc conversion should be okay

            dbg!(&nuclide_string);

        }




        let nuclide_enum: Nuclide = 
            parse_nuclide_allow_underscore_isomer(&nuclide_string)
            .unwrap();


        // final step, finish the data
        let data = NuclideReactionAndDecayData {
            nuclide: nuclide_enum,
        };

        return data;
    }
}
/// this contains tests for alkali metals and hydrogen
#[cfg(test)]
pub mod alkali_metals_and_hydrogen;

/// this contains tests for alkaline_earth metals 
#[cfg(test)]
pub mod alkaline_earth_metals;

/// this contains tests for transition metals
#[cfg(test)]
pub mod transition_metals_test;


/// this contains tests for noble gases
#[cfg(test)]
pub mod noble_gases_test;


/// this contains tests for halogens
#[cfg(test)]
pub mod halogens_test;


/// this contains tests for chalcogens
#[cfg(test)]
pub mod chalcogens_test;


/// this contains tests for pnictogens
#[cfg(test)]
pub mod pnictogens_test;


/// this contains tests for lanthanides
#[cfg(test)]
pub mod lanthanides_test;


/// this contains tests for actinides
#[cfg(test)]
pub mod actinides_test;

/// this contains tests for the carbon group 
#[cfg(test)]
pub mod carbon_group_test;

/// this contains tests for the boron group
#[cfg(test)]
pub mod boron_group_test;

/// this contains tests for heavier than actinides 
#[cfg(test)]
pub mod heavier_than_actinides;
