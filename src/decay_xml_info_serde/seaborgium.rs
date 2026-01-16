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
        use crate::decay_xml_info_serde::seaborgium::get_seaborgium_xml_serde_data;
        assert_eq!(nuclides,get_seaborgium_xml_serde_data());

    }
}


use crate::decay_xml_info_serde::SerdeNuclideVec;
pub fn get_seaborgium_xml_serde_data() -> SerdeNuclideVec {
    
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
