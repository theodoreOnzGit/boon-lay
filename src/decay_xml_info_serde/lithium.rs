
// notes, code was done from vibe coding using AI, then 
// modified


#[test]
fn serde_nuclide_test() {
    use crate::decay_xml_info_serde::SerdeNuclideVec;
    let xml = r#"
<nuclides>
          <nuclide name="Li4" half_life="7.55721e-23" decay_modes="1" decay_energy="3103000.0" reactions="0">
    <decay type="p" target="He3" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Li5" half_life="3.06868e-22" decay_modes="1" decay_energy="1965000.0" reactions="0">
    <decay type="alpha" target="H1" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Li6" reactions="2">
    <reaction type="(n,gamma)" Q="7250600.0" target="Li7"/>
    <reaction type="(n,p)" Q="-2727300.0" target="He6"/>
  </nuclide>
  <nuclide name="Li7" reactions="2">
    <reaction type="(n,2n)" Q="-7250500.0" target="Li6"/>
    <reaction type="(n,gamma)" Q="2032800.0" target="Li8"/>
  </nuclide>
  <nuclide name="Li8" half_life="0.838" decay_modes="1" decay_energy="9362853.1" reactions="0">
    <decay type="beta-,alpha" target="He4" branching_ratio="1.0"/>
    <source type="discrete" particle="electron">
      <parameters>12965000.0 0.827144606873443</parameters>
    </source>
    <source type="discrete" particle="alpha">
      <parameters>1566000.0 0.827144606873443</parameters>
    </source>
  </nuclide>
  <nuclide name="Li9" half_life="0.1783" decay_modes="2" decay_energy="5726145.7" reactions="0">
    <decay type="beta-" target="Be9" branching_ratio="0.505"/>
    <decay type="beta-,n" target="Be8" branching_ratio="0.495"/>
    <source type="discrete" particle="electron">
      <parameters>2320000.0 5670000.0 10830000.0 11177000.0 13606000.0 0.15550133046773873 0.05831299892540202 0.3887533261693468 1.3217613089757794 1.9632042971552015</parameters>
    </source>
  </nuclide>
  <nuclide name="Li10" half_life="2e-21" decay_modes="1" decay_energy="26400.0" reactions="0">
    <decay type="n" target="Li9" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Li11" half_life="0.00859" decay_modes="1" decay_energy="13700666.0" reactions="0">
    <decay type="beta-" target="Be11" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Li12" half_life="1e-08" decay_modes="1" decay_energy="120000.0" reactions="0">
    <decay type="n" target="Li11" branching_ratio="1.0"/>
  </nuclide>
</nuclides>
  "#;

        let nuclides: SerdeNuclideVec = serde_xml_rs::from_str(xml).unwrap();
        assert_eq!(nuclides,get_lithium_xml_serde_data());
}

use crate::decay_xml_info_serde::SerdeNuclideVec;
pub fn get_lithium_xml_serde_data() -> SerdeNuclideVec {
    let xml = r#"
<nuclides>
          <nuclide name="Li4" half_life="7.55721e-23" decay_modes="1" decay_energy="3103000.0" reactions="0">
    <decay type="p" target="He3" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Li5" half_life="3.06868e-22" decay_modes="1" decay_energy="1965000.0" reactions="0">
    <decay type="alpha" target="H1" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Li6" reactions="2">
    <reaction type="(n,gamma)" Q="7250600.0" target="Li7"/>
    <reaction type="(n,p)" Q="-2727300.0" target="He6"/>
  </nuclide>
  <nuclide name="Li7" reactions="2">
    <reaction type="(n,2n)" Q="-7250500.0" target="Li6"/>
    <reaction type="(n,gamma)" Q="2032800.0" target="Li8"/>
  </nuclide>
  <nuclide name="Li8" half_life="0.838" decay_modes="1" decay_energy="9362853.1" reactions="0">
    <decay type="beta-,alpha" target="He4" branching_ratio="1.0"/>
    <source type="discrete" particle="electron">
      <parameters>12965000.0 0.827144606873443</parameters>
    </source>
    <source type="discrete" particle="alpha">
      <parameters>1566000.0 0.827144606873443</parameters>
    </source>
  </nuclide>
  <nuclide name="Li9" half_life="0.1783" decay_modes="2" decay_energy="5726145.7" reactions="0">
    <decay type="beta-" target="Be9" branching_ratio="0.505"/>
    <decay type="beta-,n" target="Be8" branching_ratio="0.495"/>
    <source type="discrete" particle="electron">
      <parameters>2320000.0 5670000.0 10830000.0 11177000.0 13606000.0 0.15550133046773873 0.05831299892540202 0.3887533261693468 1.3217613089757794 1.9632042971552015</parameters>
    </source>
  </nuclide>
  <nuclide name="Li10" half_life="2e-21" decay_modes="1" decay_energy="26400.0" reactions="0">
    <decay type="n" target="Li9" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Li11" half_life="0.00859" decay_modes="1" decay_energy="13700666.0" reactions="0">
    <decay type="beta-" target="Be11" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Li12" half_life="1e-08" decay_modes="1" decay_energy="120000.0" reactions="0">
    <decay type="n" target="Li11" branching_ratio="1.0"/>
  </nuclide>
</nuclides>
  "#;

    let nuclides: SerdeNuclideVec = serde_xml_rs::from_str(xml).unwrap();

    return nuclides;
}
