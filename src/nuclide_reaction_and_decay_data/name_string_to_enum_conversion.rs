
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
