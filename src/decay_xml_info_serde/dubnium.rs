#[cfg(test)]
mod parsing_tests {
#[test]
fn serde_nuclide_test() {
    use crate::decay_xml_info_serde::SerdeNuclideVec;
    let xml = r#"
    <nuclides>
    
  <nuclide name="Db255" half_life="1.7" decay_modes="2" decay_energy="7776000.0" reactions="0">
    <decay type="alpha" target="Lr251" branching_ratio="0.8"/>
    <decay type="sf" target="Db255" branching_ratio="0.2"/>
  </nuclide>
  <nuclide name="Db256" half_life="1.7" decay_modes="3" decay_energy="7607118.8" reactions="0">
    <decay type="alpha" target="Lr252" branching_ratio="0.639872"/>
    <decay type="ec/beta+" target="Rf256" branching_ratio="0.359928"/>
    <decay type="sf" target="Db256" branching_ratio="0.00019999999999997797"/>
  </nuclide>
  <nuclide name="Db257" half_life="1.52" decay_modes="2" decay_energy="8676200.0" reactions="0">
    <decay type="alpha" target="Lr253" branching_ratio="0.94"/>
    <decay type="sf" target="Db257" branching_ratio="0.06"/>
  </nuclide>
  <nuclide name="Db257_m1" half_life="0.78" decay_modes="2" decay_energy="8030100.0" reactions="0">
    <decay type="alpha" target="Lr253" branching_ratio="0.87"/>
    <decay type="sf" target="Db257" branching_ratio="0.13"/>
  </nuclide>
  <nuclide name="Db258" half_life="4.0" decay_modes="3" decay_energy="7450969.8" reactions="0">
    <decay type="alpha" target="Lr254" branching_ratio="0.6633663"/>
    <decay type="ec/beta+" target="Rf258" branching_ratio="0.3267326"/>
    <decay type="sf" target="Db258" branching_ratio="0.009901099999999996"/>
  </nuclide>
  <nuclide name="Db258_m1" half_life="20.0" decay_modes="1" decay_energy="3565334.0" reactions="0">
    <decay type="ec/beta+" target="Rf258" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Db259" half_life="0.51" decay_modes="1" decay_energy="9618800.0" reactions="0">
    <decay type="alpha" target="Lr255" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Db260" half_life="1.52" decay_modes="3" decay_energy="8344926.76" reactions="0">
    <decay type="alpha" target="Lr256" branching_ratio="0.8819512"/>
    <decay type="ec/beta+" target="Rf260" branching_ratio="0.02439024"/>
    <decay type="sf" target="Db260" branching_ratio="0.09365855999999995"/>
  </nuclide>
  <nuclide name="Db261" half_life="1.8" decay_modes="2" decay_energy="7559580.0" reactions="0">
    <decay type="alpha" target="Lr257" branching_ratio="0.82"/>
    <decay type="sf" target="Db261" branching_ratio="0.18"/>
  </nuclide>
  <nuclide name="Db262" half_life="35.0" decay_modes="2" decay_energy="6033000.0" reactions="0">
    <decay type="alpha" target="Lr258" branching_ratio="0.67"/>
    <decay type="sf" target="Db262" branching_ratio="0.33"/>
  </nuclide>
  <nuclide name="Db263" half_life="28.5" decay_modes="3" decay_energy="3704464.4" reactions="0">
    <decay type="alpha" target="Lr259" branching_ratio="0.4141414"/>
    <decay type="ec/beta+" target="Rf263" branching_ratio="0.03030303"/>
    <decay type="sf" target="Db263" branching_ratio="0.55555557"/>
  </nuclide>
  <nuclide name="Db264" half_life="180.0" decay_modes="1" decay_energy="8660000.0" reactions="0">
    <decay type="alpha" target="Lr260" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Db265" half_life="900.0" decay_modes="1" decay_energy="8490000.0" reactions="0">
    <decay type="alpha" target="Lr261" branching_ratio="1.0"/>
  </nuclide>

</nuclides>
  "#;

        let nuclides: SerdeNuclideVec = serde_xml_rs::from_str(xml).unwrap();
        use crate::decay_xml_info_serde::dubnium::get_dubnium_xml_serde_data;
        assert_eq!(nuclides,get_dubnium_xml_serde_data());

    }
}


