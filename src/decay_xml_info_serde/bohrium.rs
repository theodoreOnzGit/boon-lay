#[cfg(test)]
mod parsing_tests {
#[test]
fn serde_nuclide_test() {
    use crate::decay_xml_info_serde::SerdeNuclideVec;
    let xml = r#"
    <nuclides>
      <nuclide name="Bh260" half_life="0.0003" decay_modes="1" decay_energy="10470000.0" reactions="0">
    <decay type="alpha" target="Db256" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Bh261" half_life="0.013" decay_modes="2" decay_energy="9556185.0" reactions="0">
    <decay type="alpha" target="Db257" branching_ratio="0.9047619"/>
    <decay type="sf" target="Bh261" branching_ratio="0.09523809999999999"/>
  </nuclide>
  <nuclide name="Bh262" half_life="0.102" decay_modes="2" decay_energy="10300460.0" reactions="0">
    <decay type="alpha" target="Db258" branching_ratio="0.5"/>
    <decay type="alpha" target="Db258" branching_ratio="0.5"/>
  </nuclide>
  <nuclide name="Bh262_m1" half_life="0.022" decay_modes="1" decay_energy="10320000.0" reactions="0">
    <decay type="alpha" target="Db258" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Bh263" half_life="0.0002" decay_modes="1" decay_energy="10080000.0" reactions="0">
    <decay type="alpha" target="Db259" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Bh264" half_life="0.66" decay_modes="1" decay_energy="9967000.0" reactions="0">
    <decay type="alpha" target="Db260" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Bh265" half_life="1.1" decay_modes="1" decay_energy="9770000.0" reactions="0">
    <decay type="alpha" target="Db261" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Bh266" half_life="5.4" decay_modes="1" decay_energy="9552000.0" reactions="0">
    <decay type="alpha" target="Db262" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Bh267" half_life="21.0" decay_modes="1" decay_energy="9370000.0" reactions="0">
    <decay type="alpha" target="Db263" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Bh269" half_life="50.0" decay_modes="1" decay_energy="8800000.0" reactions="0">
    <decay type="alpha" target="Db265" branching_ratio="1.0"/>
  </nuclide>

</nuclides>
  "#;

        let nuclides: SerdeNuclideVec = serde_xml_rs::from_str(xml).unwrap();
        use crate::decay_xml_info_serde::bohrium::get_bohrium_xml_serde_data;
        assert_eq!(nuclides,get_bohrium_xml_serde_data());

    }
}


use crate::decay_xml_info_serde::SerdeNuclideVec;
pub fn get_bohrium_xml_serde_data() -> SerdeNuclideVec {
    
    let xml = r#"
    <nuclides>
      <nuclide name="Bh260" half_life="0.0003" decay_modes="1" decay_energy="10470000.0" reactions="0">
    <decay type="alpha" target="Db256" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Bh261" half_life="0.013" decay_modes="2" decay_energy="9556185.0" reactions="0">
    <decay type="alpha" target="Db257" branching_ratio="0.9047619"/>
    <decay type="sf" target="Bh261" branching_ratio="0.09523809999999999"/>
  </nuclide>
  <nuclide name="Bh262" half_life="0.102" decay_modes="2" decay_energy="10300460.0" reactions="0">
    <decay type="alpha" target="Db258" branching_ratio="0.5"/>
    <decay type="alpha" target="Db258" branching_ratio="0.5"/>
  </nuclide>
  <nuclide name="Bh262_m1" half_life="0.022" decay_modes="1" decay_energy="10320000.0" reactions="0">
    <decay type="alpha" target="Db258" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Bh263" half_life="0.0002" decay_modes="1" decay_energy="10080000.0" reactions="0">
    <decay type="alpha" target="Db259" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Bh264" half_life="0.66" decay_modes="1" decay_energy="9967000.0" reactions="0">
    <decay type="alpha" target="Db260" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Bh265" half_life="1.1" decay_modes="1" decay_energy="9770000.0" reactions="0">
    <decay type="alpha" target="Db261" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Bh266" half_life="5.4" decay_modes="1" decay_energy="9552000.0" reactions="0">
    <decay type="alpha" target="Db262" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Bh267" half_life="21.0" decay_modes="1" decay_energy="9370000.0" reactions="0">
    <decay type="alpha" target="Db263" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Bh269" half_life="50.0" decay_modes="1" decay_energy="8800000.0" reactions="0">
    <decay type="alpha" target="Db265" branching_ratio="1.0"/>
  </nuclide>

</nuclides>
  "#;

    let nuclides: SerdeNuclideVec = serde_xml_rs::from_str(xml).unwrap();

    return nuclides;
}
