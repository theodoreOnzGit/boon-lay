
use crate::prelude::{decay_library::DecayLibrary, DecayType};
use fission_yields_data::prelude::Nuclide;
#[test]
fn test_rng(){

    let some_seed = 4;
    let mut rng = oorandom::Rand64::new(some_seed);
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
    assert_eq!(plat_counter,207);
    assert_eq!(ir_counter,4855);
    assert_eq!(ir168_m_counter,4938);
}