use crate::decay_xml_info_serde::SerdeNuclideVec;
pub fn get_dubnium_xml_serde_data() -> SerdeNuclideVec {
    
    let xml = r#"
    <nuclides>
    
  <nuclide name="Db255" half_life="1.7" decay_modes="2" decay_energy="7776000.0" reactions="0">
    <decay type="alpha" target="Lr251" branching_ratio="0.8"/>
    <decay type="sf" target="Db255" branching_ratio="0.2"/>
  </nuclide>
  <nuclide name="Db256" half_life="1.7" decay_modes="3" decay_energy="7607118.8" reactions="0">
    <decay type="alpha" target="Lr252" branching_ratio="0.639872"/>
    <decay type="ec/beta+" target="Rf256" branching_ratio="0.359928"/>
    <decay type="sf" target="Db256" branching_ratio="0.00019999999999997797"/>
  </nuclide>
  <nuclide name="Db257" half_life="1.52" decay_modes="2" decay_energy="8676200.0" reactions="0">
    <decay type="alpha" target="Lr253" branching_ratio="0.94"/>
    <decay type="sf" target="Db257" branching_ratio="0.06"/>
  </nuclide>
  <nuclide name="Db257_m1" half_life="0.78" decay_modes="2" decay_energy="8030100.0" reactions="0">
    <decay type="alpha" target="Lr253" branching_ratio="0.87"/>
    <decay type="sf" target="Db257" branching_ratio="0.13"/>
  </nuclide>
  <nuclide name="Db258" half_life="4.0" decay_modes="3" decay_energy="7450969.8" reactions="0">
    <decay type="alpha" target="Lr254" branching_ratio="0.6633663"/>
    <decay type="ec/beta+" target="Rf258" branching_ratio="0.3267326"/>
    <decay type="sf" target="Db258" branching_ratio="0.009901099999999996"/>
  </nuclide>
  <nuclide name="Db258_m1" half_life="20.0" decay_modes="1" decay_energy="3565334.0" reactions="0">
    <decay type="ec/beta+" target="Rf258" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Db259" half_life="0.51" decay_modes="1" decay_energy="9618800.0" reactions="0">
    <decay type="alpha" target="Lr255" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Db260" half_life="1.52" decay_modes="3" decay_energy="8344926.76" reactions="0">
    <decay type="alpha" target="Lr256" branching_ratio="0.8819512"/>
    <decay type="ec/beta+" target="Rf260" branching_ratio="0.02439024"/>
    <decay type="sf" target="Db260" branching_ratio="0.09365855999999995"/>
  </nuclide>
  <nuclide name="Db261" half_life="1.8" decay_modes="2" decay_energy="7559580.0" reactions="0">
    <decay type="alpha" target="Lr257" branching_ratio="0.82"/>
    <decay type="sf" target="Db261" branching_ratio="0.18"/>
  </nuclide>
  <nuclide name="Db262" half_life="35.0" decay_modes="2" decay_energy="6033000.0" reactions="0">
    <decay type="alpha" target="Lr258" branching_ratio="0.67"/>
    <decay type="sf" target="Db262" branching_ratio="0.33"/>
  </nuclide>
  <nuclide name="Db263" half_life="28.5" decay_modes="3" decay_energy="3704464.4" reactions="0">
    <decay type="alpha" target="Lr259" branching_ratio="0.4141414"/>
    <decay type="ec/beta+" target="Rf263" branching_ratio="0.03030303"/>
    <decay type="sf" target="Db263" branching_ratio="0.55555557"/>
  </nuclide>
  <nuclide name="Db264" half_life="180.0" decay_modes="1" decay_energy="8660000.0" reactions="0">
    <decay type="alpha" target="Lr260" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Db265" half_life="900.0" decay_modes="1" decay_energy="8490000.0" reactions="0">
    <decay type="alpha" target="Lr261" branching_ratio="1.0"/>
  </nuclide>

</nuclides>
  "#;


    let nuclides: SerdeNuclideVec = serde_xml_rs::from_str(xml).unwrap();

    return nuclides;
}
