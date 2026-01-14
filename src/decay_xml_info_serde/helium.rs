
// notes, code was done from vibe coding using AI, then 
// modified


#[test]
fn serde_nuclide_test() {
    use crate::decay_xml_info_serde::SerdeNuclideVec;
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
    dbg!("{:#?}", nuclides);

}
