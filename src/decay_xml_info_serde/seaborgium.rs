#[cfg(test)]
mod parsing_tests {
#[test]
fn serde_nuclide_test() {
    use crate::decay_xml_info_serde::SerdeNuclideVec;
    let xml = r#"
    <nuclides>
      <nuclide name="Sg258" half_life="0.0032" decay_modes="1" decay_energy="0.0" reactions="0">
    <decay type="sf" target="Sg258" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Sg259" half_life="0.555" decay_modes="2" decay_energy="8046000.0" reactions="0">
    <decay type="alpha" target="Rf255" branching_ratio="0.8181818"/>
    <decay type="sf" target="Sg259" branching_ratio="0.18181820000000004"/>
  </nuclide>
  <nuclide name="Sg260" half_life="0.0036" decay_modes="2" decay_energy="4961425.0" reactions="0">
    <decay type="alpha" target="Rf256" branching_ratio="0.5"/>
    <decay type="sf" target="Sg260" branching_ratio="0.5"/>
  </nuclide>
  <nuclide name="Sg261" half_life="0.23" decay_modes="2" decay_energy="9705941.0" reactions="0">
    <decay type="alpha" target="Rf257" branching_ratio="0.990099"/>
    <decay type="sf" target="Sg261" branching_ratio="0.009901000000000049"/>
  </nuclide>
  <nuclide name="Sg262" half_life="0.0079" decay_modes="2" decay_energy="2112000.0" reactions="0">
    <decay type="alpha" target="Rf258" branching_ratio="0.22"/>
    <decay type="sf" target="Sg262" branching_ratio="0.78"/>
  </nuclide>
  <nuclide name="Sg263" half_life="1.0" decay_modes="2" decay_energy="6573700.0" reactions="0">
    <decay type="alpha" target="Rf259" branching_ratio="0.7"/>
    <decay type="sf" target="Sg263" branching_ratio="0.3"/>
  </nuclide>
  <nuclide name="Sg263_m1" half_life="0.12" decay_modes="2" decay_energy="4695500.0" reactions="0">
    <decay type="alpha" target="Rf259" branching_ratio="0.5"/>
    <decay type="IT" target="Sg263" branching_ratio="0.5"/>
  </nuclide>
  <nuclide name="Sg264" half_life="0.045" decay_modes="2" decay_energy="2437941.0" reactions="0">
    <decay type="alpha" target="Rf260" branching_ratio="0.2647058"/>
    <decay type="sf" target="Sg264" branching_ratio="0.7352942"/>
  </nuclide>
  <nuclide name="Sg265" half_life="8.0" decay_modes="2" decay_energy="3903239.0" reactions="0">
    <decay type="alpha" target="Rf261" branching_ratio="0.43"/>
    <decay type="sf" target="Sg265" branching_ratio="0.57"/>
  </nuclide>
  <nuclide name="Sg266" half_life="25.0" decay_modes="2" decay_energy="2351708.0" reactions="0">
    <decay type="alpha" target="Rf262" branching_ratio="0.2647058"/>
    <decay type="sf" target="Sg266" branching_ratio="0.7352942"/>
  </nuclide>
  <nuclide name="Sg269" half_life="50.0" decay_modes="1" decay_energy="8700000.0" reactions="0">
    <decay type="alpha" target="Rf265" branching_ratio="1.0"/>
  </nuclide>

</nuclides>
  "#;

    let nuclides: SerdeNuclideVec = serde_xml_rs::from_str(xml).unwrap();
    dbg!("{:#?}", nuclides);
}
}

