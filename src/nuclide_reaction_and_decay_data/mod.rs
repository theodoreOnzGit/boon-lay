use fission_yields_data::prelude::Nuclide;

#[derive(Debug, PartialEq)]
pub struct NuclideReactionAndDecayData {
    // contains the nuclide of interest
    pub nuclide: Nuclide,
}

/// First, to convert the nuclide name, I want to convert it to an enum
/// this will require long match statements
pub mod name_string_to_enum_conversion;
