#[cfg(test)]
mod parsing_tests {
#[test]
fn serde_nuclide_test() {
    use crate::decay_xml_info_serde::SerdeNuclideVec;
    let xml = r#"
    <nuclides>
    
  <nuclide name="Lr251" half_life="1.341" decay_modes="2" decay_energy="6462000.0" reactions="0">
    <decay type="alpha" target="Md247" branching_ratio="0.5"/>
    <decay type="ec/beta+" target="No251" branching_ratio="0.5"/>
  </nuclide>
  <nuclide name="Lr252" half_life="0.38" decay_modes="3" decay_energy="4411555.6" reactions="0">
    <decay type="alpha" target="Md248" branching_ratio="0.3333333"/>
    <decay type="ec/beta+" target="No252" branching_ratio="0.3333333"/>
    <decay type="sf" target="Lr252" branching_ratio="0.3333334"/>
  </nuclide>
  <nuclide name="Lr253" half_life="0.575" decay_modes="2" decay_energy="8820484.0" reactions="0">
    <decay type="alpha" target="Md249" branching_ratio="0.987"/>
    <decay type="sf" target="Lr253" branching_ratio="0.013"/>
  </nuclide>
  <nuclide name="Lr253_m1" half_life="1.535" decay_modes="2" decay_energy="8221727.0" reactions="0">
    <decay type="alpha" target="Md249" branching_ratio="0.92"/>
    <decay type="sf" target="Lr253" branching_ratio="0.08"/>
  </nuclide>
  <nuclide name="Lr254" half_life="13.0" decay_modes="3" decay_energy="7489390.2" reactions="0">
    <decay type="alpha" target="Md250" branching_ratio="0.7592407"/>
    <decay type="ec/beta+" target="No254" branching_ratio="0.2397602"/>
    <decay type="sf" target="Lr254" branching_ratio="0.0009991000000000305"/>
  </nuclide>
  <nuclide name="Lr255" half_life="22.0" decay_modes="2" decay_energy="7634650.0" reactions="0">
    <decay type="alpha" target="Md251" branching_ratio="0.85"/>
    <decay type="ec/beta+" target="No255" branching_ratio="0.15"/>
  </nuclide>
  <nuclide name="Lr255_m1" half_life="2.53" decay_modes="2" decay_energy="3459224.0" reactions="0">
    <decay type="alpha" target="Md251" branching_ratio="0.4"/>
    <decay type="IT" target="Lr255" branching_ratio="0.6"/>
  </nuclide>
  <nuclide name="Lr256" half_life="27.0" decay_modes="3" decay_energy="7896980.6" reactions="0">
    <decay type="alpha" target="Md252" branching_ratio="0.849745"/>
    <decay type="ec/beta+" target="No256" branching_ratio="0.149955"/>
    <decay type="sf" target="Lr256" branching_ratio="0.00029999999999996696"/>
  </nuclide>
  <nuclide name="Lr257" half_life="0.646" decay_modes="2" decay_energy="9008000.0" reactions="0">
    <decay type="alpha" target="Md253" branching_ratio="0.9997"/>
    <decay type="sf" target="Lr257" branching_ratio="0.0003"/>
  </nuclide>
  <nuclide name="Lr258" half_life="4.1" decay_modes="2" decay_energy="8455000.0" reactions="0">
    <decay type="alpha" target="Md254" branching_ratio="0.95"/>
    <decay type="sf" target="Lr258" branching_ratio="0.05"/>
  </nuclide>
  <nuclide name="Lr259" half_life="6.2" decay_modes="2" decay_energy="6695520.0" reactions="0">
    <decay type="alpha" target="Md255" branching_ratio="0.78"/>
    <decay type="sf" target="Lr259" branching_ratio="0.22"/>
  </nuclide>
  <nuclide name="Lr260" half_life="180.0" decay_modes="3" decay_energy="5614974.6" reactions="0">
    <decay type="alpha" target="Md256" branching_ratio="0.6153846"/>
    <decay type="ec/beta+" target="No260" branching_ratio="0.3076923"/>
    <decay type="sf" target="Lr260" branching_ratio="0.07692310000000013"/>
  </nuclide>
  <nuclide name="Lr261" half_life="2340.0" decay_modes="1" decay_energy="0.0" reactions="0">
    <decay type="sf" target="Lr261" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Lr262" half_life="14400.0" decay_modes="1" decay_energy="0.0" reactions="0">
    <decay type="sf" target="Lr262" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Lr263" half_life="18000.0" decay_modes="1" decay_energy="7620000.0" reactions="0">
    <decay type="alpha" target="Md259" branching_ratio="1.0"/>
  </nuclide>


</nuclides>
  "#;

        let nuclides: SerdeNuclideVec = serde_xml_rs::from_str(xml).unwrap();
        use crate::decay_xml_info_serde::lawrencium::get_lawrencium_xml_serde_data;
        assert_eq!(nuclides,get_lawrencium_xml_serde_data());

    }
}


