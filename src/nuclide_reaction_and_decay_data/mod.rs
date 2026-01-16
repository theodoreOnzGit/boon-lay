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

        // these are missing nuclides I need to add in 
        {

            // Ru103m, just replace with Ru103 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Ru103_m") {
                nuclide_string = format!("{stripped}Ru103");
            }
            // alkali metals
            // Na24m, just replace with Na24 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Na24_m") {
                nuclide_string = format!("{stripped}Na24");
            }
            // Na36, just replace with Na35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Na36") {
                nuclide_string = format!("{stripped}Na35");
            }
            // K32, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("K32") {
                nuclide_string = format!("{stripped}K35");
            }
            // K33, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("K33") {
                nuclide_string = format!("{stripped}K35");
            }
            // K34, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("K34") {
                nuclide_string = format!("{stripped}K35");
            }
            // Rb71, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Rb71") {
                nuclide_string = format!("{stripped}K35");
            }
            // Cs125m, just replace with Cs125 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Cs125_m") {
                nuclide_string = format!("{stripped}Cs125");
            }
            // Cs144m, just replace with Cs144 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Cs144_m") {
                nuclide_string = format!("{stripped}Cs144");
            }
            // Fr214m, just replace with Fr214 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Fr214_m") {
                nuclide_string = format!("{stripped}Fr214");
            }
            // Fr218m, just replace with Fr218 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Fr218_m") {
                nuclide_string = format!("{stripped}Fr218");
            }

            // Alkaline earth metals
            // Be5, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Be5") {
                nuclide_string = format!("{stripped}K35");
            }
            // Mg39, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Mg39") {
                nuclide_string = format!("{stripped}K35");
            }
            // Ba130_m, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Ba130_m") {
                nuclide_string = format!("{stripped}K35");
            }
            // Ra203_m, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Ra203_m") {
                nuclide_string = format!("{stripped}K35");
            }
            // Ra207_m, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Ra207_m") {
                nuclide_string = format!("{stripped}K35");
            }
            // Ra213_m, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Ra213_m") {
                nuclide_string = format!("{stripped}K35");
            }
            // Ca34, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Ca34") {
                nuclide_string = format!("{stripped}K35");
            }
            //noble gases
            // Xe132_m, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Xe132_m") {
                nuclide_string = format!("{stripped}K35");
            }
            // Rn197_m, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Rn197_m") {
                nuclide_string = format!("{stripped}K35");
            }

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
