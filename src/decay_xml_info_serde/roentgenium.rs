#[cfg(test)]
mod parsing_tests {
#[test]
fn serde_nuclide_test() {
    use crate::decay_xml_info_serde::SerdeNuclideVec;
    let xml = r#"
    <nuclides>
    
  <nuclide name="Rg272" half_life="0.0041" decay_modes="1" decay_energy="11442000.0" reactions="0">
    <decay type="alpha" target="Mt268" branching_ratio="1.0"/>
  </nuclide>

</nuclides>
  "#;

        let nuclides: SerdeNuclideVec = serde_xml_rs::from_str(xml).unwrap();
        use crate::decay_xml_info_serde::roentgenium::get_roentgenium_xml_serde_data;
        assert_eq!(nuclides,get_roentgenium_xml_serde_data());

    }
}


use crate::decay_xml_info_serde::SerdeNuclideVec;
pub fn get_roentgenium_xml_serde_data() -> SerdeNuclideVec {
    
    let xml = r#"
    <nuclides>
    
  <nuclide name="Rg272" half_life="0.0041" decay_modes="1" decay_energy="11442000.0" reactions="0">
    <decay type="alpha" target="Mt268" branching_ratio="1.0"/>
  </nuclide>

</nuclides>
  "#;

    let nuclides: SerdeNuclideVec = serde_xml_rs::from_str(xml).unwrap();

    return nuclides;
}