use crate::decay_xml_info_serde::SerdeNuclideVec;
pub fn get_lawrencium_xml_serde_data() -> SerdeNuclideVec {
    
    let xml = r#"
    <nuclides>
    
  <nuclide name="Lr251" half_life="1.341" decay_modes="2" decay_energy="6462000.0" reactions="0">
    <decay type="alpha" target="Md247" branching_ratio="0.5"/>
    <decay type="ec/beta+" target="No251" branching_ratio="0.5"/>
  </nuclide>
  <nuclide name="Lr252" half_life="0.38" decay_modes="3" decay_energy="4411555.6" reactions="0">
    <decay type="alpha" target="Md248" branching_ratio="0.3333333"/>
    <decay type="ec/beta+" target="No252" branching_ratio="0.3333333"/>
    <decay type="sf" target="Lr252" branching_ratio="0.3333334"/>
  </nuclide>
  <nuclide name="Lr253" half_life="0.575" decay_modes="2" decay_energy="8820484.0" reactions="0">
    <decay type="alpha" target="Md249" branching_ratio="0.987"/>
    <decay type="sf" target="Lr253" branching_ratio="0.013"/>
  </nuclide>
  <nuclide name="Lr253_m1" half_life="1.535" decay_modes="2" decay_energy="8221727.0" reactions="0">
    <decay type="alpha" target="Md249" branching_ratio="0.92"/>
    <decay type="sf" target="Lr253" branching_ratio="0.08"/>
  </nuclide>
  <nuclide name="Lr254" half_life="13.0" decay_modes="3" decay_energy="7489390.2" reactions="0">
    <decay type="alpha" target="Md250" branching_ratio="0.7592407"/>
    <decay type="ec/beta+" target="No254" branching_ratio="0.2397602"/>
    <decay type="sf" target="Lr254" branching_ratio="0.0009991000000000305"/>
  </nuclide>
  <nuclide name="Lr255" half_life="22.0" decay_modes="2" decay_energy="7634650.0" reactions="0">
    <decay type="alpha" target="Md251" branching_ratio="0.85"/>
    <decay type="ec/beta+" target="No255" branching_ratio="0.15"/>
  </nuclide>
  <nuclide name="Lr255_m1" half_life="2.53" decay_modes="2" decay_energy="3459224.0" reactions="0">
    <decay type="alpha" target="Md251" branching_ratio="0.4"/>
    <decay type="IT" target="Lr255" branching_ratio="0.6"/>
  </nuclide>
  <nuclide name="Lr256" half_life="27.0" decay_modes="3" decay_energy="7896980.6" reactions="0">
    <decay type="alpha" target="Md252" branching_ratio="0.849745"/>
    <decay type="ec/beta+" target="No256" branching_ratio="0.149955"/>
    <decay type="sf" target="Lr256" branching_ratio="0.00029999999999996696"/>
  </nuclide>
  <nuclide name="Lr257" half_life="0.646" decay_modes="2" decay_energy="9008000.0" reactions="0">
    <decay type="alpha" target="Md253" branching_ratio="0.9997"/>
    <decay type="sf" target="Lr257" branching_ratio="0.0003"/>
  </nuclide>
  <nuclide name="Lr258" half_life="4.1" decay_modes="2" decay_energy="8455000.0" reactions="0">
    <decay type="alpha" target="Md254" branching_ratio="0.95"/>
    <decay type="sf" target="Lr258" branching_ratio="0.05"/>
  </nuclide>
  <nuclide name="Lr259" half_life="6.2" decay_modes="2" decay_energy="6695520.0" reactions="0">
    <decay type="alpha" target="Md255" branching_ratio="0.78"/>
    <decay type="sf" target="Lr259" branching_ratio="0.22"/>
  </nuclide>
  <nuclide name="Lr260" half_life="180.0" decay_modes="3" decay_energy="5614974.6" reactions="0">
    <decay type="alpha" target="Md256" branching_ratio="0.6153846"/>
    <decay type="ec/beta+" target="No260" branching_ratio="0.3076923"/>
    <decay type="sf" target="Lr260" branching_ratio="0.07692310000000013"/>
  </nuclide>
  <nuclide name="Lr261" half_life="2340.0" decay_modes="1" decay_energy="0.0" reactions="0">
    <decay type="sf" target="Lr261" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Lr262" half_life="14400.0" decay_modes="1" decay_energy="0.0" reactions="0">
    <decay type="sf" target="Lr262" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Lr263" half_life="18000.0" decay_modes="1" decay_energy="7620000.0" reactions="0">
    <decay type="alpha" target="Md259" branching_ratio="1.0"/>
  </nuclide>


</nuclides>
  "#;

    let nuclides: SerdeNuclideVec = serde_xml_rs::from_str(xml).unwrap();

    return nuclides;
}
