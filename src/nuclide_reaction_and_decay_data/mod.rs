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

            // halogens
            // Br67, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Br67") {
                nuclide_string = format!("{stripped}K35");
            }
            // At196_m, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("At196_m") {
                nuclide_string = format!("{stripped}K35");
            }
            // chalcogens
            // O27, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("O27") {
                nuclide_string = format!("{stripped}K35");
            }
            // Po191_m, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Po191_m") {
                nuclide_string = format!("{stripped}K35");
            }
            // Po205_m1, just replace with Po205_m for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Po205_m1") {
                nuclide_string = format!("{stripped}Po205_m");
            }
            // Po205_m, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Po205_m") {
                nuclide_string = format!("{stripped}K35");
            }

            // pnictogens 
            // As60, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("As60") {
                nuclide_string = format!("{stripped}K35");
            }
            // As61, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("As61") {
                nuclide_string = format!("{stripped}K35");
            }
            // As62, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("As62") {
                nuclide_string = format!("{stripped}K35");
            }
            // As62, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("As62") {
                nuclide_string = format!("{stripped}K35");
            }
            // Sb103, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Sb103") {
                nuclide_string = format!("{stripped}K35");
            }
            // Bi184_m, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Bi184_m") {
                nuclide_string = format!("{stripped}K35");
            }
            // Bi186_m, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Bi186_m") {
                nuclide_string = format!("{stripped}K35");
            }
            // Bi187_m, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Bi187_m") {
                nuclide_string = format!("{stripped}K35");
            }
            // Bi187_m, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Bi187_m") {
                nuclide_string = format!("{stripped}K35");
            }
            // Bi189_m, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Bi189_m") {
                nuclide_string = format!("{stripped}K35");
            }
            // Bi204_m1, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Bi204_m1") {
                nuclide_string = format!("{stripped}Bi204_m");
            }
            // Bi204_m, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Bi204_m") {
                nuclide_string = format!("{stripped}K35");
            }
            // Bi208_m, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Bi208_m") {
                nuclide_string = format!("{stripped}K35");
            }
            // P24, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("P24") {
                nuclide_string = format!("{stripped}K35");
            }
            // N25, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("N25") {
                nuclide_string = format!("{stripped}K35");
            }

            // transition metals
            // Zn61_m1, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Zn61_m1") {
                nuclide_string = format!("{stripped}K35");
            }
            // Zn61_m, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Zn61_m") {
                nuclide_string = format!("{stripped}K35");
            }
            // Zn73_m1, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Zn73_m1") {
                nuclide_string = format!("{stripped}K35");
            }
            // Zn73_m, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Zn73_m") {
                nuclide_string = format!("{stripped}K35");
            }
            // Cu52, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Cu52") {
                nuclide_string = format!("{stripped}K35");
            }
            // Cu76_m, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Cu76_m") {
                nuclide_string = format!("{stripped}K35");
            }
            // Co49, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Co49") {
                nuclide_string = format!("{stripped}K35");
            }
            // V40, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("V40") {
                nuclide_string = format!("{stripped}K35");
            }
            // V41, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("V41") {
                nuclide_string = format!("{stripped}K35");
            }
            // Mn45, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Mn45") {
                nuclide_string = format!("{stripped}K35");
            }
            // V46_m, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("V46_m") {
                nuclide_string = format!("{stripped}K35");
            }
            // Ti38, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Ti38") {
                nuclide_string = format!("{stripped}K35");
            }
            // Sc36, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Sc36") {
                nuclide_string = format!("{stripped}K35");
            }
            // Sc37, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Sc37") {
                nuclide_string = format!("{stripped}K35");
            }
            // Sc38, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Sc38") {
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
