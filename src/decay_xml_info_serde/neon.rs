
// notes, code was done from vibe coding using AI, then 
// modified


#[cfg(test)]
mod parsing_tests {
#[test]
fn serde_nuclide_test() {
    use crate::decay_xml_info_serde::SerdeNuclideVec;
    let xml = r#"
    <nuclides>
          <nuclide name="Ne16" half_life="3.73524e-21" decay_modes="1" decay_energy="1411000.0" reactions="0">
    <decay type="p,p" target="O14" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Ne17" half_life="0.1092" decay_modes="1" decay_energy="10431696.0" reactions="0">
    <decay type="ec/beta+,p" target="O16" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Ne18" half_life="1.672" decay_modes="1" decay_energy="2604134.0" reactions="0">
    <decay type="ec/beta+" target="F18" branching_ratio="1.0"/>
    <source type="discrete" particle="photon">
      <parameters>677.0 510998.9 659250.0 1041520.0 1080510.0 1700720.0 1.6630619777164982e-06 0.8287926014771553 0.0005596583096626353 0.03246018196043284 9.369094665463375e-06 0.00022303420044333166</parameters>
    </source>
    <source type="discrete" particle="positron">
      <parameters>2744890.0 3365160.0 3404150.0 4445700.0 0.0007793760164190772 8.705795928085437e-06 0.03192125173631327 0.38185279187426174</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>660.0 0.00012391460964114145</parameters>
    </source>
  </nuclide>
  <nuclide name="Ne19" half_life="17.22" decay_modes="1" decay_energy="1983181.9" reactions="0">
    <decay type="ec/beta+" target="F19" branching_ratio="1.0"/>
    <source type="discrete" particle="photon">
      <parameters>677.0 109894.0 197142.0 510998.9 1356843.0 1444085.0 1553970.0 5.041796411888944e-07 4.830293941184288e-06 8.292004599033027e-07 0.08042612497604731 8.292004599033027e-07 4.347264547065859e-08 2.2943896220625368e-08</parameters>
    </source>
    <source type="discrete" particle="positron">
      <parameters>1684362.0 3128506.0 3238400.0 8.936043791190933e-07 4.830293941184288e-06 0.040247619215927885</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>660.0 3.75663955035856e-05</parameters>
    </source>
  </nuclide>
  <nuclide name="Ne20" reactions="4">
    <reaction type="(n,2n)" Q="-16957400.0" target="Ne19"/>
    <reaction type="(n,gamma)" Q="6669040.0" target="Ne21"/>
    <reaction type="(n,p)" Q="-6334240.0" target="F20"/>
    <reaction type="(n,a)" Q="-603283.0" target="O17"/>
  </nuclide>
  <nuclide name="Ne21" reactions="5">
    <reaction type="(n,2n)" Q="-6669040.0" target="Ne20"/>
    <reaction type="(n,3n)" Q="-23626500.0" target="Ne19"/>
    <reaction type="(n,gamma)" Q="10364300.0" target="Ne22"/>
    <reaction type="(n,p)" Q="-4901810.0" target="F21"/>
    <reaction type="(n,a)" Q="778193.0" target="O18"/>
  </nuclide>
  <nuclide name="Ne22" reactions="5">
    <reaction type="(n,2n)" Q="-10364300.0" target="Ne21"/>
    <reaction type="(n,3n)" Q="-17033300.0" target="Ne20"/>
    <reaction type="(n,gamma)" Q="5200650.0" target="Ne23"/>
    <reaction type="(n,p)" Q="-10035700.0" target="F22"/>
    <reaction type="(n,a)" Q="-5711160.0" target="O19"/>
  </nuclide>
  <nuclide name="Ne23" half_life="37.24" decay_modes="1" decay_energy="2066016.1" reactions="0">
    <decay type="beta-" target="Na23" branching_ratio="1.0"/>
    <source type="discrete" particle="photon">
      <parameters>1041.0 439986.0 1635960.0 2075910.0 2541920.0 2981850.0 4.824849736858078e-09 0.006142281675208967 0.00018612974773360506 1.8799104521094112e-05 5.0255031888073364e-06 7.072930413876992e-06</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>990.0 438913.9 439986.0 1393750.0 1634888.0 1635960.0 2074838.0 2075910.0 2299800.0 2540848.0 2541920.0 2980778.0 2981850.0 3935819.0 4375810.0 3.086327163722068e-07 3.144848217706991e-07 1.9278779493978384e-08 1.2098433602684328e-05 7.817449404811412e-10 4.795260690860867e-11 6.109708969355586e-11 3.747789477325321e-12 0.00020474272250696555 1.045304663271926e-11 6.386912002655244e-13 1.1578387087516636e-11 7.094149205118624e-13 0.005956151927475362 0.012452080123378178</parameters>
    </source>
  </nuclide>
  <nuclide name="Ne24" half_life="202.8" decay_modes="1" decay_energy="872557.39" reactions="0">
    <decay type="beta-" target="Na24_m1" branching_ratio="1.0"/>
    <source type="discrete" particle="photon">
      <parameters>874410.0 0.00027001295495185243</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>1119970.0 1994393.0 0.00027001295495185243 0.003147872550767799</parameters>
    </source>
  </nuclide>
  <nuclide name="Ne25" half_life="0.602" decay_modes="1" decay_energy="3588564.9" reactions="0">
    <decay type="beta-" target="Na25" branching_ratio="1.0"/>
    <source type="discrete" particle="photon">
      <parameters>89530.0 979770.0 1069300.0 1132800.0 2112500.0 2202000.0 3220000.0 3599000.0 3688000.0 1.098442541950478 0.20840471707865463 0.02694293027425701 0.0046056291067106 0.007138725115401429 0.01266548004345415 0.006102458566391545 0.00253309600869083 0.01105350985610544</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>2961000.0 3563000.0 5048000.0 6180680.0 7160470.0 0.006102458566391545 0.0138168873201318 0.02417955281023065 0.22452441895214176 0.8819779739350799</parameters>
    </source>
  </nuclide>
  <nuclide name="Ne26" half_life="0.197" decay_modes="2" decay_energy="3484880.54" reactions="0">
    <decay type="beta-" target="Na26" branching_ratio="0.9987"/>
    <decay type="beta-,n" target="Na25" branching_ratio="0.0013"/>
    <source type="discrete" particle="photon">
      <parameters>1041.0 82500.0 151100.0 233600.0 0.005905529534044168 2.9805589134084554 0.12842574665196954 0.12842574665196954</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>990.0 81427.9 82500.0 7106400.0 7257500.0 0.3777609911888418 0.3859823500827321 0.02372763484073409 0.2568514933039391 3.2616621135993364</parameters>
    </source>
  </nuclide>
  <nuclide name="Ne27" half_life="0.032" decay_modes="2" decay_energy="8289504.75" reactions="0">
    <decay type="beta-" target="Na27" branching_ratio="0.98"/>
    <decay type="beta-,n" target="Na26" branching_ratio="0.02"/>
  </nuclide>
  <nuclide name="Ne28" half_life="0.0189" decay_modes="3" decay_energy="7724240.0" reactions="0">
    <decay type="beta-" target="Na28" branching_ratio="0.845"/>
    <decay type="beta-,n" target="Na27" branching_ratio="0.119"/>
    <decay type="beta-,n,n" target="Na26" branching_ratio="0.036"/>
  </nuclide>
  <nuclide name="Ne29" half_life="0.0148" decay_modes="3" decay_energy="9715809.0" reactions="0">
    <decay type="beta-" target="Na29" branching_ratio="0.68"/>
    <decay type="beta-,n" target="Na28" branching_ratio="0.28"/>
    <decay type="beta-,n,n" target="Na27" branching_ratio="0.04"/>
  </nuclide>
  <nuclide name="Ne30" half_life="0.0073" decay_modes="3" decay_energy="9473072.2" reactions="0">
    <decay type="beta-" target="Na30" branching_ratio="0.781"/>
    <decay type="beta-,n" target="Na29" branching_ratio="0.13"/>
    <decay type="beta-,n,n" target="Na28" branching_ratio="0.089"/>
  </nuclide>
  <nuclide name="Ne31" half_life="0.0034" decay_modes="1" decay_energy="12187174.0" reactions="0">
    <decay type="beta-" target="Na31" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Ne32" half_life="0.0035" decay_modes="1" decay_energy="12126000.0" reactions="0">
    <decay type="beta-" target="Na32" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Ne33" half_life="1.8e-07" decay_modes="1" decay_energy="927000.0" reactions="0">
    <decay type="n" target="Ne32" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Ne34" half_life="6e-08" decay_modes="2" decay_energy="14744334.0" reactions="0">
    <decay type="beta-" target="Na34" branching_ratio="0.5"/>
    <decay type="beta-,n" target="Na33" branching_ratio="0.5"/>
  </nuclide>
</nuclides>
  "#;

        let nuclides: SerdeNuclideVec = serde_xml_rs::from_str(xml).unwrap();
        use crate::decay_xml_info_serde::neon::get_neon_xml_serde_data;
        assert_eq!(nuclides,get_neon_xml_serde_data());

    }
}


