
use crate::decay_xml_info_serde::ruthenium;
use crate::decay_xml_info_serde::SerdeNuclideVec;
use crate::nuclide_reaction_and_decay_data::NuclideReactionAndDecayData;



#[test] 
fn test_ruthenium_parsing(){
    let ruthenium_raw_data: SerdeNuclideVec = ruthenium::get_ruthenium_xml_serde_data();

    let nuclide_vec_raw = ruthenium_raw_data.nuclides;
    let mut nuclide_vec_processed: Vec<NuclideReactionAndDecayData>
        = vec![];

    for raw_nuclide_data in nuclide_vec_raw {

        let nuclide_data: NuclideReactionAndDecayData 
            = raw_nuclide_data.try_into().unwrap();
        // now, in doing this test, I realise the nuclear isomers have 
        // different naming conventions
        //
        // for example m1 is meant by m in my crate
        //
        // the openmc part m1, needs to be replaced by m

        nuclide_vec_processed.push(nuclide_data);
        dbg!(&nuclide_vec_processed);
        
    }
}
