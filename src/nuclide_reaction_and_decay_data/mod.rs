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

        let nuclide_string: String = value.name;

        let nuclide_enum: Nuclide = 
            parse_nuclide_allow_underscore_isomer(&nuclide_string)
            .unwrap();


        let data = NuclideReactionAndDecayData {
            nuclide: nuclide_enum,
        };

        return data;
    }
}
/// First, to convert the nuclide name, I want to convert it to an enum
/// this will require long match statements
#[cfg(test)]
pub mod name_string_to_enum_conversion_test;
