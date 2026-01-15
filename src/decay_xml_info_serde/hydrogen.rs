// notes, code was done from vibe coding using AI, then 
// modified

use crate::decay_xml_info_serde::SerdeNuclideVec;


#[cfg(test)]
mod parsing_tests {

    #[test]
    fn serde_nuclide_test() {
        use crate::decay_xml_info_serde::SerdeNuclideVec;
        let xml = r#"
    <nuclides>
      <nuclide name="H1" reactions="1">
        <reaction type="(n,gamma)" Q="2224648.0" target="H2"/>
      </nuclide>
      <nuclide name="H2" reactions="2">
        <reaction type="(n,2n)" Q="-2225002.0" target="H1"/>
        <reaction type="(n,gamma)" Q="6257402.0" target="H3"/>
      </nuclide>
      <nuclide name="H3" half_life="388789600.0" decay_modes="1" decay_energy="5690.0" reactions="1">
        <decay type="beta-" target="He3" branching_ratio="1.0"/>
        <source type="discrete" particle="electron">
          <parameters>18590.0 1.7828336471961835e-09</parameters>
        </source>
        <reaction type="(n,2n)" Q="-6257557.0" target="H2"/>
      </nuclide>
      <nuclide name="H4" half_life="9.90652e-23" decay_modes="1" decay_energy="2880390.0" reactions="0">
        <decay type="n" target="H3" branching_ratio="1.0"/>
      </nuclide>
      <nuclide name="H5" half_life="7.99473e-23" decay_modes="1" decay_energy="200000.0" reactions="0">
        <decay type="n" target="H4" branching_ratio="1.0"/>
      </nuclide>
      <nuclide name="H6" half_life="2.84812e-22" decay_modes="1" decay_energy="900000.0" reactions="0">
        <decay type="n" target="H5" branching_ratio="1.0"/>
      </nuclide>
      <nuclide name="H7" half_life="2.3e-23" decay_modes="1" decay_energy="107400.0" reactions="0">
        <decay type="n,n" target="H5" branching_ratio="1.0"/>
      </nuclide>
    </nuclides>
    "#;

        let nuclides: SerdeNuclideVec = serde_xml_rs::from_str(xml).unwrap();
        dbg!("{:#?}", nuclides);
    }
}


pub fn get_hydrogen_xml_serde_data() -> SerdeNuclideVec {
    let xml = r#"
    <nuclides>
      <nuclide name="H1" reactions="1">
        <reaction type="(n,gamma)" Q="2224648.0" target="H2"/>
      </nuclide>
      <nuclide name="H2" reactions="2">
        <reaction type="(n,2n)" Q="-2225002.0" target="H1"/>
        <reaction type="(n,gamma)" Q="6257402.0" target="H3"/>
      </nuclide>
      <nuclide name="H3" half_life="388789600.0" decay_modes="1" decay_energy="5690.0" reactions="1">
        <decay type="beta-" target="He3" branching_ratio="1.0"/>
        <source type="discrete" particle="electron">
          <parameters>18590.0 1.7828336471961835e-09</parameters>
        </source>
        <reaction type="(n,2n)" Q="-6257557.0" target="H2"/>
      </nuclide>
      <nuclide name="H4" half_life="9.90652e-23" decay_modes="1" decay_energy="2880390.0" reactions="0">
        <decay type="n" target="H3" branching_ratio="1.0"/>
      </nuclide>
      <nuclide name="H5" half_life="7.99473e-23" decay_modes="1" decay_energy="200000.0" reactions="0">
        <decay type="n" target="H4" branching_ratio="1.0"/>
      </nuclide>
      <nuclide name="H6" half_life="2.84812e-22" decay_modes="1" decay_energy="900000.0" reactions="0">
        <decay type="n" target="H5" branching_ratio="1.0"/>
      </nuclide>
      <nuclide name="H7" half_life="2.3e-23" decay_modes="1" decay_energy="107400.0" reactions="0">
        <decay type="n,n" target="H5" branching_ratio="1.0"/>
      </nuclide>
    </nuclides>
    "#;

    let nuclides: SerdeNuclideVec = serde_xml_rs::from_str(xml).unwrap();

    return nuclides;
}
