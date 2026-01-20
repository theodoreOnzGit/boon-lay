use fission_yields_data::prelude::Nuclide;

use crate::prelude::HalfLifeAndDecayEnergyInfo;

// this code here is meant to simulate decay chains 
// Basically, it takes information from the nuclide, converts it into decay 
// data and then terminates it as it reaches stability
//
// 
#[derive(Debug, PartialEq,Clone)]
pub struct DecayChain {
    pub nuclides_and_decay_data: Vec<(Nuclide,HalfLifeAndDecayEnergyInfo)>
}

// and then I want to implement a method that starts from a single nuclide 
// stable or unstable 

impl DecayChain {

    pub fn new_from_nuclide(starting_nuclide: Nuclide) -> DecayChain {

        let mut nuclides_and_decay_data: Vec<(Nuclide,HalfLifeAndDecayEnergyInfo)>
            = vec![];

        // let's first check if the nuclide is stable by obtaining the decay 
        // data 
        //
        // to do so, the nuclide must get the right z first



        return DecayChain {
            nuclides_and_decay_data
        };
    }
}


