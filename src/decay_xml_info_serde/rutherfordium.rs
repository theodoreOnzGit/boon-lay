#[cfg(test)]
mod parsing_tests {
#[test]
fn serde_nuclide_test() {
    use crate::decay_xml_info_serde::SerdeNuclideVec;
    let xml = r#"
    <nuclides>
      <nuclide name="Rf253" half_life="5.15e-05" decay_modes="3" decay_energy="2387500.0" reactions="0">
    <decay type="alpha" target="Fm245" branching_ratio="0.25"/>
    <decay type="sf" target="Rf253" branching_ratio="0.25"/>
    <decay type="sf" target="Rf253" branching_ratio="0.5"/>
  </nuclide>
  <nuclide name="Rf254" half_life="2.3e-05" decay_modes="1" decay_energy="0.0" reactions="0">
    <decay type="sf" target="Rf254" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Rf255" half_life="1.68" decay_modes="3" decay_energy="4333438.86" reactions="0">
    <decay type="alpha" target="No251" branching_ratio="0.4752475"/>
    <decay type="ec/beta+" target="Lr255" branching_ratio="0.00990099"/>
    <decay type="sf" target="Rf255" branching_ratio="0.51485151"/>
  </nuclide>
  <nuclide name="Rf256" half_life="0.0064" decay_modes="2" decay_energy="28575.01" reactions="0">
    <decay type="alpha" target="No252" branching_ratio="0.0032"/>
    <decay type="sf" target="Rf256" branching_ratio="0.9968"/>
  </nuclide>
  <nuclide name="Rf257" half_life="4.7" decay_modes="2" decay_energy="8919063.0" reactions="0">
    <decay type="alpha" target="No253" branching_ratio="0.9861932"/>
    <decay type="sf" target="Rf257" branching_ratio="0.013806800000000008"/>
  </nuclide>
  <nuclide name="Rf257_m1" half_life="3.9" decay_modes="2" decay_energy="8919063.0" reactions="0">
    <decay type="alpha" target="No253" branching_ratio="0.9861932"/>
    <decay type="sf" target="Rf257" branching_ratio="0.013806800000000008"/>
  </nuclide>
  <nuclide name="Rf258" half_life="0.012" decay_modes="2" decay_energy="1202500.0" reactions="0">
    <decay type="alpha" target="No254" branching_ratio="0.13"/>
    <decay type="sf" target="Rf258" branching_ratio="0.87"/>
  </nuclide>
  <nuclide name="Rf259" half_life="3.2" decay_modes="2" decay_energy="8391320.0" reactions="0">
    <decay type="alpha" target="No255" branching_ratio="0.92"/>
    <decay type="sf" target="Rf259" branching_ratio="0.08"/>
  </nuclide>
  <nuclide name="Rf260" half_life="0.021" decay_modes="1" decay_energy="0.0" reactions="0">
    <decay type="sf" target="Rf260" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Rf261" half_life="65.0" decay_modes="3" decay_energy="6757493.62" reactions="0">
    <decay type="alpha" target="No257" branching_ratio="0.7619047"/>
    <decay type="ec/beta+" target="Lr261" branching_ratio="0.1428571"/>
    <decay type="sf" target="Rf261" branching_ratio="0.09523820000000005"/>
  </nuclide>
  <nuclide name="Rf261_m1" half_life="81.0" decay_modes="3" decay_energy="6579000.0" reactions="0">
    <decay type="alpha" target="No257" branching_ratio="0.74"/>
    <decay type="ec/beta+" target="Lr261" branching_ratio="0.15"/>
    <decay type="sf" target="Rf261" branching_ratio="0.11"/>
  </nuclide>
  <nuclide name="Rf262" half_life="2.3" decay_modes="2" decay_energy="247281.6" reactions="0">
    <decay type="alpha" target="No258" branching_ratio="0.02912621"/>
    <decay type="sf" target="Rf262" branching_ratio="0.97087379"/>
  </nuclide>
  <nuclide name="Rf263" half_life="600.0" decay_modes="1" decay_energy="0.0" reactions="0">
    <decay type="sf" target="Rf263" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Rf264" half_life="3600.0" decay_modes="1" decay_energy="8140000.0" reactions="0">
    <decay type="alpha" target="No260" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Rf265" half_life="1.0" decay_modes="1" decay_energy="0.0" reactions="0">
    <decay type="sf" target="Rf265" branching_ratio="1.0"/>
  </nuclide>

</nuclides>
  "#;

        let nuclides: SerdeNuclideVec = serde_xml_rs::from_str(xml).unwrap();
        use crate::decay_xml_info_serde::rutherfordium::get_rutherfordium_xml_serde_data;
        assert_eq!(nuclides,get_rutherfordium_xml_serde_data());

    }
}


