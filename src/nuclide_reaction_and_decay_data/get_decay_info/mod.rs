use crate::prelude::{decay_library::DecayLibrary, DecayType, NuclideReactionAndDecayData};
use fission_yields_data::prelude::Nuclide;
use oorandom::Rand32;
use uom::si::{f64::*, ratio::ratio};

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


    /// get decay branch, branching ratio, decay type and target 
    ///
    /// Question is how to represent branch data most effectively 
    /// so that it is easy to access and construct decay chains
    pub fn get_decay_branch_info(&self) -> Option<(Ratio, Nuclide,DecayType)> {


        todo!()

    }


    // get next target using oorandom pseudorandom number generator
    // this is done along with decay type
    // from oorandom
    pub fn get_next_target_nuclide(&self, rng: &mut Rand32)-> Option<(Nuclide, DecayType)> {


        // first let's use the rng to get a number between 0 and 1 
        let mut random_num_between_0_and_1 = rng.rand_float();

        // let's obtain the branching ratios 
        let decay_branch_data = self.decay_information.clone();

        // let's do a case for 0 and 1 

        if decay_branch_data.len() == 0 {
            // this is no more nuclide, this is, the nuclide is stable 
            return None;

        }

        if decay_branch_data.len() == 1 {
            // this means radionuclide is unstable, but only one 
            // decay path
            let target_nuclide = decay_branch_data[0].target.unwrap();
            let decay_type = decay_branch_data[0].decay_type;

            return Some((target_nuclide,decay_type));
        }

        // then if we have more than one branch, we do the main code
        //
        // basically if we have a branching ratio of 0.6, 0.2 and 0.2 
        //
        // for decay branch 1, 2 and 3
        //
        // and our RNG produces 0.75, 
        //
        // we should be going for branch 2 
        //
        // |--------------|----|----|
        // 0.0          0.6   0.8   1.0
        //     branch 1     br 2  br3
        //
        // we can see 0.75 is in branch 2
        //
        // 
        // For this, the algorithm can be,
        //
        // suppose the branching ratio is 0.6 first 
        //
        // and 0.75 > 0.6, 
        //
        // we subtract 0.6 from 0.75 to get 0.15 
        //
        // the next branching ratio is 0.2 
        //
        // now, 0.15 < 0.2 
        //
        // so we select branch 2, and the target within branch 2
        //
        //


        for decay_data in decay_branch_data.iter() {
            // now, we check if the random number is greater than the 
            // branching ratio

            let branching_ratio_float = decay_data.branching_ratio.get::<ratio>();

            if random_num_between_0_and_1 as f64 > branching_ratio_float {
                // if greater than the branching ratio float, then 
                // don't select this path, move on.
                //
                // BUT subtract the branching_ratio_float from the random_num_between_0_and_1
                random_num_between_0_and_1 -= branching_ratio_float as f32;
            } else {
                // in this case, we want to select this branch 

                let target_nuclide = decay_data.target.unwrap();
                let decay_type = decay_data.decay_type;

                return Some((target_nuclide,decay_type));

            };

        }



        todo!("code is buggy!");


    }




}

#[test]
fn test_rng(){

    let some_seed = 4;
    let mut rng = oorandom::Rand32::new(some_seed);
    println!("Your random number is: {}", rng.rand_float());
    println!("Your random number is: {}", rng.rand_float());
    println!("Your random number is: {}", rng.rand_float());
    println!("Your random number is: {}", rng.rand_float());
    // let's consider the decay of Au172
    // which will branch into 3 types of decay
    //
    // Here is the data:
    // <decay type="alpha" target="Ir168" branching_ratio="0.49"/>
    // <decay type="alpha" target="Ir168_m1" branching_ratio="0.49"/>
    // <decay type="p" target="Pt171" branching_ratio="0.02"/>

    //
    let gold172 = Nuclide::Au172;
    // basically, i expect that from 1000 trials, I should get around 
    // 20 times the target of Pt171, 
    // 490 times the target of Ir168
    // and 490 times the target of Ir168m

    let plat171 = Nuclide::Pt171;
    let ir168 = Nuclide::Ir168;
    let ir168m = Nuclide::Ir168m;

    let mut plat_counter = 0;
    let mut ir_counter = 0;
    let mut ir168_m_counter = 0;
    // construct the library 
    let decay_library = DecayLibrary::new();
    let gold_decay_data = decay_library.match_nuclides_to_decay_data(gold172).unwrap();

    // let me perform about 1000 decays 

    for _i in 0..10000 {

        let (new_target,_decay_type):
            (Nuclide, DecayType) = gold_decay_data
             .get_next_target_nuclide(&mut rng)
             .unwrap();

        if new_target == plat171 {
            plat_counter += 1;
        } else if new_target == ir168 {
            ir_counter += 1;
        } else if new_target == ir168m {

            ir168_m_counter += 1;
        };
    }

    
    dbg!(&(plat_counter,ir_counter,ir168_m_counter));
    assert_eq!(plat_counter,209);
    assert_eq!(ir_counter,4882);
    assert_eq!(ir168_m_counter,4909);
}

