#[cfg(test)]
mod parsing_tests {
#[test]
fn serde_nuclide_test() {
    use crate::decay_xml_info_serde::SerdeNuclideVec;
    let xml = r#"
    <nuclides>
      <nuclide name="Mt265" half_life="120.0" decay_modes="1" decay_energy="10990000.0" reactions="0">
    <decay type="alpha" target="Bh261" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Mt266" half_life="0.0018" decay_modes="1" decay_energy="10995570.0" reactions="0">
    <decay type="alpha" target="Bh262" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Mt266_m1" half_life="0.0017" decay_modes="1" decay_energy="11000000.0" reactions="0">
    <decay type="alpha" target="Bh262" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Mt267" half_life="0.01" decay_modes="1" decay_energy="10870000.0" reactions="0">
    <decay type="alpha" target="Bh263" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Mt268" half_life="0.0225" decay_modes="1" decay_energy="10732000.0" reactions="0">
    <decay type="alpha" target="Bh264" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Mt269" half_life="0.05" decay_modes="1" decay_energy="10500000.0" reactions="0">
    <decay type="alpha" target="Bh265" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Mt270" half_life="0.00605" decay_modes="1" decay_energy="10350000.0" reactions="0">
    <decay type="alpha" target="Bh266" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Mt271" half_life="5.0" decay_modes="1" decay_energy="10140000.0" reactions="0">
    <decay type="alpha" target="Bh267" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Mt273" half_life="20.0" decay_modes="2" decay_energy="5300000.0" reactions="0">
    <decay type="alpha" target="Bh269" branching_ratio="0.5"/>
    <decay type="sf" target="Mt273" branching_ratio="0.5"/>
  </nuclide>

</nuclides>
  "#;

        let nuclides: SerdeNuclideVec = serde_xml_rs::from_str(xml).unwrap();
        use crate::decay_xml_info_serde::meitnerium::get_meitnerium_xml_serde_data;
        assert_eq!(nuclides,get_meitnerium_xml_serde_data());

    }
}


use crate::decay_xml_info_serde::SerdeNuclideVec;
pub fn get_meitnerium_xml_serde_data() -> SerdeNuclideVec {
    
        let xml = 
            r#"
<nuclides>
     <nuclide name="He3" reactions="2">
    <reaction type="(n,gamma)" Q="20577780.0" target="He4"/>
    <reaction type="(n,p)" Q="763752.0" target="H3"/>
  </nuclide>
  <nuclide name="He4" reactions="0"/>
  <nuclide name="He5" half_life="7.595e-22" decay_modes="1" decay_energy="890000.0" reactions="0">
    <decay type="alpha" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="He6" half_life="0.8067" decay_modes="1" decay_energy="1567620.0" reactions="0">
    <decay type="beta-" target="Li6" branching_ratio="1.0"/>
    <source type="discrete" particle="electron">
      <parameters>3507800.0 0.8592378586338729</parameters>
    </source>
  </nuclide>
  <nuclide name="He7" half_life="3.038e-21" decay_modes="1" decay_energy="403000.0" reactions="0">
    <decay type="n" target="He6" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="He8" half_life="0.1191" decay_modes="2" decay_energy="5185336.0" reactions="0">
    <decay type="beta-" target="Li8" branching_ratio="0.84"/>
    <decay type="beta-,n" target="Li7" branching_ratio="0.16"/>
    <source type="discrete" particle="photon">
      <parameters>980000.0 4.888695480019765</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>981000.0 9671000.0 0.05237888014306891 4.888695480019765</parameters>
    </source>
  </nuclide>
  <nuclide name="He9" half_life="7e-21" decay_modes="1" decay_energy="100000.0" reactions="0">
    <decay type="n" target="He8" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="He10" half_life="1.519e-21" decay_modes="1" decay_energy="957680.0" reactions="0">
    <decay type="n" target="He9" branching_ratio="1.0"/>
  </nuclide>
</nuclides>
  "#;

    let nuclides: SerdeNuclideVec = serde_xml_rs::from_str(xml).unwrap();

    return nuclides;
}