use crate::decay_xml_info_serde::SerdeNuclideVec;
pub fn get_rutherfordium_xml_serde_data() -> SerdeNuclideVec {
    
    let xml = r#"
    <nuclides>
      <nuclide name="Rf253" half_life="5.15e-05" decay_modes="3" decay_energy="2387500.0" reactions="0">
    <decay type="alpha" target="Fm245" branching_ratio="0.25"/>
    <decay type="sf" target="Rf253" branching_ratio="0.25"/>
    <decay type="sf" target="Rf253" branching_ratio="0.5"/>
  </nuclide>
  <nuclide name="Rf254" half_life="2.3e-05" decay_modes="1" decay_energy="0.0" reactions="0">
    <decay type="sf" target="Rf254" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Rf255" half_life="1.68" decay_modes="3" decay_energy="4333438.86" reactions="0">
    <decay type="alpha" target="No251" branching_ratio="0.4752475"/>
    <decay type="ec/beta+" target="Lr255" branching_ratio="0.00990099"/>
    <decay type="sf" target="Rf255" branching_ratio="0.51485151"/>
  </nuclide>
  <nuclide name="Rf256" half_life="0.0064" decay_modes="2" decay_energy="28575.01" reactions="0">
    <decay type="alpha" target="No252" branching_ratio="0.0032"/>
    <decay type="sf" target="Rf256" branching_ratio="0.9968"/>
  </nuclide>
  <nuclide name="Rf257" half_life="4.7" decay_modes="2" decay_energy="8919063.0" reactions="0">
    <decay type="alpha" target="No253" branching_ratio="0.9861932"/>
    <decay type="sf" target="Rf257" branching_ratio="0.013806800000000008"/>
  </nuclide>
  <nuclide name="Rf257_m1" half_life="3.9" decay_modes="2" decay_energy="8919063.0" reactions="0">
    <decay type="alpha" target="No253" branching_ratio="0.9861932"/>
    <decay type="sf" target="Rf257" branching_ratio="0.013806800000000008"/>
  </nuclide>
  <nuclide name="Rf258" half_life="0.012" decay_modes="2" decay_energy="1202500.0" reactions="0">
    <decay type="alpha" target="No254" branching_ratio="0.13"/>
    <decay type="sf" target="Rf258" branching_ratio="0.87"/>
  </nuclide>
  <nuclide name="Rf259" half_life="3.2" decay_modes="2" decay_energy="8391320.0" reactions="0">
    <decay type="alpha" target="No255" branching_ratio="0.92"/>
    <decay type="sf" target="Rf259" branching_ratio="0.08"/>
  </nuclide>
  <nuclide name="Rf260" half_life="0.021" decay_modes="1" decay_energy="0.0" reactions="0">
    <decay type="sf" target="Rf260" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Rf261" half_life="65.0" decay_modes="3" decay_energy="6757493.62" reactions="0">
    <decay type="alpha" target="No257" branching_ratio="0.7619047"/>
    <decay type="ec/beta+" target="Lr261" branching_ratio="0.1428571"/>
    <decay type="sf" target="Rf261" branching_ratio="0.09523820000000005"/>
  </nuclide>
  <nuclide name="Rf261_m1" half_life="81.0" decay_modes="3" decay_energy="6579000.0" reactions="0">
    <decay type="alpha" target="No257" branching_ratio="0.74"/>
    <decay type="ec/beta+" target="Lr261" branching_ratio="0.15"/>
    <decay type="sf" target="Rf261" branching_ratio="0.11"/>
  </nuclide>
  <nuclide name="Rf262" half_life="2.3" decay_modes="2" decay_energy="247281.6" reactions="0">
    <decay type="alpha" target="No258" branching_ratio="0.02912621"/>
    <decay type="sf" target="Rf262" branching_ratio="0.97087379"/>
  </nuclide>
  <nuclide name="Rf263" half_life="600.0" decay_modes="1" decay_energy="0.0" reactions="0">
    <decay type="sf" target="Rf263" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Rf264" half_life="3600.0" decay_modes="1" decay_energy="8140000.0" reactions="0">
    <decay type="alpha" target="No260" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Rf265" half_life="1.0" decay_modes="1" decay_energy="0.0" reactions="0">
    <decay type="sf" target="Rf265" branching_ratio="1.0"/>
  </nuclide>

</nuclides>
  "#;


    let nuclides: SerdeNuclideVec = serde_xml_rs::from_str(xml).unwrap();

    return nuclides;
}
