
// notes, code was done from vibe coding using AI, then 
// modified


#[cfg(test)]
mod parsing_tests {
#[test]
fn serde_nuclide_test() {
    use crate::decay_xml_info_serde::SerdeNuclideVec;
    let xml = r#"
    <nuclides>
           <nuclide name="B6" half_life="1e-09" decay_modes="1" decay_energy="7419000.0" reactions="0">
    <decay type="p,p" target="Li4" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="B7" half_life="3.255e-22" decay_modes="1" decay_energy="2206640.0" reactions="0">
    <decay type="p" target="Be6" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="B8" half_life="0.77" decay_modes="1" decay_energy="13553808.0" reactions="0">
    <decay type="ec/beta+,alpha" target="He4" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="B9" half_life="8.43888e-19" decay_modes="1" decay_energy="185830.0" reactions="0">
    <decay type="p" target="Be8" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="B10" reactions="3">
    <reaction type="(n,gamma)" Q="11456000.0" target="B11"/>
    <reaction type="(n,p)" Q="226241.0" target="Be10"/>
    <reaction type="(n,a)" Q="2789323.0" target="Li7"/>
  </nuclide>
  <nuclide name="B11" reactions="4">
    <reaction type="(n,2n)" Q="-11454100.0" target="B10"/>
    <reaction type="(n,gamma)" Q="3369870.0" target="B12"/>
    <reaction type="(n,p)" Q="-10723700.0" target="Be11"/>
    <reaction type="(n,a)" Q="-6630950.0" target="Li8"/>
  </nuclide>
  <nuclide name="B12" half_life="0.0202" decay_modes="1" decay_energy="6406833.76" reactions="0">
    <decay type="beta-" target="C12" branching_ratio="1.0"/>
    <source type="discrete" particle="photon">
      <parameters>277.0 4438030.0 1.870759631849375e-10 0.42206452422849117</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>260.0 3068900.0 4437746.0 4438020.0 5714600.0 8929990.0 13368900.0 6.667924894432492e-08 0.027451373487522586 6.681286850477544e-08 3.0599702884372833e-09 0.5147132528910484 0.42206486737065974 33.36028163071182</parameters>
    </source>
  </nuclide>
  <nuclide name="B13" half_life="0.01736" decay_modes="1" decay_energy="6678216.8" reactions="0">
    <decay type="beta-" target="C13" branching_ratio="1.0"/>
    <source type="discrete" particle="photon">
      <parameters>277.0 169300.0 595013.0 764316.0 3089049.0 3683921.0 3853170.0 7545000.0 8857000.0 9893000.0 3.6309253334110514e-09 0.0017967524841703652 0.02275886479949129 0.059891749472345504 0.13974741543547284 3.0345153065988386 0.09981958245390918 0.037532163002669844 0.06388453277050188 0.008784123255944007</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>260.0 169016.2 169289.7 594729.2 595002.8 764032.2 764305.8 3088766.0 3089039.0 3540200.0 3683637.0 3683911.0 3852886.0 3853160.0 4577200.0 5890200.0 9583394.0 9752693.0 10347760.0 13437200.0 1.2941664764115498e-06 2.5190469828068514e-07 1.1535150948373744e-08 7.806290626225513e-08 3.575417660000082e-09 3.084425097825793e-07 1.4134452875473539e-08 2.7390493425352676e-08 1.2549317906105462e-09 0.008784123255944007 6.069030613197678e-07 2.7765815055379374e-08 2.405651937139211e-08 1.1000117986420792e-09 0.06388453277050188 0.037532163002669844 0.13974741543547284 3.0345153065988386 0.13974741543547284 36.77353417602014</parameters>
    </source>
  </nuclide>
  <nuclide name="B14" half_life="0.0125" decay_modes="1" decay_energy="13021020.0" reactions="0">
    <decay type="beta-" target="C14" branching_ratio="1.0"/>
    <source type="discrete" particle="photon">
      <parameters>613000.0 634400.0 1248000.0 6092400.0 6726500.0 7339000.0 1.0535837144511166 0.17744567822334598 1.5526496844542774 47.68852602252423 4.768852602252423 0.5267918572255583</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>13303000.0 13915800.0 14550200.0 20644000.0 3.049847594463759 3.881624211135694 45.470455044732404 2.772588722239781</parameters>
    </source>
  </nuclide>
  <nuclide name="B15" half_life="0.00993" decay_modes="3" decay_energy="13345235.0" reactions="0">
    <decay type="beta-" target="C15" branching_ratio="0.06"/>
    <decay type="beta-,n" target="C14" branching_ratio="0.936"/>
    <decay type="beta-,n,n" target="C13" branching_ratio="0.004"/>
  </nuclide>
  <nuclide name="B16" half_life="1.9e-10" decay_modes="1" decay_energy="85000.0" reactions="0">
    <decay type="n" target="B15" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="B17" half_life="0.00508" decay_modes="5" decay_energy="15821076.0" reactions="0">
    <decay type="beta-" target="C17" branching_ratio="0.221"/>
    <decay type="beta-,n" target="C16" branching_ratio="0.63"/>
    <decay type="beta-,n,n" target="C15" branching_ratio="0.11"/>
    <decay type="beta-,n,n,n" target="C14" branching_ratio="0.035"/>
    <decay type="beta-,n,n,n,n" target="C13" branching_ratio="0.004"/>
  </nuclide>
  <nuclide name="B18" half_life="2.6e-08" decay_modes="1" decay_energy="480000.0" reactions="0">
    <decay type="n" target="B17" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="B19" half_life="0.00292" decay_modes="3" decay_energy="19112292.0" reactions="0">
    <decay type="beta-,n" target="C18" branching_ratio="0.72"/>
    <decay type="beta-" target="C19" branching_ratio="0.12"/>
    <decay type="beta-,n,n" target="C17" branching_ratio="0.16"/>
  </nuclide>
</nuclides>
  "#;

    let nuclides: SerdeNuclideVec = serde_xml_rs::from_str(xml).unwrap();
    dbg!("{:#?}", nuclides);
}
}
