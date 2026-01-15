#[cfg(test)]
mod parsing_tests {
#[test]
fn serde_nuclide_test() {
    use crate::decay_xml_info_serde::SerdeNuclideVec;
    let xml = r#"
    <nuclides>
    
  <nuclide name="No250" half_life="4.35e-06" decay_modes="2" decay_energy="175490.2" reactions="0">
    <decay type="alpha" target="Fm246" branching_ratio="0.01960784"/>
    <decay type="sf" target="No250" branching_ratio="0.98039216"/>
  </nuclide>
  <nuclide name="No251" half_life="0.8" decay_modes="2" decay_energy="8875302.0" reactions="0">
    <decay type="alpha" target="Fm247" branching_ratio="0.9964412"/>
    <decay type="sf" target="No251" branching_ratio="0.003558799999999973"/>
  </nuclide>
  <nuclide name="No251_m1" half_life="1.02" decay_modes="1" decay_energy="8752000.0" reactions="0">
    <decay type="alpha" target="Fm247" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="No252" half_life="2.44" decay_modes="3" decay_energy="5719203.334" reactions="0">
    <decay type="alpha" target="Fm248" branching_ratio="0.667"/>
    <decay type="ec/beta+" target="Md252" branching_ratio="0.011"/>
    <decay type="sf" target="No252" branching_ratio="0.322"/>
  </nuclide>
  <nuclide name="No253" half_life="97.2" decay_modes="1" decay_energy="8421330.0" reactions="0">
    <decay type="alpha" target="Fm249" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="No254" half_life="51.0" decay_modes="3" decay_energy="7471167.76" reactions="0">
    <decay type="alpha" target="Fm250" branching_ratio="0.8984726"/>
    <decay type="ec/beta+" target="Md254" branching_ratio="0.09983028"/>
    <decay type="sf" target="No254" branching_ratio="0.0016971199999999964"/>
  </nuclide>
  <nuclide name="No254_m1" half_life="0.28" decay_modes="1" decay_energy="500000.0" reactions="0">
    <decay type="IT" target="No254" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="No255" half_life="186.0" decay_modes="2" decay_energy="5672365.8" reactions="0">
    <decay type="alpha" target="Fm251" branching_ratio="0.61"/>
    <decay type="ec/beta+" target="Md255" branching_ratio="0.39"/>
  </nuclide>
  <nuclide name="No256" half_life="2.91" decay_modes="2" decay_energy="8535682.0063" reactions="0">
    <decay type="alpha" target="Fm252" branching_ratio="0.995"/>
    <decay type="sf" target="No256" branching_ratio="0.005"/>
    <source type="discrete" particle="photon">
      <parameters>16400.0 46600.0 0.012578082169450026 3.109031883107652e-05</parameters>
    </source>
    <source type="discrete" particle="alpha">
      <parameters>8402000.0 8448000.0 0.030810511273343267 0.20619342159852805</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>11600.0 19010.0 39394.0 44646.0 46399.9 46575.5 0.009605483565408053 0.022183568116807155 0.006470204985453012 0.001817820165127253 0.000462157669100149 7.086417592868952e-05</parameters>
    </source>
  </nuclide>
  <nuclide name="No257" half_life="25.0" decay_modes="1" decay_energy="8380435.0" reactions="0">
    <decay type="alpha" target="Fm253" branching_ratio="1.0"/>
    <source type="discrete" particle="alpha">
      <parameters>8220000.0 8270000.0 8320000.0 0.015249237972318799 0.007208730677823432 0.005267918572255585</parameters>
    </source>
  </nuclide>
  <nuclide name="No258" half_life="0.0012" decay_modes="1" decay_energy="0.0" reactions="0">
    <decay type="sf" target="No258" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="No259" half_life="3480.0" decay_modes="3" decay_energy="5449772.36" reactions="0">
    <decay type="alpha" target="Fm255" branching_ratio="0.6818181"/>
    <decay type="ec/beta+" target="Md259" branching_ratio="0.2272727"/>
    <decay type="sf" target="No259" branching_ratio="0.0909092"/>
  </nuclide>
  <nuclide name="No260" half_life="0.106" decay_modes="1" decay_energy="0.0" reactions="0">
    <decay type="sf" target="No260" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="No261" half_life="160000.0" decay_modes="1" decay_energy="7490000.0" reactions="0">
    <decay type="alpha" target="Fm257" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="No262" half_life="0.005" decay_modes="1" decay_energy="0.0" reactions="0">
    <decay type="sf" target="No262" branching_ratio="1.0"/>
  </nuclide>


</nuclides>
  "#;

    let nuclides: SerdeNuclideVec = serde_xml_rs::from_str(xml).unwrap();
    dbg!("{:#?}", nuclides);
}
}

