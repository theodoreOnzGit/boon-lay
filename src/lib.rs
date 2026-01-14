pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

/// prelude is here for easy imports
pub mod prelude;

/// import the nuclide enum
pub use fission_yields_data::prelude::Nuclide;
/// import all nuclides into this crate
pub use fission_yields_data::prelude::Nuclide::*;

/// this contains the raw information 
pub mod decay_xml_info_serde;

#[test]
pub fn test_1(){
    use prelude::*;
    let seaborgium = Nuclide::Sg264;

    dbg!(&seaborgium);

    // I should be able to see what this decays to next 
    //
    // Then if I wanted to simulate a particle, I make use of the data 
    // to plot a decay path.

    todo!();
}
