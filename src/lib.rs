pub fn add(left: u64, right: u64) -> u64 {
    left + right
}


/// prelude is here for easy imports
pub mod prelude;

/// import the nuclide enum
pub use fission_yields_data::prelude::Nuclide;
/// import all nuclides into this crate
pub use fission_yields_data::prelude::Nuclide::*;

/// this contains the raw information 
/// based on pwr neutron spectrum
pub mod decay_xml_info_serde;
