#[cfg(test)]
mod parsing_tests {
#[test]
fn serde_nuclide_test() {
    use crate::decay_xml_info_serde::SerdeNuclideVec;
    let xml = r#"
    <nuclides>
      <nuclide name="Ds267" half_life="2.8e-06" decay_modes="1" decay_energy="12277000.0" reactions="0">
    <decay type="alpha" target="Hs263" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Ds268" half_life="0.0001" decay_modes="1" decay_energy="11660000.0" reactions="0">
    <decay type="alpha" target="Hs264" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Ds269" half_life="0.000268" decay_modes="1" decay_energy="11584560.0" reactions="0">
    <decay type="alpha" target="Hs265" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Ds270" half_life="0.00015" decay_modes="2" decay_energy="11173740.0" reactions="0">
    <decay type="alpha" target="Hs266" branching_ratio="0.998"/>
    <decay type="sf" target="Ds270" branching_ratio="0.002"/>
  </nuclide>
  <nuclide name="Ds270_m1" half_life="0.009" decay_modes="2" decay_energy="8970000.0" reactions="0">
    <decay type="alpha" target="Hs266" branching_ratio="0.7"/>
    <decay type="IT" target="Ds270" branching_ratio="0.3"/>
  </nuclide>
  <nuclide name="Ds271" half_life="0.001705" decay_modes="1" decay_energy="10869650.0" reactions="0">
    <decay type="alpha" target="Hs267" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Ds271_m1" half_life="0.0865" decay_modes="1" decay_energy="10870000.0" reactions="0">
    <decay type="alpha" target="Hs267" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Ds272" half_life="1.0" decay_modes="1" decay_energy="0.0" reactions="0">
    <decay type="sf" target="Ds272" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Ds273" half_life="0.000225" decay_modes="1" decay_energy="11367900.0" reactions="0">
    <decay type="alpha" target="Hs269" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Ds279_m1" half_life="0.19" decay_modes="2" decay_energy="960000.0" reactions="0">
    <decay type="alpha" target="Fm259" branching_ratio="0.1"/>
    <decay type="sf" target="Fm259" branching_ratio="0.9"/>
  </nuclide>

</nuclides>
  "#;

    let nuclides: SerdeNuclideVec = serde_xml_rs::from_str(xml).unwrap();
    dbg!("{:#?}", nuclides);
}
}

