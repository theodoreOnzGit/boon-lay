#[cfg(test)]
mod parsing_tests {
#[test]
fn serde_nuclide_test() {
    use crate::decay_xml_info_serde::SerdeNuclideVec;
    let xml = r#"
    <nuclides>
    
  <nuclide name="Md245" half_life="0.0009" decay_modes="2" decay_energy="4512000.0" reactions="0">
    <decay type="alpha" target="Es241" branching_ratio="0.5"/>
    <decay type="sf" target="Md245" branching_ratio="0.5"/>
  </nuclide>
  <nuclide name="Md245_m1" half_life="0.39" decay_modes="2" decay_energy="6176000.0" reactions="0">
    <decay type="alpha" target="Es241" branching_ratio="0.5"/>
    <decay type="ec/beta+" target="Fm245" branching_ratio="0.5"/>
  </nuclide>
  <nuclide name="Md246" half_life="0.9" decay_modes="1" decay_energy="8888710.0" reactions="0">
    <decay type="alpha" target="Es242" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Md247" half_life="1.12" decay_modes="1" decay_energy="8834000.0" reactions="0">
    <decay type="alpha" target="Es243" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Md247_m1" half_life="0.26" decay_modes="1" decay_energy="8764000.0" reactions="0">
    <decay type="alpha" target="Es243" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Md248" half_life="7.0" decay_modes="3" decay_energy="4532867.0" reactions="0">
    <decay type="alpha" target="Es244" branching_ratio="0.1999"/>
    <decay type="ec/beta+" target="Fm248" branching_ratio="0.7996002"/>
    <decay type="sf" target="Md248" branching_ratio="0.0004998000000000502"/>
  </nuclide>
  <nuclide name="Md249" half_life="24.0" decay_modes="2" decay_energy="6066066.6" reactions="0">
    <decay type="alpha" target="Es245" branching_ratio="0.6"/>
    <decay type="ec/beta+" target="Fm249" branching_ratio="0.4"/>
  </nuclide>
  <nuclide name="Md249_m1" half_life="1.9" decay_modes="1" decay_energy="8460000.0" reactions="0">
    <decay type="alpha" target="Es245" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Md250" half_life="27.5" decay_modes="2" decay_energy="3410690.0" reactions="0">
    <decay type="alpha" target="Es246" branching_ratio="0.07"/>
    <decay type="ec/beta+" target="Fm250" branching_ratio="0.93"/>
  </nuclide>
  <nuclide name="Md251" half_life="240.0" decay_modes="2" decay_energy="2623200.0" reactions="0">
    <decay type="alpha" target="Es247" branching_ratio="0.1"/>
    <decay type="ec/beta+" target="Fm251" branching_ratio="0.9"/>
  </nuclide>
  <nuclide name="Md252" half_life="138.0" decay_modes="1" decay_energy="2542000.0" reactions="0">
    <decay type="ec/beta+" target="Fm252" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Md253" half_life="630.0" decay_modes="1" decay_energy="1300666.6" reactions="0">
    <decay type="ec/beta+" target="Fm253" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Md254" half_life="1680.0" decay_modes="2" decay_energy="1740000.0" reactions="0">
    <decay type="ec/beta+" target="Fm254" branching_ratio="0.5"/>
    <decay type="ec/beta+" target="Fm254" branching_ratio="0.5"/>
  </nuclide>
  <nuclide name="Md254_m1" half_life="1680.0" decay_modes="1" decay_energy="1740000.0" reactions="0">
    <decay type="ec/beta+" target="Fm254" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Md255" half_life="1620.0" decay_modes="3" decay_energy="1270726.1" reactions="0">
    <decay type="alpha" target="Es251" branching_ratio="0.07988018"/>
    <decay type="ec/beta+" target="Fm255" branching_ratio="0.918622"/>
    <decay type="sf" target="Md255" branching_ratio="0.001497819999999983"/>
  </nuclide>
  <nuclide name="Md256" half_life="4620.0" decay_modes="3" decay_energy="1956720.0" reactions="0">
    <decay type="alpha" target="Es252" branching_ratio="0.08932038"/>
    <decay type="ec/beta+" target="Fm256" branching_ratio="0.8815534"/>
    <decay type="sf" target="Md256" branching_ratio="0.02912621999999998"/>
  </nuclide>
  <nuclide name="Md257" half_life="19872.0" decay_modes="3" decay_energy="1350615.4" reactions="0">
    <decay type="alpha" target="Es253" branching_ratio="0.1485148"/>
    <decay type="ec/beta+" target="Fm257" branching_ratio="0.8415841"/>
    <decay type="sf" target="Md257" branching_ratio="0.009901099999999996"/>
  </nuclide>
  <nuclide name="Md258" half_life="4449600.0" decay_modes="1" decay_energy="7271280.0" reactions="0">
    <decay type="alpha" target="Es254" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Md258_m1" half_life="3420.0" decay_modes="1" decay_energy="842000.0" reactions="0">
    <decay type="ec/beta+" target="Fm258" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Md259" half_life="5760.0" decay_modes="2" decay_energy="91243.83" reactions="0">
    <decay type="alpha" target="Es255" branching_ratio="0.01283316"/>
    <decay type="sf" target="Md259" branching_ratio="0.98716684"/>
  </nuclide>
  <nuclide name="Md260" half_life="2747520.0" decay_modes="4" decay_energy="1936740.0" reactions="0">
    <decay type="alpha" target="Es256" branching_ratio="0.25"/>
    <decay type="beta-" target="No260" branching_ratio="0.1"/>
    <decay type="ec/beta+" target="Fm260" branching_ratio="0.23"/>
    <decay type="sf" target="Md260" branching_ratio="0.42"/>
  </nuclide>
  <nuclide name="Md261" half_life="2400.0" decay_modes="1" decay_energy="6750000.0" reactions="0">
    <decay type="alpha" target="Es257" branching_ratio="1.0"/>
  </nuclide>


</nuclides>
  "#;

        let nuclides: SerdeNuclideVec = serde_xml_rs::from_str(xml).unwrap();
        use crate::decay_xml_info_serde::mendelevium::get_mendelevium_xml_serde_data;
        assert_eq!(nuclides,get_mendelevium_xml_serde_data());

    }
}