use crate::decay_xml_info_serde::SerdeNuclideVec;
pub fn get_neon_xml_serde_data() -> SerdeNuclideVec {
    
    let xml = r#"
    <nuclides>
          <nuclide name="Ne16" half_life="3.73524e-21" decay_modes="1" decay_energy="1411000.0" reactions="0">
    <decay type="p,p" target="O14" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Ne17" half_life="0.1092" decay_modes="1" decay_energy="10431696.0" reactions="0">
    <decay type="ec/beta+,p" target="O16" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Ne18" half_life="1.672" decay_modes="1" decay_energy="2604134.0" reactions="0">
    <decay type="ec/beta+" target="F18" branching_ratio="1.0"/>
    <source type="discrete" particle="photon">
      <parameters>677.0 510998.9 659250.0 1041520.0 1080510.0 1700720.0 1.6630619777164982e-06 0.8287926014771553 0.0005596583096626353 0.03246018196043284 9.369094665463375e-06 0.00022303420044333166</parameters>
    </source>
    <source type="discrete" particle="positron">
      <parameters>2744890.0 3365160.0 3404150.0 4445700.0 0.0007793760164190772 8.705795928085437e-06 0.03192125173631327 0.38185279187426174</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>660.0 0.00012391460964114145</parameters>
    </source>
  </nuclide>
  <nuclide name="Ne19" half_life="17.22" decay_modes="1" decay_energy="1983181.9" reactions="0">
    <decay type="ec/beta+" target="F19" branching_ratio="1.0"/>
    <source type="discrete" particle="photon">
      <parameters>677.0 109894.0 197142.0 510998.9 1356843.0 1444085.0 1553970.0 5.041796411888944e-07 4.830293941184288e-06 8.292004599033027e-07 0.08042612497604731 8.292004599033027e-07 4.347264547065859e-08 2.2943896220625368e-08</parameters>
    </source>
    <source type="discrete" particle="positron">
      <parameters>1684362.0 3128506.0 3238400.0 8.936043791190933e-07 4.830293941184288e-06 0.040247619215927885</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>660.0 3.75663955035856e-05</parameters>
    </source>
  </nuclide>
  <nuclide name="Ne20" reactions="4">
    <reaction type="(n,2n)" Q="-16957400.0" target="Ne19"/>
    <reaction type="(n,gamma)" Q="6669040.0" target="Ne21"/>
    <reaction type="(n,p)" Q="-6334240.0" target="F20"/>
    <reaction type="(n,a)" Q="-603283.0" target="O17"/>
  </nuclide>
  <nuclide name="Ne21" reactions="5">
    <reaction type="(n,2n)" Q="-6669040.0" target="Ne20"/>
    <reaction type="(n,3n)" Q="-23626500.0" target="Ne19"/>
    <reaction type="(n,gamma)" Q="10364300.0" target="Ne22"/>
    <reaction type="(n,p)" Q="-4901810.0" target="F21"/>
    <reaction type="(n,a)" Q="778193.0" target="O18"/>
  </nuclide>
  <nuclide name="Ne22" reactions="5">
    <reaction type="(n,2n)" Q="-10364300.0" target="Ne21"/>
    <reaction type="(n,3n)" Q="-17033300.0" target="Ne20"/>
    <reaction type="(n,gamma)" Q="5200650.0" target="Ne23"/>
    <reaction type="(n,p)" Q="-10035700.0" target="F22"/>
    <reaction type="(n,a)" Q="-5711160.0" target="O19"/>
  </nuclide>
  <nuclide name="Ne23" half_life="37.24" decay_modes="1" decay_energy="2066016.1" reactions="0">
    <decay type="beta-" target="Na23" branching_ratio="1.0"/>
    <source type="discrete" particle="photon">
      <parameters>1041.0 439986.0 1635960.0 2075910.0 2541920.0 2981850.0 4.824849736858078e-09 0.006142281675208967 0.00018612974773360506 1.8799104521094112e-05 5.0255031888073364e-06 7.072930413876992e-06</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>990.0 438913.9 439986.0 1393750.0 1634888.0 1635960.0 2074838.0 2075910.0 2299800.0 2540848.0 2541920.0 2980778.0 2981850.0 3935819.0 4375810.0 3.086327163722068e-07 3.144848217706991e-07 1.9278779493978384e-08 1.2098433602684328e-05 7.817449404811412e-10 4.795260690860867e-11 6.109708969355586e-11 3.747789477325321e-12 0.00020474272250696555 1.045304663271926e-11 6.386912002655244e-13 1.1578387087516636e-11 7.094149205118624e-13 0.005956151927475362 0.012452080123378178</parameters>
    </source>
  </nuclide>
  <nuclide name="Ne24" half_life="202.8" decay_modes="1" decay_energy="872557.39" reactions="0">
    <decay type="beta-" target="Na24_m1" branching_ratio="1.0"/>
    <source type="discrete" particle="photon">
      <parameters>874410.0 0.00027001295495185243</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>1119970.0 1994393.0 0.00027001295495185243 0.003147872550767799</parameters>
    </source>
  </nuclide>
  <nuclide name="Ne25" half_life="0.602" decay_modes="1" decay_energy="3588564.9" reactions="0">
    <decay type="beta-" target="Na25" branching_ratio="1.0"/>
    <source type="discrete" particle="photon">
      <parameters>89530.0 979770.0 1069300.0 1132800.0 2112500.0 2202000.0 3220000.0 3599000.0 3688000.0 1.098442541950478 0.20840471707865463 0.02694293027425701 0.0046056291067106 0.007138725115401429 0.01266548004345415 0.006102458566391545 0.00253309600869083 0.01105350985610544</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>2961000.0 3563000.0 5048000.0 6180680.0 7160470.0 0.006102458566391545 0.0138168873201318 0.02417955281023065 0.22452441895214176 0.8819779739350799</parameters>
    </source>
  </nuclide>
  <nuclide name="Ne26" half_life="0.197" decay_modes="2" decay_energy="3484880.54" reactions="0">
    <decay type="beta-" target="Na26" branching_ratio="0.9987"/>
    <decay type="beta-,n" target="Na25" branching_ratio="0.0013"/>
    <source type="discrete" particle="photon">
      <parameters>1041.0 82500.0 151100.0 233600.0 0.005905529534044168 2.9805589134084554 0.12842574665196954 0.12842574665196954</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>990.0 81427.9 82500.0 7106400.0 7257500.0 0.3777609911888418 0.3859823500827321 0.02372763484073409 0.2568514933039391 3.2616621135993364</parameters>
    </source>
  </nuclide>
  <nuclide name="Ne27" half_life="0.032" decay_modes="2" decay_energy="8289504.75" reactions="0">
    <decay type="beta-" target="Na27" branching_ratio="0.98"/>
    <decay type="beta-,n" target="Na26" branching_ratio="0.02"/>
  </nuclide>
  <nuclide name="Ne28" half_life="0.0189" decay_modes="3" decay_energy="7724240.0" reactions="0">
    <decay type="beta-" target="Na28" branching_ratio="0.845"/>
    <decay type="beta-,n" target="Na27" branching_ratio="0.119"/>
    <decay type="beta-,n,n" target="Na26" branching_ratio="0.036"/>
  </nuclide>
  <nuclide name="Ne29" half_life="0.0148" decay_modes="3" decay_energy="9715809.0" reactions="0">
    <decay type="beta-" target="Na29" branching_ratio="0.68"/>
    <decay type="beta-,n" target="Na28" branching_ratio="0.28"/>
    <decay type="beta-,n,n" target="Na27" branching_ratio="0.04"/>
  </nuclide>
  <nuclide name="Ne30" half_life="0.0073" decay_modes="3" decay_energy="9473072.2" reactions="0">
    <decay type="beta-" target="Na30" branching_ratio="0.781"/>
    <decay type="beta-,n" target="Na29" branching_ratio="0.13"/>
    <decay type="beta-,n,n" target="Na28" branching_ratio="0.089"/>
  </nuclide>
  <nuclide name="Ne31" half_life="0.0034" decay_modes="1" decay_energy="12187174.0" reactions="0">
    <decay type="beta-" target="Na31" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Ne32" half_life="0.0035" decay_modes="1" decay_energy="12126000.0" reactions="0">
    <decay type="beta-" target="Na32" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Ne33" half_life="1.8e-07" decay_modes="1" decay_energy="927000.0" reactions="0">
    <decay type="n" target="Ne32" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Ne34" half_life="6e-08" decay_modes="2" decay_energy="14744334.0" reactions="0">
    <decay type="beta-" target="Na34" branching_ratio="0.5"/>
    <decay type="beta-,n" target="Na33" branching_ratio="0.5"/>
  </nuclide>
</nuclides>
  "#;

    let nuclides: SerdeNuclideVec = serde_xml_rs::from_str(xml).unwrap();

    return nuclides;
}
