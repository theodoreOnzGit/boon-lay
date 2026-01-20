use crate::prelude::NuclideReactionAndDecayData;
use uom::si::f64::*;

impl NuclideReactionAndDecayData {
    /// this obtains half life of the nuclide 
    ///
    /// if stable, this returns none
    pub fn get_half_life(&self) -> Option<Time> {

        match self.half_life_information {
            super::HalfLifeAndDecayEnergyInfo::Stable => None,
            super::HalfLifeAndDecayEnergyInfo::Unstable(
                half_life, _decay_energy
            ) => Some(half_life),
        }
    }


    /// this obtains decay energy of the nuclide 
    ///
    /// if stable, this returns none
    pub fn get_decay_energy(&self) -> Option<Energy> {

        match self.half_life_information {
            super::HalfLifeAndDecayEnergyInfo::Stable => None,
            super::HalfLifeAndDecayEnergyInfo::Unstable(
                _half_life, decay_energy
            ) => Some(decay_energy),
        }
    }




}
