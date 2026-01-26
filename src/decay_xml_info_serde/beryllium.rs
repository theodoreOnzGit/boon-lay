
// notes, code was done from vibe coding using AI, then 
// modified


#[cfg(test)]
mod parsing_tests {
    use crate::decay_xml_info_serde::beryllium::get_beryllium_xml_serde_data;

#[test]
fn serde_nuclide_test() {
    use crate::decay_xml_info_serde::SerdeNuclideVec;
    let xml = r#"
    <nuclides>
          <nuclide name="Be5" half_life="1e-09" decay_modes="1" decay_energy="4527000.0" reactions="0">
    <decay type="p" target="Li4" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Be6" half_life="4.95326e-21" decay_modes="1" decay_energy="1372000.0" reactions="0">
    <decay type="p,p" target="He4" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Be7" half_life="4598208.0" decay_modes="1" decay_energy="49861.81" reactions="2">
    <decay type="ec/beta+" target="Li7" branching_ratio="1.0"/>
    <source type="discrete" particle="photon">
      <parameters>477603.5 1.5737558120567467e-08</parameters>
    </source>
    <source type="discrete" particle="positron">
      <parameters>384203.0 861815.0 1.5737558120567467e-08 1.3500533575459982e-07</parameters>
    </source>
    <reaction type="(n,p)" Q="1644239.0" target="Li7"/>
    <reaction type="(n,a)" Q="18991520.0" target="He4"/>
  </nuclide>
  <nuclide name="Be8" half_life="8.18132e-17" decay_modes="1" decay_energy="91840.0" reactions="0">
    <decay type="alpha" target="He4" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Be9" reactions="4">
    <reaction type="(n,2n)" Q="-1572800.0" target="Be8"/>
    <reaction type="(n,gamma)" Q="6812380.0" target="Be10"/>
    <reaction type="(n,p)" Q="-12830000.0" target="Li9"/>
    <reaction type="(n,a)" Q="-600000.0" target="He6"/>
  </nuclide>
  <nuclide name="Be10" half_life="47652000000000.0" decay_modes="1" decay_energy="202560.0" reactions="0">
    <decay type="beta-" target="B10" branching_ratio="1.0"/>
    <source type="discrete" particle="electron">
      <parameters>556000.0 1.4546024942498642e-14</parameters>
    </source>
  </nuclide>
  <nuclide name="Be11" half_life="13.81" decay_modes="2" decay_energy="7501115.93" reactions="0">
    <decay type="beta-" target="B11" branching_ratio="0.969"/>
    <decay type="beta-,alpha" target="Li7" branching_ratio="0.031"/>
  </nuclide>
  <nuclide name="Be12" half_life="0.0213" decay_modes="1" decay_energy="5614900.0" reactions="0">
    <decay type="beta-" target="B12" branching_ratio="1.0"/>
    <source type="discrete" particle="electron">
      <parameters>11708000.0 32.542121153049074</parameters>
    </source>
  </nuclide>
  <nuclide name="Be13" half_life="2.7e-21" decay_modes="1" decay_energy="100000.0" reactions="0">
    <decay type="n" target="Be12" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Be14" half_life="0.00435" decay_modes="3" decay_energy="11297930.0" reactions="0">
    <decay type="beta-" target="B14" branching_ratio="0.14"/>
    <decay type="beta-,n" target="B13" branching_ratio="0.81"/>
    <decay type="beta-,n,n" target="B12" branching_ratio="0.05"/>
  </nuclide>
  <nuclide name="Be15" half_life="2e-07" decay_modes="1" decay_energy="1772000.0" reactions="0">
    <decay type="n" target="Be14" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Be16" half_life="2e-07" decay_modes="1" decay_energy="1581000.0" reactions="0">
    <decay type="n,n" target="Be14" branching_ratio="1.0"/>
  </nuclide>
</nuclides>
  "#;

        let nuclides: SerdeNuclideVec = serde_xml_rs::from_str(xml).unwrap();
        assert_eq!(nuclides,get_beryllium_xml_serde_data());
}
}
use crate::decay_xml_info_serde::SerdeNuclideVec;
pub fn get_beryllium_xml_serde_data() -> SerdeNuclideVec {
    let xml = r#"
    <nuclides>
          <nuclide name="Be5" half_life="1e-09" decay_modes="1" decay_energy="4527000.0" reactions="0">
    <decay type="p" target="Li4" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Be6" half_life="4.95326e-21" decay_modes="1" decay_energy="1372000.0" reactions="0">
    <decay type="p,p" target="He4" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Be7" half_life="4598208.0" decay_modes="1" decay_energy="49861.81" reactions="2">
    <decay type="ec/beta+" target="Li7" branching_ratio="1.0"/>
    <source type="discrete" particle="photon">
      <parameters>477603.5 1.5737558120567467e-08</parameters>
    </source>
    <source type="discrete" particle="positron">
      <parameters>384203.0 861815.0 1.5737558120567467e-08 1.3500533575459982e-07</parameters>
    </source>
    <reaction type="(n,p)" Q="1644239.0" target="Li7"/>
    <reaction type="(n,a)" Q="18991520.0" target="He4"/>
  </nuclide>
  <nuclide name="Be8" half_life="8.18132e-17" decay_modes="1" decay_energy="91840.0" reactions="0">
    <decay type="alpha" target="He4" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Be9" reactions="4">
    <reaction type="(n,2n)" Q="-1572800.0" target="Be8"/>
    <reaction type="(n,gamma)" Q="6812380.0" target="Be10"/>
    <reaction type="(n,p)" Q="-12830000.0" target="Li9"/>
    <reaction type="(n,a)" Q="-600000.0" target="He6"/>
  </nuclide>
  <nuclide name="Be10" half_life="47652000000000.0" decay_modes="1" decay_energy="202560.0" reactions="0">
    <decay type="beta-" target="B10" branching_ratio="1.0"/>
    <source type="discrete" particle="electron">
      <parameters>556000.0 1.4546024942498642e-14</parameters>
    </source>
  </nuclide>
  <nuclide name="Be11" half_life="13.81" decay_modes="2" decay_energy="7501115.93" reactions="0">
    <decay type="beta-" target="B11" branching_ratio="0.969"/>
    <decay type="beta-,alpha" target="Li7" branching_ratio="0.031"/>
  </nuclide>
  <nuclide name="Be12" half_life="0.0213" decay_modes="1" decay_energy="5614900.0" reactions="0">
    <decay type="beta-" target="B12" branching_ratio="1.0"/>
    <source type="discrete" particle="electron">
      <parameters>11708000.0 32.542121153049074</parameters>
    </source>
  </nuclide>
  <nuclide name="Be13" half_life="2.7e-21" decay_modes="1" decay_energy="100000.0" reactions="0">
    <decay type="n" target="Be12" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Be14" half_life="0.00435" decay_modes="3" decay_energy="11297930.0" reactions="0">
    <decay type="beta-" target="B14" branching_ratio="0.14"/>
    <decay type="beta-,n" target="B13" branching_ratio="0.81"/>
    <decay type="beta-,n,n" target="B12" branching_ratio="0.05"/>
  </nuclide>
  <nuclide name="Be15" half_life="2e-07" decay_modes="1" decay_energy="1772000.0" reactions="0">
    <decay type="n" target="Be14" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Be16" half_life="2e-07" decay_modes="1" decay_energy="1581000.0" reactions="0">
    <decay type="n,n" target="Be14" branching_ratio="1.0"/>
  </nuclide>
</nuclides>
  "#;

    let nuclides: SerdeNuclideVec = serde_xml_rs::from_str(xml).unwrap();

    return nuclides;
}
