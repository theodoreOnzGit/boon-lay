#[cfg(test)]
mod parsing_tests {
#[test]
fn serde_nuclide_test() {
    use crate::decay_xml_info_serde::SerdeNuclideVec;
    let xml = r#"
    <nuclides>
      <nuclide name="Hs263" half_life="0.00355" decay_modes="1" decay_energy="10670000.0" reactions="0">
    <decay type="alpha" target="Sg259" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Hs264" half_life="0.0008" decay_modes="2" decay_energy="5295340.0" reactions="0">
    <decay type="alpha" target="Sg260" branching_ratio="0.5"/>
    <decay type="sf" target="Hs264" branching_ratio="0.5"/>
  </nuclide>
  <nuclide name="Hs265" half_life="0.00205" decay_modes="2" decay_energy="10481390.0" reactions="0">
    <decay type="alpha" target="Sg261" branching_ratio="0.990099"/>
    <decay type="sf" target="Hs265" branching_ratio="0.009901000000000049"/>
  </nuclide>
  <nuclide name="Hs265_m1" half_life="0.0003" decay_modes="1" decay_energy="10740000.0" reactions="0">
    <decay type="alpha" target="Sg261" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Hs266" half_life="0.00265" decay_modes="2" decay_energy="10192920.0" reactions="0">
    <decay type="alpha" target="Sg262" branching_ratio="0.9861932"/>
    <decay type="sf" target="Hs266" branching_ratio="0.013806800000000008"/>
  </nuclide>
  <nuclide name="Hs267" half_life="0.0545" decay_modes="2" decay_energy="8096000.0" reactions="0">
    <decay type="alpha" target="Sg263" branching_ratio="0.8"/>
    <decay type="sf" target="Hs267" branching_ratio="0.2"/>
  </nuclide>
  <nuclide name="Hs268" half_life="1.2" decay_modes="1" decay_energy="9623000.0" reactions="0">
    <decay type="alpha" target="Sg264" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Hs269" half_life="12.9" decay_modes="1" decay_energy="9630000.0" reactions="0">
    <decay type="alpha" target="Sg265" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Hs273" half_life="50.0" decay_modes="1" decay_energy="9730000.0" reactions="0">
    <decay type="alpha" target="Sg269" branching_ratio="1.0"/>
  </nuclide>

</nuclides>
  "#;

    let nuclides: SerdeNuclideVec = serde_xml_rs::from_str(xml).unwrap();
    dbg!("{:#?}", nuclides);
}
}

