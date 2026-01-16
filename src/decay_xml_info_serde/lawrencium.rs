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
