use oorandom::Rand64;
use uom::si::time::second;
use uom::si::f64::*;

use crate::lagrangian_decay_simulator::monte_carlo_single_radionuclide_decay_simulator::SingleNuclideSimualtorMC;

/// basically we want to sample a time to live for 10000 particles
/// based on half life of 30s
#[test]
fn stochastic_half_life_calculator(){

    let half_life = Time::new::<second>(30.0);
    let mut rng = Rand64::new(77);

    let mut time_to_live_vec: Vec<Time> = vec![];


    // now lets do time to live for 10000 particles 
    let number_of_particles = 10000;
    // we will also have a survivor counter 
    // that is if the particles live beyond a certain time, then it 
    // survives 

    for _i in 1..number_of_particles {
        let time_to_live = 
            SingleNuclideSimualtorMC::get_time_to_decay_stochastic(&mut rng, half_life);

        time_to_live_vec.push(time_to_live);
    }

    // now let's determine a function to see how many particles remain 
    fn determine_surviving_fraction(
        simulated_time: Time, 
        time_to_live_vec: &Vec<Time>) -> f64 {

        let mut survivor_counter = 0;
        let number_of_particles = time_to_live_vec.len();

        for time_to_live in time_to_live_vec {

            if *time_to_live > simulated_time {
                survivor_counter += 1;
            }

        }


        let surviving_fraction: f64 = 
            survivor_counter as f64/number_of_particles as f64;

        return surviving_fraction;

    }

    // let's first test at 10 seconds 

    let surviving_fraction_10s = 
        determine_surviving_fraction(
            Time::new::<second>(10.0), &time_to_live_vec
        );
    dbg!(&time_to_live_vec);
    
    dbg!(&surviving_fraction_10s);

    todo!();


}
