use fission_yields_data::prelude::Nuclide;
use uom::si::{f64::Time, time::{second, year}};

use crate::prelude::{decay_library::DecayLibrary, SingleNuclideSimulatorMC};

/// basically shows you how to get half life for a single nuclide
#[test]
fn half_life_test_tritium(){
    let mut decay_library: DecayLibrary = DecayLibrary::new();

    // begin tests
    let current_nuclide = Nuclide::H3;
    let sim = SingleNuclideSimulatorMC::new_decay_chain_simulation(
        current_nuclide, 
        &mut decay_library);

    let half_life: Time = Time::new::<year>(12.3);
    let test_half_life = sim.get_current_half_life();

    approx::assert_relative_eq!(
        half_life.get::<second>(),
        test_half_life.get::<second>(),
        max_relative=9e-3
    );



}
