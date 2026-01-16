
// notes, code was done from vibe coding using AI, then 
// modified


#[cfg(test)]
mod parsing_tests {
#[test]
    fn serde_nuclide_test() {
        use crate::decay_xml_info_serde::SerdeNuclideVec;
        let xml = r#"
    <nuclides>
          <nuclide name="C8" half_life="1.9813e-21" decay_modes="1" decay_energy="2141000.0" reactions="0">
    <decay type="p,p" target="Be6" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="C9" half_life="0.1265" decay_modes="2" decay_energy="11970417.0" reactions="0">
    <decay type="ec/beta+,p" target="Be8" branching_ratio="0.616"/>
    <decay type="ec/beta+,alpha" target="Li5" branching_ratio="0.384"/>
  </nuclide>
  <nuclide name="C10" half_life="19.29" decay_modes="1" decay_energy="2562080.9" reactions="0">
    <decay type="ec/beta+" target="B10" branching_ratio="1.0"/>
    <source type="discrete" particle="photon">
      <parameters>183.0 510998.9 718353.0 1021646.0 1.8656040194930016e-08 0.0718372851047073 0.03593297981129836 0.0005251604999421255</parameters>
    </source>
    <source type="discrete" particle="positron">
      <parameters>1907950.0 2929620.0 0.0005262384893364644 0.03540476500807227</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>170.0 1.1227338594595757e-05</parameters>
    </source>
  </nuclide>
  <nuclide name="C11" half_life="1223.1" decay_modes="1" decay_energy="1404206.1" reactions="0">
    <decay type="ec/beta+" target="B11" branching_ratio="1.0"/>
    <source type="discrete" particle="positron">
      <parameters>1982200.0 0.0005667134171857945</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>170.0 1.256279535662543e-06</parameters>
    </source>
    <source type="discrete" particle="photon">
      <parameters>183.0 510998.9 2.0875115407405614e-09 0.0011306952757007536</parameters>
    </source>
  </nuclide>
  <nuclide name="C12" reactions="3">
    <reaction type="(n,gamma)" Q="4946285.0" target="C13"/>
    <reaction type="(n,p)" Q="-12588000.0" target="B12"/>
    <reaction type="(n,a)" Q="-5702000.0" target="Be9"/>
  </nuclide>
  <nuclide name="C13" reactions="2">
    <reaction type="(n,gamma)" Q="8176430.0" target="C14"/>
    <reaction type="(n,a)" Q="-3836080.0" target="Be10"/>
  </nuclide>
  <nuclide name="C14" half_life="179878000000.0" decay_modes="1" decay_energy="49470.0" reactions="0">
    <decay type="beta-" target="N14" branching_ratio="1.0"/>
    <source type="discrete" particle="electron">
      <parameters>156475.0 3.853429438619205e-12</parameters>
    </source>
  </nuclide>
  <nuclide name="C15" half_life="2.449" decay_modes="1" decay_energy="6352186.0" reactions="0">
    <decay type="beta-" target="N15" branching_ratio="1.0"/>
    <source type="discrete" particle="photon">
      <parameters>977020.0 1011750.0 1157520.0 1416280.0 1748770.0 1884770.0 1988700.0 2001860.0 2030530.0 2247440.0 2725660.0 3013470.0 3042130.0 3300850.0 3779040.0 5269161.0 5297817.0 6322350.0 7298920.0 8310150.0 8568770.0 9046780.0 3.3963928814697196e-08 5.09458932220458e-06 1.4151637006123833e-06 1.3302538785756402e-06 1.1604342345021542e-06 2.8303274012247666e-06 5.09458932220458e-06 5.377622062327056e-08 1.6415898927103643e-07 5.09458932220458e-07 4.24549110183715e-06 1.1604342345021542e-05 1.9812291808573364e-06 2.3774750170288037e-05 3.3963928814697197e-06 1.0472211384531636e-05 0.17887669175740525 1.5566800706736215e-05 2.688811031163528e-05 9.057047683919253e-05 1.2170407825266496e-05 8.774014943796776e-05</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>721990.2 1200300.0 1459080.0 2470870.0 3447920.0 4472878.0 9771700.0 9.623113164164207e-05 3.679425621592196e-05 0.00011604342345021542 2.0944422769063272e-05 5.660654802449533e-06 0.17887669175740525 0.1041560483650714</parameters>
    </source>
  </nuclide>
  <nuclide name="C16" half_life="0.747" decay_modes="2" decay_energy="4153021.0" reactions="0">
    <decay type="beta-" target="N16" branching_ratio="0.01"/>
    <decay type="beta-,n" target="N15" branching_ratio="0.99"/>
  </nuclide>
  <nuclide name="C17" half_life="0.193" decay_modes="2" decay_energy="7714972.8" reactions="0">
    <decay type="beta-" target="N17" branching_ratio="0.68"/>
    <decay type="beta-,n" target="N16" branching_ratio="0.32"/>
  </nuclide>
  <nuclide name="C18" half_life="0.092" decay_modes="2" decay_energy="7516363.9" reactions="0">
    <decay type="beta-" target="N18" branching_ratio="0.685"/>
    <decay type="beta-,n" target="N17" branching_ratio="0.315"/>
  </nuclide>
  <nuclide name="C19" half_life="0.049" decay_modes="2" decay_energy="9444803.0" reactions="0">
    <decay type="beta-" target="N19" branching_ratio="0.39"/>
    <decay type="beta-,n" target="N18" branching_ratio="0.61"/>
  </nuclide>
  <nuclide name="C20" half_life="0.0145" decay_modes="2" decay_energy="10304979.0" reactions="0">
    <decay type="beta-" target="N20" branching_ratio="0.28"/>
    <decay type="beta-,n" target="N19" branching_ratio="0.72"/>
  </nuclide>
  <nuclide name="C21" half_life="3e-08" decay_modes="1" decay_energy="331000.0" reactions="0">
    <decay type="n" target="C20" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="C22" half_life="0.0062" decay_modes="3" decay_energy="13610768.0" reactions="0">
    <decay type="beta-" target="N22" branching_ratio="0.02"/>
    <decay type="beta-,n" target="N21" branching_ratio="0.61"/>
    <decay type="beta-,n,n" target="N20" branching_ratio="0.37"/>
  </nuclide>
</nuclides>
  "#;

        let nuclides: SerdeNuclideVec = serde_xml_rs::from_str(xml).unwrap();
        use crate::decay_xml_info_serde::carbon::get_carbon_xml_serde_data;
        assert_eq!(nuclides,get_carbon_xml_serde_data());

    }
}


use crate::decay_xml_info_serde::SerdeNuclideVec;
pub fn get_carbon_xml_serde_data() -> SerdeNuclideVec {
    
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
