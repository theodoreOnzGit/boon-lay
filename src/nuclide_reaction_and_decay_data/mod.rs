use fission_yields_data::prelude::{parse_nuclide_allow_underscore_isomer, Nuclide};

use crate::decay_xml_info_serde::SerdeNuclideData;


#[derive(Debug, PartialEq)]
pub struct NuclideReactionAndDecayData {
    // contains the nuclide of interest
    pub nuclide: Nuclide,
}


impl From<SerdeNuclideData> for NuclideReactionAndDecayData {
    fn from(value: SerdeNuclideData) -> Self {


        // first convert nuclide name to string 

        let mut nuclide_string: String = value.name;

        {
            // this part modifies the isomer nuclides 
            // in the string from OpenMC style to my Nuclide style 
            //
            // I denote the first isomer as m 
            // OpenMC denotes it as m1 
            //
            // likewise, 
            //
            // OpenMC calls the second isomer m2 
            //
            // OpenMC => fission-yields-data library
            // m1 => m 
            // m2 => m1 
            // m3 => m2
            //
            //

            // first, if i detect m1, change it to m 

            if let Some(stripped) = nuclide_string.strip_suffix("m1") {
                nuclide_string = format!("{stripped}m");
            }

            // second, if i detect m2, change it to m1
            if let Some(stripped) = nuclide_string.strip_suffix("m2") {
                nuclide_string = format!("{stripped}m1");
            }

            // third, if i detect m3, change it to m2
            if let Some(stripped) = nuclide_string.strip_suffix("m3") {
                nuclide_string = format!("{stripped}m2");
            }

            // third, if i detect m4, change it to m3
            if let Some(stripped) = nuclide_string.strip_suffix("m4") {
                nuclide_string = format!("{stripped}m3");
            }

            // thus, openmc conversion should be okay

            dbg!(&nuclide_string);

        }

        // these are missing nuclides I need to add in 
        {

            // Ru103m, just replace with Ru103 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Ru103_m") {
                nuclide_string = format!("{stripped}Ru103");
            }
            // alkali metals
            // Na24m, just replace with Na24 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Na24_m") {
                nuclide_string = format!("{stripped}Na24");
            }
            // Na36, just replace with Na35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Na36") {
                nuclide_string = format!("{stripped}Na35");
            }
            // K32, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("K32") {
                nuclide_string = format!("{stripped}K35");
            }
            // K33, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("K33") {
                nuclide_string = format!("{stripped}K35");
            }
            // K34, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("K34") {
                nuclide_string = format!("{stripped}K35");
            }
            // Rb71, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Rb71") {
                nuclide_string = format!("{stripped}K35");
            }
            // Cs125m, just replace with Cs125 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Cs125_m") {
                nuclide_string = format!("{stripped}Cs125");
            }
            // Cs144m, just replace with Cs144 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Cs144_m") {
                nuclide_string = format!("{stripped}Cs144");
            }
            // Fr214m, just replace with Fr214 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Fr214_m") {
                nuclide_string = format!("{stripped}Fr214");
            }
            // Fr218m, just replace with Fr218 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Fr218_m") {
                nuclide_string = format!("{stripped}Fr218");
            }

            // Alkaline earth metals
            // Be5, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Be5") {
                nuclide_string = format!("{stripped}K35");
            }
            // Mg39, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Mg39") {
                nuclide_string = format!("{stripped}K35");
            }
            // Ba130_m, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Ba130_m") {
                nuclide_string = format!("{stripped}K35");
            }
            // Ra203_m, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Ra203_m") {
                nuclide_string = format!("{stripped}K35");
            }
            // Ra207_m, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Ra207_m") {
                nuclide_string = format!("{stripped}K35");
            }
            // Ra213_m, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Ra213_m") {
                nuclide_string = format!("{stripped}K35");
            }
            // Ca34, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Ca34") {
                nuclide_string = format!("{stripped}K35");
            }
            //noble gases
            // Xe132_m, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Xe132_m") {
                nuclide_string = format!("{stripped}K35");
            }
            // Rn197_m, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Rn197_m") {
                nuclide_string = format!("{stripped}K35");
            }

            // halogens
            // Br67, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Br67") {
                nuclide_string = format!("{stripped}K35");
            }
            // At196_m, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("At196_m") {
                nuclide_string = format!("{stripped}K35");
            }
            // chalcogens
            // O27, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("O27") {
                nuclide_string = format!("{stripped}K35");
            }
            // Po191_m, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Po191_m") {
                nuclide_string = format!("{stripped}K35");
            }
            // Po205_m1, just replace with Po205_m for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Po205_m1") {
                nuclide_string = format!("{stripped}Po205_m");
            }
            // Po205_m, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Po205_m") {
                nuclide_string = format!("{stripped}K35");
            }

            // pnictogens 
            // As60, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("As60") {
                nuclide_string = format!("{stripped}K35");
            }
            // As61, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("As61") {
                nuclide_string = format!("{stripped}K35");
            }
            // As62, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("As62") {
                nuclide_string = format!("{stripped}K35");
            }
            // As62, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("As62") {
                nuclide_string = format!("{stripped}K35");
            }
            // Sb103, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Sb103") {
                nuclide_string = format!("{stripped}K35");
            }
            // Bi184_m, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Bi184_m") {
                nuclide_string = format!("{stripped}K35");
            }
            // Bi186_m, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Bi186_m") {
                nuclide_string = format!("{stripped}K35");
            }
            // Bi187_m, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Bi187_m") {
                nuclide_string = format!("{stripped}K35");
            }
            // Bi187_m, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Bi187_m") {
                nuclide_string = format!("{stripped}K35");
            }
            // Bi189_m, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Bi189_m") {
                nuclide_string = format!("{stripped}K35");
            }
            // Bi204_m1, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Bi204_m1") {
                nuclide_string = format!("{stripped}Bi204_m");
            }
            // Bi204_m, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Bi204_m") {
                nuclide_string = format!("{stripped}K35");
            }
            // Bi208_m, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Bi208_m") {
                nuclide_string = format!("{stripped}K35");
            }
            // P24, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("P24") {
                nuclide_string = format!("{stripped}K35");
            }
            // N25, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("N25") {
                nuclide_string = format!("{stripped}K35");
            }

            // transition metals
            // Zn61_m1, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Zn61_m1") {
                nuclide_string = format!("{stripped}K35");
            }
            // Zn61_m, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Zn61_m") {
                nuclide_string = format!("{stripped}K35");
            }
            // Zn73_m1, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Zn73_m1") {
                nuclide_string = format!("{stripped}K35");
            }
            // Zn73_m, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Zn73_m") {
                nuclide_string = format!("{stripped}K35");
            }
            // Cu52, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Cu52") {
                nuclide_string = format!("{stripped}K35");
            }
            // Cu76_m, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Cu76_m") {
                nuclide_string = format!("{stripped}K35");
            }
            // Co49, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Co49") {
                nuclide_string = format!("{stripped}K35");
            }
            // V40, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("V40") {
                nuclide_string = format!("{stripped}K35");
            }
            // V41, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("V41") {
                nuclide_string = format!("{stripped}K35");
            }
            // Mn45, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Mn45") {
                nuclide_string = format!("{stripped}K35");
            }
            // V46_m, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("V46_m") {
                nuclide_string = format!("{stripped}K35");
            }
            // Ti38, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Ti38") {
                nuclide_string = format!("{stripped}K35");
            }
            // Sc36, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Sc36") {
                nuclide_string = format!("{stripped}K35");
            }
            // Sc37, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Sc37") {
                nuclide_string = format!("{stripped}K35");
            }
            // Sc38, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Sc38") {
                nuclide_string = format!("{stripped}K35");
            }
            // Y88_m, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Y88_m") {
                nuclide_string = format!("{stripped}K35");
            }
            // Y88_m1, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Y88_m1") {
                nuclide_string = format!("{stripped}K35");
            }
            // Nb90_m1, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Nb90_m1") {
                nuclide_string = format!("{stripped}K35");
            }
            // Tc86_m, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Tc86_m") {
                nuclide_string = format!("{stripped}K35");
            }
            // Tc117_m, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Tc117_m") {
                nuclide_string = format!("{stripped}K35");
            }
            // Pd117_m, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Pd117_m") {
                nuclide_string = format!("{stripped}K35");
            }
            // Ag95_m, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Ag95_m") {
                nuclide_string = format!("{stripped}K35");
            }
            // Ag95_m1, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Ag95_m1") {
                nuclide_string = format!("{stripped}K35");
            }
            // Ag95_m2, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Ag95_m2") {
                nuclide_string = format!("{stripped}K35");
            }
            // Ag114_m, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Ag114_m") {
                nuclide_string = format!("{stripped}K35");
            }
            // Ta157_m, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Ta157_m") {
                nuclide_string = format!("{stripped}K35");
            }
            // Ta157_m1, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Ta157_m1") {
                nuclide_string = format!("{stripped}K35");
            }
            // Ta158_m, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Ta158_m") {
                nuclide_string = format!("{stripped}K35");
            }
            // Ta176_m, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Ta176_m") {
                nuclide_string = format!("{stripped}K35");
            }
            // Ta176_m1, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Ta176_m1") {
                nuclide_string = format!("{stripped}K35");
            }
            // Ta179_m, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Ta179_m") {
                nuclide_string = format!("{stripped}K35");
            }
            // Ta179_m1, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Ta179_m1") {
                nuclide_string = format!("{stripped}K35");
            }
            // Ta185_m, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Ta185_m") {
                nuclide_string = format!("{stripped}K35");
            }
            // W180_m, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("W180_m") {
                nuclide_string = format!("{stripped}K35");
            }
            // W186_m, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("W186_m") {
                nuclide_string = format!("{stripped}K35");
            }
            // W190_m, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("W190_m") {
                nuclide_string = format!("{stripped}K35");
            }
            // Re183_m, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Re183_m") {
                nuclide_string = format!("{stripped}K35");
            }
            // Ir164_m, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Ir164_m") {
                nuclide_string = format!("{stripped}K35");
            }
            // Ir188_m, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Ir188_m") {
                nuclide_string = format!("{stripped}K35");
            }
            // Ir189_m, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Ir189_m") {
                nuclide_string = format!("{stripped}K35");
            }
            // Ir189_m1, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Ir189_m1") {
                nuclide_string = format!("{stripped}K35");
            }
            // Ir194_m1, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Ir194_m1") {
                nuclide_string = format!("{stripped}K35");
            }
            // Pt184_m, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Pt184_m") {
                nuclide_string = format!("{stripped}K35");
            }
            // Au169, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Au169") {
                nuclide_string = format!("{stripped}K35");
            }
            // Au192_m1, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Au192_m1") {
                nuclide_string = format!("{stripped}K35");
            }
            // Hg205_m, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Hg205_m") {
                nuclide_string = format!("{stripped}K35");
            }
            // lanthanides
            // La117_m, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("La117_m") {
                nuclide_string = format!("{stripped}K35");
            }
            // La118, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("La118") {
                nuclide_string = format!("{stripped}K35");
            }
            // La119, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("La119") {
                nuclide_string = format!("{stripped}K35");
            }
            // Ce119, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Ce119") {
                nuclide_string = format!("{stripped}K35");
            }
            // Ce120, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Ce120") {
                nuclide_string = format!("{stripped}K35");
            }
            // Ce132_m, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Ce132_m") {
                nuclide_string = format!("{stripped}K35");
            }
            // Pr122, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Pr122") {
                nuclide_string = format!("{stripped}K35");
            }
            // Pr123, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Pr123") {
                nuclide_string = format!("{stripped}K35");
            }
            // Nd124, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Nd124") {
                nuclide_string = format!("{stripped}K35");
            }
            // Pm126, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Pm126") {
                nuclide_string = format!("{stripped}K35");
            }
            // Pm127, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Pm127") {
                nuclide_string = format!("{stripped}K35");
            }
            // Pm138_m, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Pm138_m") {
                nuclide_string = format!("{stripped}K35");
            }
            // Pm142_m, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Pm142_m") {
                nuclide_string = format!("{stripped}K35");
            }
            // Sm128, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Sm128") {
                nuclide_string = format!("{stripped}K35");
            }
            // Sm143_m1, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Sm143_m1") {
                nuclide_string = format!("{stripped}K35");
            }
            // Sm153_m, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Sm153_m") {
                nuclide_string = format!("{stripped}K35");
            }
            // Eu133, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Eu133") {
                nuclide_string = format!("{stripped}K35");
            }
            // Eu136_m1, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Eu136_m1") {
                nuclide_string = format!("{stripped}K35");
            }
            // Gd155_m, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Gd155_m") {
                nuclide_string = format!("{stripped}K35");
            }
            // Tb136, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Tb136") {
                nuclide_string = format!("{stripped}K35");
            }
            // Tb137, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Tb137") {
                nuclide_string = format!("{stripped}K35");
            }
            // Tb141_m, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Tb141_m") {
                nuclide_string = format!("{stripped}K35");
            }
            // Tb145_m, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Tb145_m") {
                nuclide_string = format!("{stripped}K35");
            }
            // Tb146_m1, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Tb146_m1") {
                nuclide_string = format!("{stripped}K35");
            }
            // Dy138, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Dy138") {
                nuclide_string = format!("{stripped}K35");
            }
            // Dy157_m, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Dy157_m") {
                nuclide_string = format!("{stripped}K35");
            }
            // Ho148_m1, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Ho148_m1") {
                nuclide_string = format!("{stripped}K35");
            }
            // Ho155_m, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Ho155_m") {
                nuclide_string = format!("{stripped}K35");
            }
            // Er157_m, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Er157_m") {
                nuclide_string = format!("{stripped}K35");
            }
            // Yb148, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Yb148") {
                nuclide_string = format!("{stripped}K35");
            }
            // Yb171_m, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Yb171_m") {
                nuclide_string = format!("{stripped}K35");
            }
            // Yb175_m, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Yb175_m") {
                nuclide_string = format!("{stripped}K35");
            }
            // Lu153_m, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Lu153_m") {
                nuclide_string = format!("{stripped}K35");
            }
            // Lu155_m1, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Lu155_m1") {
                nuclide_string = format!("{stripped}K35");
            }
            // Lu161_m, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Lu161_m") {
                nuclide_string = format!("{stripped}K35");
            }
            // Lu177_m1, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Lu177_m1") {
                nuclide_string = format!("{stripped}K35");
            }
            // Lu179_m, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Lu179_m") {
                nuclide_string = format!("{stripped}K35");
            }
            // Lu180_m, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Lu180_m") {
                nuclide_string = format!("{stripped}K35");
            }

            // boron group
            // B6, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("B6") {
                nuclide_string = format!("{stripped}K35");
            }
            // Ga56, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Ga56") {
                nuclide_string = format!("{stripped}K35");
            }
            // Ga57, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Ga57") {
                nuclide_string = format!("{stripped}K35");
            }
            // Ga58, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Ga58") {
                nuclide_string = format!("{stripped}K35");
            }
            // In114_m1, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("In114_m1") {
                nuclide_string = format!("{stripped}K35");
            }
            // Tl179_m, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Tl179_m") {
                nuclide_string = format!("{stripped}K35");
            }
            // Tl181_m, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Tl181_m") {
                nuclide_string = format!("{stripped}K35");
            }
            // Tl183_m, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Tl183_m") {
                nuclide_string = format!("{stripped}K35");
            }
            // Tl188_m1, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Tl188_m1") {
                nuclide_string = format!("{stripped}K35");
            }
            // Tl198_m1, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Tl198_m1") {
                nuclide_string = format!("{stripped}K35");
            }
            // Tl199_m, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Tl199_m") {
                nuclide_string = format!("{stripped}K35");
            }
            // Tl200_m, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Tl200_m") {
                nuclide_string = format!("{stripped}K35");
            }
            // Tl201_m, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Tl201_m") {
                nuclide_string = format!("{stripped}K35");
            }


            // actinides 

            // Ac208_m, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Ac208_m") {
                nuclide_string = format!("{stripped}K35");
            }
            // Ac216_m, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Ac216_m") {
                nuclide_string = format!("{stripped}K35");
            }
            // Pa217_m, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Pa217_m") {
                nuclide_string = format!("{stripped}K35");
            }
            // Pa240, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Pa240") {
                nuclide_string = format!("{stripped}K35");
            }
            // U220, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("U220") {
                nuclide_string = format!("{stripped}K35");
            }
            // Am231, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Am231") {
                nuclide_string = format!("{stripped}K35");
            }
            // Am242_m1, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Am242_m1") {
                nuclide_string = format!("{stripped}K35");
            }
            // Am248, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Am248") {
                nuclide_string = format!("{stripped}K35");
            }
            // Am249, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Am249") {
                nuclide_string = format!("{stripped}K35");
            }
            // Cm244_m, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Cm244_m") {
                nuclide_string = format!("{stripped}K35");
            }
            // Bk235, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Bk235") {
                nuclide_string = format!("{stripped}K35");
            }
            // Bk254, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Bk254") {
                nuclide_string = format!("{stripped}K35");
            }
            // Es247_m, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Es247_m") {
                nuclide_string = format!("{stripped}K35");
            }
            // Es256, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Es256") {
                nuclide_string = format!("{stripped}K35");
            }
            // Es258, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Es258") {
                nuclide_string = format!("{stripped}K35");
            }
            // Fm260, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Fm260") {
                nuclide_string = format!("{stripped}K35");
            }
            // Md245_m, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Md245_m") {
                nuclide_string = format!("{stripped}K35");
            }
            // Md261, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Md261") {
                nuclide_string = format!("{stripped}K35");
            }
            // No261, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("No261") {
                nuclide_string = format!("{stripped}K35");
            }
            // Lr263, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Lr263") {
                nuclide_string = format!("{stripped}K35");
            }

            // Carbon Group
            // C21, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("C21") {
                nuclide_string = format!("{stripped}K35");
            }
            // Ge58, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Ge58") {
                nuclide_string = format!("{stripped}K35");
            }
            // Pb181_m, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Pb181_m") {
                nuclide_string = format!("{stripped}K35");
            }
            // Pb205_m, just replace with K35 for the time being
            if let Some(stripped) = nuclide_string.strip_suffix("Pb205_m") {
                nuclide_string = format!("{stripped}K35");
            }

            // heavier than actinides group

        }


        let nuclide_enum: Nuclide = 
            parse_nuclide_allow_underscore_isomer(&nuclide_string)
            .unwrap();


        // final step, finish the data
        let data = NuclideReactionAndDecayData {
            nuclide: nuclide_enum,
        };

        return data;
    }
}
/// this contains tests for alkali metals and hydrogen
#[cfg(test)]
pub mod alkali_metals_and_hydrogen;

/// this contains tests for alkaline_earth metals 
#[cfg(test)]
pub mod alkaline_earth_metals;

/// this contains tests for transition metals
#[cfg(test)]
pub mod transition_metals_test;


/// this contains tests for noble gases
#[cfg(test)]
pub mod noble_gases_test;


/// this contains tests for halogens
#[cfg(test)]
pub mod halogens_test;


/// this contains tests for chalcogens
#[cfg(test)]
pub mod chalcogens_test;


/// this contains tests for pnictogens
#[cfg(test)]
pub mod pnictogens_test;


/// this contains tests for lanthanides
#[cfg(test)]
pub mod lanthanides_test;


/// this contains tests for actinides
#[cfg(test)]
pub mod actinides_test;

/// this contains tests for the carbon group 
#[cfg(test)]
pub mod carbon_group_test;

/// this contains tests for the boron group
#[cfg(test)]
pub mod boron_group_test;

/// this contains tests for heavier than actinides 
#[cfg(test)]
pub mod heavier_than_actinides;