use crate::decay_xml_info_serde::SerdeNuclideVec;
pub fn get_mendelevium_xml_serde_data() -> SerdeNuclideVec {
    
    let xml = r#"
    <nuclides>
    
  <nuclide name="Md245" half_life="0.0009" decay_modes="2" decay_energy="4512000.0" reactions="0">
    <decay type="alpha" target="Es241" branching_ratio="0.5"/>
    <decay type="sf" target="Md245" branching_ratio="0.5"/>
  </nuclide>
  <nuclide name="Md245_m1" half_life="0.39" decay_modes="2" decay_energy="6176000.0" reactions="0">
    <decay type="alpha" target="Es241" branching_ratio="0.5"/>
    <decay type="ec/beta+" target="Fm245" branching_ratio="0.5"/>
  </nuclide>
  <nuclide name="Md246" half_life="0.9" decay_modes="1" decay_energy="8888710.0" reactions="0">
    <decay type="alpha" target="Es242" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Md247" half_life="1.12" decay_modes="1" decay_energy="8834000.0" reactions="0">
    <decay type="alpha" target="Es243" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Md247_m1" half_life="0.26" decay_modes="1" decay_energy="8764000.0" reactions="0">
    <decay type="alpha" target="Es243" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Md248" half_life="7.0" decay_modes="3" decay_energy="4532867.0" reactions="0">
    <decay type="alpha" target="Es244" branching_ratio="0.1999"/>
    <decay type="ec/beta+" target="Fm248" branching_ratio="0.7996002"/>
    <decay type="sf" target="Md248" branching_ratio="0.0004998000000000502"/>
  </nuclide>
  <nuclide name="Md249" half_life="24.0" decay_modes="2" decay_energy="6066066.6" reactions="0">
    <decay type="alpha" target="Es245" branching_ratio="0.6"/>
    <decay type="ec/beta+" target="Fm249" branching_ratio="0.4"/>
  </nuclide>
  <nuclide name="Md249_m1" half_life="1.9" decay_modes="1" decay_energy="8460000.0" reactions="0">
    <decay type="alpha" target="Es245" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Md250" half_life="27.5" decay_modes="2" decay_energy="3410690.0" reactions="0">
    <decay type="alpha" target="Es246" branching_ratio="0.07"/>
    <decay type="ec/beta+" target="Fm250" branching_ratio="0.93"/>
  </nuclide>
  <nuclide name="Md251" half_life="240.0" decay_modes="2" decay_energy="2623200.0" reactions="0">
    <decay type="alpha" target="Es247" branching_ratio="0.1"/>
    <decay type="ec/beta+" target="Fm251" branching_ratio="0.9"/>
  </nuclide>
  <nuclide name="Md252" half_life="138.0" decay_modes="1" decay_energy="2542000.0" reactions="0">
    <decay type="ec/beta+" target="Fm252" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Md253" half_life="630.0" decay_modes="1" decay_energy="1300666.6" reactions="0">
    <decay type="ec/beta+" target="Fm253" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Md254" half_life="1680.0" decay_modes="2" decay_energy="1740000.0" reactions="0">
    <decay type="ec/beta+" target="Fm254" branching_ratio="0.5"/>
    <decay type="ec/beta+" target="Fm254" branching_ratio="0.5"/>
  </nuclide>
  <nuclide name="Md254_m1" half_life="1680.0" decay_modes="1" decay_energy="1740000.0" reactions="0">
    <decay type="ec/beta+" target="Fm254" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Md255" half_life="1620.0" decay_modes="3" decay_energy="1270726.1" reactions="0">
    <decay type="alpha" target="Es251" branching_ratio="0.07988018"/>
    <decay type="ec/beta+" target="Fm255" branching_ratio="0.918622"/>
    <decay type="sf" target="Md255" branching_ratio="0.001497819999999983"/>
  </nuclide>
  <nuclide name="Md256" half_life="4620.0" decay_modes="3" decay_energy="1956720.0" reactions="0">
    <decay type="alpha" target="Es252" branching_ratio="0.08932038"/>
    <decay type="ec/beta+" target="Fm256" branching_ratio="0.8815534"/>
    <decay type="sf" target="Md256" branching_ratio="0.02912621999999998"/>
  </nuclide>
  <nuclide name="Md257" half_life="19872.0" decay_modes="3" decay_energy="1350615.4" reactions="0">
    <decay type="alpha" target="Es253" branching_ratio="0.1485148"/>
    <decay type="ec/beta+" target="Fm257" branching_ratio="0.8415841"/>
    <decay type="sf" target="Md257" branching_ratio="0.009901099999999996"/>
  </nuclide>
  <nuclide name="Md258" half_life="4449600.0" decay_modes="1" decay_energy="7271280.0" reactions="0">
    <decay type="alpha" target="Es254" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Md258_m1" half_life="3420.0" decay_modes="1" decay_energy="842000.0" reactions="0">
    <decay type="ec/beta+" target="Fm258" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Md259" half_life="5760.0" decay_modes="2" decay_energy="91243.83" reactions="0">
    <decay type="alpha" target="Es255" branching_ratio="0.01283316"/>
    <decay type="sf" target="Md259" branching_ratio="0.98716684"/>
  </nuclide>
  <nuclide name="Md260" half_life="2747520.0" decay_modes="4" decay_energy="1936740.0" reactions="0">
    <decay type="alpha" target="Es256" branching_ratio="0.25"/>
    <decay type="beta-" target="No260" branching_ratio="0.1"/>
    <decay type="ec/beta+" target="Fm260" branching_ratio="0.23"/>
    <decay type="sf" target="Md260" branching_ratio="0.42"/>
  </nuclide>
  <nuclide name="Md261" half_life="2400.0" decay_modes="1" decay_energy="6750000.0" reactions="0">
    <decay type="alpha" target="Es257" branching_ratio="1.0"/>
  </nuclide>


</nuclides>
  "#;

    let nuclides: SerdeNuclideVec = serde_xml_rs::from_str(xml).unwrap();

    return nuclides;
}
