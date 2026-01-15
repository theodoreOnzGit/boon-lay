

// notes, code was done from vibe coding using AI, then 
// modified


#[cfg(test)]
mod parsing_tests {
    use crate::prelude::magnesium::get_magnesium_xml_serde_data;

#[test]
fn serde_nuclide_test() {
    use crate::decay_xml_info_serde::SerdeNuclideVec;
    let xml = r#"
    <nuclides>
            <nuclide name="Mg19" half_life="4e-12" decay_modes="1" decay_energy="750000.0" reactions="0">
    <decay type="p,p" target="Ne17" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Mg20" half_life="0.0908" decay_modes="2" decay_energy="6938791.0" reactions="0">
    <decay type="ec/beta+" target="Na20" branching_ratio="0.73"/>
    <decay type="ec/beta+,p" target="Ne19" branching_ratio="0.27"/>
  </nuclide>
  <nuclide name="Mg21" half_life="0.122" decay_modes="3" decay_energy="8473187.0" reactions="0">
    <decay type="ec/beta+" target="Na21" branching_ratio="0.669"/>
    <decay type="ec/beta+,p" target="Ne20" branching_ratio="0.326"/>
    <decay type="ec/beta+,alpha" target="F17" branching_ratio="0.005"/>
  </nuclide>
  <nuclide name="Mg22" half_life="3.8755" decay_modes="1" decay_energy="3083365.0" reactions="0">
    <decay type="ec/beta+" target="Na22" branching_ratio="1.0"/>
    <source type="discrete" particle="photon">
      <parameters>1041.0 73920.0 510998.9 582040.0 1278820.0 1353000.0 1936000.0 7.651328984855005e-06 0.10434555920047876 0.3573852912385196 0.1787963807153054 0.009655004558626492 2.681945710729581e-05 5.721484182889773e-05</parameters>
    </source>
    <source type="discrete" particle="positron">
      <parameters>2845150.0 4124440.0 4198490.0 0.009747521956010068 0.09518589330254235 0.07392019861319195</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>990.0 72847.9 73920.0 580967.9 582040.0 0.0004894350606033949 0.000352688058777406 2.1759180784307994e-05 1.2533626645850136e-05 7.677337787957427e-07</parameters>
    </source>
  </nuclide>
  <nuclide name="Mg23" half_life="11.317" decay_modes="1" decay_energy="2394868.0" reactions="0">
    <decay type="ec/beta+" target="Na23" branching_ratio="1.0"/>
    <source type="discrete" particle="photon">
      <parameters>1041.0 439986.0 510998.9 1950652.0 2390598.0 6.776060331184608e-07 0.005328603402731752 0.12238197310222518 1.5312078743482047e-06 2.69492585885284e-06</parameters>
    </source>
    <source type="discrete" particle="positron">
      <parameters>1665368.0 3616109.0 4056100.0 4.287382048174972e-06 0.005328603402731752 0.055919711571196434</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>990.0 438913.9 439986.0 1949580.0 1950652.0 4.334462402929132e-05 2.728244942198657e-07 1.672488750015415e-08 5.451100032679609e-12 3.33466450875552e-13</parameters>
    </source>
  </nuclide>
  <nuclide name="Mg24" reactions="4">
    <reaction type="(n,2n)" Q="-16530600.0" target="Mg23"/>
    <reaction type="(n,gamma)" Q="7331890.0" target="Mg25"/>
    <reaction type="(n,p)" Q="-4732090.0" target="Na24"/>
    <reaction type="(n,a)" Q="-2553350.0" target="Ne21"/>
  </nuclide>
  <nuclide name="Mg25" reactions="4">
    <reaction type="(n,2n)" Q="-7331880.0" target="Mg24"/>
    <reaction type="(n,gamma)" Q="11093300.0" target="Mg26"/>
    <reaction type="(n,p)" Q="-3053030.0" target="Na25"/>
    <reaction type="(n,a)" Q="480349.0" target="Ne22"/>
  </nuclide>
  <nuclide name="Mg26" reactions="4">
    <reaction type="(n,2n)" Q="-11093600.0" target="Mg25"/>
    <reaction type="(n,gamma)" Q="6444290.0" target="Mg27"/>
    <reaction type="(n,p)" Q="-7920920.0" target="Na26"/>
    <reaction type="(n,a)" Q="-5416640.0" target="Ne23"/>
  </nuclide>
  <nuclide name="Mg27" half_life="567.48" decay_modes="1" decay_energy="1593053.4" reactions="0">
    <decay type="beta-" target="Al27" branching_ratio="1.0"/>
    <source type="discrete" particle="photon">
      <parameters>170686.0 843760.0 1014440.0 9.771582160568763e-06 0.0008769994989110465 0.00034200537561990675</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>1595590.0 1766290.0 0.0003542198533206177 0.0008672279167504778</parameters>
    </source>
  </nuclide>
  <nuclide name="Mg28" half_life="75294.0" decay_modes="1" decay_energy="1531039.7" reactions="0">
    <decay type="beta-" target="Al28" branching_ratio="1.0"/>
    <source type="discrete" particle="photon">
      <parameters>44.84818 114.1751 1468.7 1469.17 1545.02 1545.03 30640.0 400690.0 607100.0 941450.0 982900.0 1013500.0 1342250.0 1372890.0 1589360.0 1620000.0 3.686902455770247e-10 7.676875217143679e-12 3.024740410550816e-08 6.004698840335119e-08 1.848808587060182e-10 3.6699710097787227e-10 6.075877748154752e-06 3.3693503876130897e-06 9.205875375992049e-10 3.5258502690049546e-06 1.8411750751984097e-09 6.444112763194433e-10 4.842290447771817e-06 4.3267614267162626e-07 3.8664676579166606e-07 2.7617626127976143e-08</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>28.50343 65.33955 1371.458 1447.427 1530.974 29080.4 30640.0 211750.0 458920.0 859630.1 1.1465337810649407e-06 4.831885046834333e-06 2.2284331111150415e-06 1.1521843473707249e-07 1.9288499911042826e-09 2.4364269770100556e-06 1.8233101534437596e-07 4.602937687996024e-07 8.745581607192446e-06 2.8538213665575348e-08</parameters>
    </source>
  </nuclide>
  <nuclide name="Mg29" half_life="1.3" decay_modes="1" decay_energy="4527162.0" reactions="0">
    <decay type="beta-" target="Al29" branching_ratio="1.0"/>
    <source type="discrete" particle="photon">
      <parameters>960400.0 1307400.0 1398000.0 1430200.0 1467500.0 1754200.0 1786400.0 2034900.0 2224000.0 2865500.0 3061600.0 3184400.0 0.07997852083383984 0.025059936527936484 0.08744318277833156 0.0373233097224586 0.01919484500012156 0.0527858237503343 0.014929323888983438 0.013329753472306641 0.19194845000121563 0.02186079569458289 0.007997852083383983 0.005865091527814921</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>3209900.0 3393400.0 3556000.0 3677800.0 3941300.0 3971500.0 4035400.0 4180000.0 4428460.0 4551300.0 4747400.0 5388900.0 5858730.0 6214950.0 7613000.0 0.001332975347230664 0.0005331901388922656 0.002665950694461328 0.0007997852083383984 0.002399355625015195 0.0009330827430614649 0.0015995704166767968 0.015995704166767967 0.1492932388898344 0.031991408333535934 0.04158883083359672 0.11196992916737578 0.007997852083383983 0.0373233097224586 0.14396133750091172</parameters>
    </source>
  </nuclide>
  <nuclide name="Mg30" half_life="0.335" decay_modes="1" decay_energy="4653606.0" reactions="0">
    <decay type="beta-" target="Al30" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Mg31" half_life="0.232" decay_modes="2" decay_energy="7768182.81" reactions="0">
    <decay type="beta-" target="Al31" branching_ratio="0.983"/>
    <decay type="beta-,n" target="Al30" branching_ratio="0.017"/>
  </nuclide>
  <nuclide name="Mg32" half_life="0.086" decay_modes="2" decay_energy="6639189.53" reactions="0">
    <decay type="beta-" target="Al32" branching_ratio="0.945"/>
    <decay type="beta-,n" target="Al31" branching_ratio="0.055"/>
  </nuclide>
  <nuclide name="Mg33" half_life="0.0905" decay_modes="2" decay_energy="8411326.6" reactions="0">
    <decay type="beta-" target="Al33" branching_ratio="0.83"/>
    <decay type="beta-,n" target="Al32" branching_ratio="0.17"/>
  </nuclide>
  <nuclide name="Mg34" half_life="0.02" decay_modes="1" decay_energy="7738372.0" reactions="0">
    <decay type="beta-" target="Al34" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Mg35" half_life="0.07" decay_modes="2" decay_energy="9370148.0" reactions="0">
    <decay type="beta-" target="Al35" branching_ratio="0.48"/>
    <decay type="beta-,n" target="Al34" branching_ratio="0.52"/>
  </nuclide>
  <nuclide name="Mg36" half_life="0.0039" decay_modes="1" decay_energy="9619850.0" reactions="0">
    <decay type="beta-" target="Al36" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Mg37" half_life="2.6e-07" decay_modes="1" decay_energy="12320000.0" reactions="0">
    <decay type="beta-" target="Al37" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Mg38" half_life="2.6e-07" decay_modes="2" decay_energy="12321791.0" reactions="0">
    <decay type="beta-" target="Al38" branching_ratio="0.5"/>
    <decay type="beta-,n" target="Al37" branching_ratio="0.5"/>
  </nuclide>
  <nuclide name="Mg39" half_life="1.8e-07" decay_modes="1" decay_energy="130000.0" reactions="0">
    <decay type="n" target="Mg38" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Mg40" half_life="1.7e-07" decay_modes="2" decay_energy="14665666.0" reactions="0">
    <decay type="beta-,n" target="Al39" branching_ratio="0.5"/>
    <decay type="beta-" target="Al40" branching_ratio="0.5"/>
  </nuclide>
</nuclides>
  "#;

        let nuclides: SerdeNuclideVec = serde_xml_rs::from_str(xml).unwrap();
        assert_eq!(nuclides,get_magnesium_xml_serde_data());
}
}
use crate::decay_xml_info_serde::SerdeNuclideVec;
pub fn get_magnesium_xml_serde_data() -> SerdeNuclideVec {
    let xml = r#"
    <nuclides>
            <nuclide name="Mg19" half_life="4e-12" decay_modes="1" decay_energy="750000.0" reactions="0">
    <decay type="p,p" target="Ne17" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Mg20" half_life="0.0908" decay_modes="2" decay_energy="6938791.0" reactions="0">
    <decay type="ec/beta+" target="Na20" branching_ratio="0.73"/>
    <decay type="ec/beta+,p" target="Ne19" branching_ratio="0.27"/>
  </nuclide>
  <nuclide name="Mg21" half_life="0.122" decay_modes="3" decay_energy="8473187.0" reactions="0">
    <decay type="ec/beta+" target="Na21" branching_ratio="0.669"/>
    <decay type="ec/beta+,p" target="Ne20" branching_ratio="0.326"/>
    <decay type="ec/beta+,alpha" target="F17" branching_ratio="0.005"/>
  </nuclide>
  <nuclide name="Mg22" half_life="3.8755" decay_modes="1" decay_energy="3083365.0" reactions="0">
    <decay type="ec/beta+" target="Na22" branching_ratio="1.0"/>
    <source type="discrete" particle="photon">
      <parameters>1041.0 73920.0 510998.9 582040.0 1278820.0 1353000.0 1936000.0 7.651328984855005e-06 0.10434555920047876 0.3573852912385196 0.1787963807153054 0.009655004558626492 2.681945710729581e-05 5.721484182889773e-05</parameters>
    </source>
    <source type="discrete" particle="positron">
      <parameters>2845150.0 4124440.0 4198490.0 0.009747521956010068 0.09518589330254235 0.07392019861319195</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>990.0 72847.9 73920.0 580967.9 582040.0 0.0004894350606033949 0.000352688058777406 2.1759180784307994e-05 1.2533626645850136e-05 7.677337787957427e-07</parameters>
    </source>
  </nuclide>
  <nuclide name="Mg23" half_life="11.317" decay_modes="1" decay_energy="2394868.0" reactions="0">
    <decay type="ec/beta+" target="Na23" branching_ratio="1.0"/>
    <source type="discrete" particle="photon">
      <parameters>1041.0 439986.0 510998.9 1950652.0 2390598.0 6.776060331184608e-07 0.005328603402731752 0.12238197310222518 1.5312078743482047e-06 2.69492585885284e-06</parameters>
    </source>
    <source type="discrete" particle="positron">
      <parameters>1665368.0 3616109.0 4056100.0 4.287382048174972e-06 0.005328603402731752 0.055919711571196434</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>990.0 438913.9 439986.0 1949580.0 1950652.0 4.334462402929132e-05 2.728244942198657e-07 1.672488750015415e-08 5.451100032679609e-12 3.33466450875552e-13</parameters>
    </source>
  </nuclide>
  <nuclide name="Mg24" reactions="4">
    <reaction type="(n,2n)" Q="-16530600.0" target="Mg23"/>
    <reaction type="(n,gamma)" Q="7331890.0" target="Mg25"/>
    <reaction type="(n,p)" Q="-4732090.0" target="Na24"/>
    <reaction type="(n,a)" Q="-2553350.0" target="Ne21"/>
  </nuclide>
  <nuclide name="Mg25" reactions="4">
    <reaction type="(n,2n)" Q="-7331880.0" target="Mg24"/>
    <reaction type="(n,gamma)" Q="11093300.0" target="Mg26"/>
    <reaction type="(n,p)" Q="-3053030.0" target="Na25"/>
    <reaction type="(n,a)" Q="480349.0" target="Ne22"/>
  </nuclide>
  <nuclide name="Mg26" reactions="4">
    <reaction type="(n,2n)" Q="-11093600.0" target="Mg25"/>
    <reaction type="(n,gamma)" Q="6444290.0" target="Mg27"/>
    <reaction type="(n,p)" Q="-7920920.0" target="Na26"/>
    <reaction type="(n,a)" Q="-5416640.0" target="Ne23"/>
  </nuclide>
  <nuclide name="Mg27" half_life="567.48" decay_modes="1" decay_energy="1593053.4" reactions="0">
    <decay type="beta-" target="Al27" branching_ratio="1.0"/>
    <source type="discrete" particle="photon">
      <parameters>170686.0 843760.0 1014440.0 9.771582160568763e-06 0.0008769994989110465 0.00034200537561990675</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>1595590.0 1766290.0 0.0003542198533206177 0.0008672279167504778</parameters>
    </source>
  </nuclide>
  <nuclide name="Mg28" half_life="75294.0" decay_modes="1" decay_energy="1531039.7" reactions="0">
    <decay type="beta-" target="Al28" branching_ratio="1.0"/>
    <source type="discrete" particle="photon">
      <parameters>44.84818 114.1751 1468.7 1469.17 1545.02 1545.03 30640.0 400690.0 607100.0 941450.0 982900.0 1013500.0 1342250.0 1372890.0 1589360.0 1620000.0 3.686902455770247e-10 7.676875217143679e-12 3.024740410550816e-08 6.004698840335119e-08 1.848808587060182e-10 3.6699710097787227e-10 6.075877748154752e-06 3.3693503876130897e-06 9.205875375992049e-10 3.5258502690049546e-06 1.8411750751984097e-09 6.444112763194433e-10 4.842290447771817e-06 4.3267614267162626e-07 3.8664676579166606e-07 2.7617626127976143e-08</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>28.50343 65.33955 1371.458 1447.427 1530.974 29080.4 30640.0 211750.0 458920.0 859630.1 1.1465337810649407e-06 4.831885046834333e-06 2.2284331111150415e-06 1.1521843473707249e-07 1.9288499911042826e-09 2.4364269770100556e-06 1.8233101534437596e-07 4.602937687996024e-07 8.745581607192446e-06 2.8538213665575348e-08</parameters>
    </source>
  </nuclide>
  <nuclide name="Mg29" half_life="1.3" decay_modes="1" decay_energy="4527162.0" reactions="0">
    <decay type="beta-" target="Al29" branching_ratio="1.0"/>
    <source type="discrete" particle="photon">
      <parameters>960400.0 1307400.0 1398000.0 1430200.0 1467500.0 1754200.0 1786400.0 2034900.0 2224000.0 2865500.0 3061600.0 3184400.0 0.07997852083383984 0.025059936527936484 0.08744318277833156 0.0373233097224586 0.01919484500012156 0.0527858237503343 0.014929323888983438 0.013329753472306641 0.19194845000121563 0.02186079569458289 0.007997852083383983 0.005865091527814921</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>3209900.0 3393400.0 3556000.0 3677800.0 3941300.0 3971500.0 4035400.0 4180000.0 4428460.0 4551300.0 4747400.0 5388900.0 5858730.0 6214950.0 7613000.0 0.001332975347230664 0.0005331901388922656 0.002665950694461328 0.0007997852083383984 0.002399355625015195 0.0009330827430614649 0.0015995704166767968 0.015995704166767967 0.1492932388898344 0.031991408333535934 0.04158883083359672 0.11196992916737578 0.007997852083383983 0.0373233097224586 0.14396133750091172</parameters>
    </source>
  </nuclide>
  <nuclide name="Mg30" half_life="0.335" decay_modes="1" decay_energy="4653606.0" reactions="0">
    <decay type="beta-" target="Al30" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Mg31" half_life="0.232" decay_modes="2" decay_energy="7768182.81" reactions="0">
    <decay type="beta-" target="Al31" branching_ratio="0.983"/>
    <decay type="beta-,n" target="Al30" branching_ratio="0.017"/>
  </nuclide>
  <nuclide name="Mg32" half_life="0.086" decay_modes="2" decay_energy="6639189.53" reactions="0">
    <decay type="beta-" target="Al32" branching_ratio="0.945"/>
    <decay type="beta-,n" target="Al31" branching_ratio="0.055"/>
  </nuclide>
  <nuclide name="Mg33" half_life="0.0905" decay_modes="2" decay_energy="8411326.6" reactions="0">
    <decay type="beta-" target="Al33" branching_ratio="0.83"/>
    <decay type="beta-,n" target="Al32" branching_ratio="0.17"/>
  </nuclide>
  <nuclide name="Mg34" half_life="0.02" decay_modes="1" decay_energy="7738372.0" reactions="0">
    <decay type="beta-" target="Al34" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Mg35" half_life="0.07" decay_modes="2" decay_energy="9370148.0" reactions="0">
    <decay type="beta-" target="Al35" branching_ratio="0.48"/>
    <decay type="beta-,n" target="Al34" branching_ratio="0.52"/>
  </nuclide>
  <nuclide name="Mg36" half_life="0.0039" decay_modes="1" decay_energy="9619850.0" reactions="0">
    <decay type="beta-" target="Al36" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Mg37" half_life="2.6e-07" decay_modes="1" decay_energy="12320000.0" reactions="0">
    <decay type="beta-" target="Al37" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Mg38" half_life="2.6e-07" decay_modes="2" decay_energy="12321791.0" reactions="0">
    <decay type="beta-" target="Al38" branching_ratio="0.5"/>
    <decay type="beta-,n" target="Al37" branching_ratio="0.5"/>
  </nuclide>
  <nuclide name="Mg39" half_life="1.8e-07" decay_modes="1" decay_energy="130000.0" reactions="0">
    <decay type="n" target="Mg38" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Mg40" half_life="1.7e-07" decay_modes="2" decay_energy="14665666.0" reactions="0">
    <decay type="beta-,n" target="Al39" branching_ratio="0.5"/>
    <decay type="beta-" target="Al40" branching_ratio="0.5"/>
  </nuclide>
</nuclides>
  "#;

    let nuclides: SerdeNuclideVec = serde_xml_rs::from_str(xml).unwrap();

    return nuclides;
}
