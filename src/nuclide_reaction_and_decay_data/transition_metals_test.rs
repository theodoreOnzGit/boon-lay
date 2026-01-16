
use crate::decay_xml_info_serde::*;
use crate::nuclide_reaction_and_decay_data::NuclideReactionAndDecayData;



#[test] 
fn test_scandium_parsing(){
    let scandium_raw_data: SerdeNuclideVec = scandium::get_scandium_xml_serde_data();

    let nuclide_vec_raw = scandium_raw_data.nuclides;
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

        dbg!(&nuclide_data);
        nuclide_vec_processed.push(nuclide_data);
        
    }
}
#[test] 
fn test_titanium_parsing(){
    let titanium_raw_data: SerdeNuclideVec = titanium::get_titanium_xml_serde_data();

    let nuclide_vec_raw = titanium_raw_data.nuclides;
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

        dbg!(&nuclide_data);
        nuclide_vec_processed.push(nuclide_data);
        
    }
}

#[test] 
fn test_vanadium_parsing(){
    let vanadium_raw_data: SerdeNuclideVec = vanadium::get_vanadium_xml_serde_data();

    let nuclide_vec_raw = vanadium_raw_data.nuclides;
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

        dbg!(&nuclide_data);
        nuclide_vec_processed.push(nuclide_data);
        
    }
}

#[test] 
fn test_chromium_parsing(){
    let chromium_raw_data: SerdeNuclideVec = chromium::get_chromium_xml_serde_data();

    let nuclide_vec_raw = chromium_raw_data.nuclides;
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

        dbg!(&nuclide_data);
        nuclide_vec_processed.push(nuclide_data);
        
    }
}


#[test] 
fn test_manganese_parsing(){
    let manganese_raw_data: SerdeNuclideVec = manganese::get_manganese_xml_serde_data();

    let nuclide_vec_raw = manganese_raw_data.nuclides;
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

        dbg!(&nuclide_data);
        nuclide_vec_processed.push(nuclide_data);
        
    }
}

#[test] 
fn test_iron_parsing(){
    let iron_raw_data: SerdeNuclideVec = iron::get_iron_xml_serde_data();

    let nuclide_vec_raw = iron_raw_data.nuclides;
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

        dbg!(&nuclide_data);
        nuclide_vec_processed.push(nuclide_data);
        
    }
}

#[test] 
fn test_cobalt_parsing(){
    let cobalt_raw_data: SerdeNuclideVec = cobalt::get_cobalt_xml_serde_data();

    let nuclide_vec_raw = cobalt_raw_data.nuclides;
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

        dbg!(&nuclide_data);
        nuclide_vec_processed.push(nuclide_data);
        
    }
}

#[test] 
fn test_nickel_parsing(){
    let nickel_raw_data: SerdeNuclideVec = nickel::get_nickel_xml_serde_data();

    let nuclide_vec_raw = nickel_raw_data.nuclides;
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

        dbg!(&nuclide_data);
        nuclide_vec_processed.push(nuclide_data);
        
    }
}

#[test] 
fn test_copper_parsing(){
    let copper_raw_data: SerdeNuclideVec = copper::get_copper_xml_serde_data();

    let nuclide_vec_raw = copper_raw_data.nuclides;
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

        dbg!(&nuclide_data);
        nuclide_vec_processed.push(nuclide_data);
        
    }
}
#[test] 
fn test_zinc_parsing(){
    let zinc_raw_data: SerdeNuclideVec = zinc::get_zinc_xml_serde_data();

    let nuclide_vec_raw = zinc_raw_data.nuclides;
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

        dbg!(&nuclide_data);
        nuclide_vec_processed.push(nuclide_data);
        
    }
}

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

        dbg!(&nuclide_data);
        nuclide_vec_processed.push(nuclide_data);
        
    }
}

// from chat gpt 5, the type that accepts m1 to translate to m 
//
//
// use std::str::FromStr;
// 
// // Normalise: remove underscores; map ...m1 -> ...m
// fn normalize_isomer_token(s: &str) -> String {
//     let mut t = s.replace('_', "");
//     if let Some(stripped) = t.strip_suffix("m1") {
//         t = format!("{stripped}m");
//     }
//     t
// }
// 
// pub fn parse_nuclide_with_m1_alias(s: &str) -> Option<Nuclide> {
//     let s = s.trim();
// 
//     match s {
//         // If it contains an underscore or ends with m1, normalise then parse
//         u if u.contains('_') || u.ends_with("m1") => {
//             let norm = normalize_isomer_token(u);
//             Nuclide::from_str(&norm).ok()
//         }
//         // Try exact parse first
//         _ => Nuclide::from_str(s).ok().or_else(|| {
//             // Fallback: try normalised (handles cases where only m1 alias appears)
//             let norm = normalize_isomer_token(s);
//             Nuclide::from_str(&norm).ok()
//         }),
//     }
// }
//
//
// we can do similar tricks for m2 and m3
