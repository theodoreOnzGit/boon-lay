
// notes, code was done from vibe coding using AI, then 
// modified


#[cfg(test)]
mod parsing_tests {
#[test]
    fn serde_nuclide_test() {
        use crate::decay_xml_info_serde::SerdeNuclideVec;
        let xml = r#"
    <nuclides>
          <nuclide name="N10" half_life="2e-22" decay_modes="1" decay_energy="2601730.0" reactions="0">
    <decay type="p" target="C9" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="N11" half_life="3.12742e-22" decay_modes="1" decay_energy="1635920.0" reactions="0">
    <decay type="p" target="C10" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="N12" half_life="0.011" decay_modes="1" decay_energy="8919143.0" reactions="0">
    <decay type="ec/beta+" target="C12" branching_ratio="1.0"/>
    <source type="discrete" particle="photon">
      <parameters>277.0 510998.9 3215300.0 4438910.0 1.4160177724899021e-08 125.92896333596923 0.9452007007635618 1.7202652753896825</parameters>
    </source>
    <source type="discrete" particle="positron">
      <parameters>2228100.0 4628100.0 7038100.0 9683899.0 12899190.0 17338100.0 0.0027725887222397813 0.19534147815780276 0.28986154823415894 1.7013612613744111 1.19599395336616 59.57915083812984</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>260.0 5.04708755173339e-06</parameters>
    </source>
  </nuclide>
  <nuclide name="N13" half_life="597.9" decay_modes="1" decay_energy="1510845.6" reactions="0">
    <decay type="ec/beta+" target="C13" branching_ratio="1.0"/>
    <source type="discrete" particle="positron">
      <parameters>2220500.0 0.0011593028609465552</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>260.0 2.1519907147178717e-06</parameters>
    </source>
    <source type="discrete" particle="photon">
      <parameters>277.0 510998.9 6.03764929980966e-09 0.0023140519802553124</parameters>
    </source>
  </nuclide>
  <nuclide name="N14" reactions="4">
    <reaction type="(n,2n)" Q="-10553470.0" target="N13"/>
    <reaction type="(n,gamma)" Q="10833390.0" target="N15"/>
    <reaction type="(n,p)" Q="625876.1" target="C14"/>
    <reaction type="(n,a)" Q="-158297.0" target="B11"/>
  </nuclide>
  <nuclide name="N15" reactions="4">
    <reaction type="(n,2n)" Q="-10833000.0" target="N14"/>
    <reaction type="(n,gamma)" Q="2490000.0" target="N16"/>
    <reaction type="(n,p)" Q="-8989300.0" target="C15"/>
    <reaction type="(n,a)" Q="-7621500.0" target="B12"/>
  </nuclide>
  <nuclide name="N16" half_life="7.13" decay_modes="2" decay_energy="7258573.2758" reactions="0">
    <decay type="beta-" target="O16" branching_ratio="0.999988"/>
    <decay type="beta-,alpha" target="C12" branching_ratio="1.2e-05"/>
    <source type="discrete" particle="photon">
      <parameters>525.0 867700.0 986930.0 1754900.0 1954700.0 2741500.0 2822200.0 6128630.0 6915500.0 7115150.0 8869300.0 1.1296451946853526e-06 2.0415274602747335e-07 3.30533017377814e-06 0.00011763086794916321 3.6941925471638036e-05 0.0007971678654406103 0.00012638027135034065 0.06513444754209864 3.6941925471638036e-05 0.004763564073974378 7.388385094327607e-05</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>520.0 574599.6 834099.6 1547199.0 3302250.0 4289210.0 4369700.0 6047668.0 10419100.0 0.00013531078740192245 6.319013567517033e-10 1.1665871201569906e-06 0.0010304852894720085 0.004666348480627963 0.06435672279532732 1.1665871201569906e-05 0.00013610183068498224 0.02722036613699645</parameters>
    </source>
    <source type="discrete" particle="alpha">
      <parameters>1817300.0 2011900.0 1.1665871201569906e-06 6.299570448847749e-10</parameters>
    </source>
  </nuclide>
  <nuclide name="N17" half_life="4.171" decay_modes="2" decay_energy="2684407.04" reactions="0">
    <decay type="beta-" target="O17" branching_ratio="0.049"/>
    <decay type="beta-,n" target="O16" branching_ratio="0.951"/>
    <source type="discrete" particle="photon">
      <parameters>525.0 870710.0 2184480.0 3.8621825212163193e-10 0.005484022286856436 0.000565020478039754</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>520.0 870178.1 870695.8 2183948.0 2184466.0 2740000.0 3299800.0 3594200.0 4125200.0 5623640.0 7808270.0 8679000.0 4.626186593372321e-08 4.612062743246263e-08 2.38554969478255e-09 4.0963984657882167e-10 2.1188267926490774e-11 0.011466592054336186 0.08325742926409316 0.0009970949612466249 0.06314934754561957 0.000565020478039754 0.004985474806233124 0.002658919896657666</parameters>
    </source>
    <source type="discrete" particle="neutron">
      <parameters>382800.0 884000.0 1170900.0 1700300.0 0.05784247579687794 0.0009482373081455401 0.08771195100346245 0.011694926800461662</parameters>
    </source>
  </nuclide>
  <nuclide name="N18" half_life="0.624" decay_modes="3" decay_energy="8138430.1" reactions="0">
    <decay type="beta-" target="O18" branching_ratio="0.735"/>
    <decay type="beta-,alpha" target="C14" branching_ratio="0.122"/>
    <decay type="beta-,n" target="O17" branching_ratio="0.143"/>
  </nuclide>
  <nuclide name="N19" half_life="0.271" decay_modes="2" decay_energy="7300785.0" reactions="0">
    <decay type="beta-" target="O19" branching_ratio="0.454"/>
    <decay type="beta-,n" target="O18" branching_ratio="0.546"/>
  </nuclide>
  <nuclide name="N20" half_life="0.13" decay_modes="2" decay_energy="9579534.0" reactions="0">
    <decay type="beta-" target="O20" branching_ratio="0.43"/>
    <decay type="beta-,n" target="O19" branching_ratio="0.57"/>
  </nuclide>
  <nuclide name="N21" half_life="0.085" decay_modes="2" decay_energy="10307100.0" reactions="0">
    <decay type="beta-" target="O21" branching_ratio="0.19"/>
    <decay type="beta-,n" target="O20" branching_ratio="0.81"/>
  </nuclide>
  <nuclide name="N22" half_life="0.024" decay_modes="3" decay_energy="13289992.0" reactions="0">
    <decay type="beta-" target="O22" branching_ratio="0.51"/>
    <decay type="beta-,n" target="O21" branching_ratio="0.36"/>
    <decay type="beta-,n,n" target="O20" branching_ratio="0.13"/>
  </nuclide>
  <nuclide name="N23" half_life="0.0145" decay_modes="1" decay_energy="15881334.0" reactions="0">
    <decay type="beta-" target="O23" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="N24" half_life="5.2e-08" decay_modes="1" decay_energy="1029000.0" reactions="0">
    <decay type="n" target="N23" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="N25" half_life="2.6e-07" decay_modes="1" decay_energy="890000.0" reactions="0">
    <decay type="n" target="N24" branching_ratio="1.0"/>
  </nuclide>
</nuclides>
  "#;

        let nuclides: SerdeNuclideVec = serde_xml_rs::from_str(xml).unwrap();
        use crate::decay_xml_info_serde::nitrogen::get_nitrogen_xml_serde_data;
        assert_eq!(nuclides,get_nitrogen_xml_serde_data());

    }
}


use crate::decay_xml_info_serde::SerdeNuclideVec;
pub fn get_nitrogen_xml_serde_data() -> SerdeNuclideVec {
    
        let xml = r#"
    <nuclides>
          <nuclide name="N10" half_life="2e-22" decay_modes="1" decay_energy="2601730.0" reactions="0">
    <decay type="p" target="C9" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="N11" half_life="3.12742e-22" decay_modes="1" decay_energy="1635920.0" reactions="0">
    <decay type="p" target="C10" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="N12" half_life="0.011" decay_modes="1" decay_energy="8919143.0" reactions="0">
    <decay type="ec/beta+" target="C12" branching_ratio="1.0"/>
    <source type="discrete" particle="photon">
      <parameters>277.0 510998.9 3215300.0 4438910.0 1.4160177724899021e-08 125.92896333596923 0.9452007007635618 1.7202652753896825</parameters>
    </source>
    <source type="discrete" particle="positron">
      <parameters>2228100.0 4628100.0 7038100.0 9683899.0 12899190.0 17338100.0 0.0027725887222397813 0.19534147815780276 0.28986154823415894 1.7013612613744111 1.19599395336616 59.57915083812984</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>260.0 5.04708755173339e-06</parameters>
    </source>
  </nuclide>
  <nuclide name="N13" half_life="597.9" decay_modes="1" decay_energy="1510845.6" reactions="0">
    <decay type="ec/beta+" target="C13" branching_ratio="1.0"/>
    <source type="discrete" particle="positron">
      <parameters>2220500.0 0.0011593028609465552</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>260.0 2.1519907147178717e-06</parameters>
    </source>
    <source type="discrete" particle="photon">
      <parameters>277.0 510998.9 6.03764929980966e-09 0.0023140519802553124</parameters>
    </source>
  </nuclide>
  <nuclide name="N14" reactions="4">
    <reaction type="(n,2n)" Q="-10553470.0" target="N13"/>
    <reaction type="(n,gamma)" Q="10833390.0" target="N15"/>
    <reaction type="(n,p)" Q="625876.1" target="C14"/>
    <reaction type="(n,a)" Q="-158297.0" target="B11"/>
  </nuclide>
  <nuclide name="N15" reactions="4">
    <reaction type="(n,2n)" Q="-10833000.0" target="N14"/>
    <reaction type="(n,gamma)" Q="2490000.0" target="N16"/>
    <reaction type="(n,p)" Q="-8989300.0" target="C15"/>
    <reaction type="(n,a)" Q="-7621500.0" target="B12"/>
  </nuclide>
  <nuclide name="N16" half_life="7.13" decay_modes="2" decay_energy="7258573.2758" reactions="0">
    <decay type="beta-" target="O16" branching_ratio="0.999988"/>
    <decay type="beta-,alpha" target="C12" branching_ratio="1.2e-05"/>
    <source type="discrete" particle="photon">
      <parameters>525.0 867700.0 986930.0 1754900.0 1954700.0 2741500.0 2822200.0 6128630.0 6915500.0 7115150.0 8869300.0 1.1296451946853526e-06 2.0415274602747335e-07 3.30533017377814e-06 0.00011763086794916321 3.6941925471638036e-05 0.0007971678654406103 0.00012638027135034065 0.06513444754209864 3.6941925471638036e-05 0.004763564073974378 7.388385094327607e-05</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>520.0 574599.6 834099.6 1547199.0 3302250.0 4289210.0 4369700.0 6047668.0 10419100.0 0.00013531078740192245 6.319013567517033e-10 1.1665871201569906e-06 0.0010304852894720085 0.004666348480627963 0.06435672279532732 1.1665871201569906e-05 0.00013610183068498224 0.02722036613699645</parameters>
    </source>
    <source type="discrete" particle="alpha">
      <parameters>1817300.0 2011900.0 1.1665871201569906e-06 6.299570448847749e-10</parameters>
    </source>
  </nuclide>
  <nuclide name="N17" half_life="4.171" decay_modes="2" decay_energy="2684407.04" reactions="0">
    <decay type="beta-" target="O17" branching_ratio="0.049"/>
    <decay type="beta-,n" target="O16" branching_ratio="0.951"/>
    <source type="discrete" particle="photon">
      <parameters>525.0 870710.0 2184480.0 3.8621825212163193e-10 0.005484022286856436 0.000565020478039754</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>520.0 870178.1 870695.8 2183948.0 2184466.0 2740000.0 3299800.0 3594200.0 4125200.0 5623640.0 7808270.0 8679000.0 4.626186593372321e-08 4.612062743246263e-08 2.38554969478255e-09 4.0963984657882167e-10 2.1188267926490774e-11 0.011466592054336186 0.08325742926409316 0.0009970949612466249 0.06314934754561957 0.000565020478039754 0.004985474806233124 0.002658919896657666</parameters>
    </source>
    <source type="discrete" particle="neutron">
      <parameters>382800.0 884000.0 1170900.0 1700300.0 0.05784247579687794 0.0009482373081455401 0.08771195100346245 0.011694926800461662</parameters>
    </source>
  </nuclide>
  <nuclide name="N18" half_life="0.624" decay_modes="3" decay_energy="8138430.1" reactions="0">
    <decay type="beta-" target="O18" branching_ratio="0.735"/>
    <decay type="beta-,alpha" target="C14" branching_ratio="0.122"/>
    <decay type="beta-,n" target="O17" branching_ratio="0.143"/>
  </nuclide>
  <nuclide name="N19" half_life="0.271" decay_modes="2" decay_energy="7300785.0" reactions="0">
    <decay type="beta-" target="O19" branching_ratio="0.454"/>
    <decay type="beta-,n" target="O18" branching_ratio="0.546"/>
  </nuclide>
  <nuclide name="N20" half_life="0.13" decay_modes="2" decay_energy="9579534.0" reactions="0">
    <decay type="beta-" target="O20" branching_ratio="0.43"/>
    <decay type="beta-,n" target="O19" branching_ratio="0.57"/>
  </nuclide>
  <nuclide name="N21" half_life="0.085" decay_modes="2" decay_energy="10307100.0" reactions="0">
    <decay type="beta-" target="O21" branching_ratio="0.19"/>
    <decay type="beta-,n" target="O20" branching_ratio="0.81"/>
  </nuclide>
  <nuclide name="N22" half_life="0.024" decay_modes="3" decay_energy="13289992.0" reactions="0">
    <decay type="beta-" target="O22" branching_ratio="0.51"/>
    <decay type="beta-,n" target="O21" branching_ratio="0.36"/>
    <decay type="beta-,n,n" target="O20" branching_ratio="0.13"/>
  </nuclide>
  <nuclide name="N23" half_life="0.0145" decay_modes="1" decay_energy="15881334.0" reactions="0">
    <decay type="beta-" target="O23" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="N24" half_life="5.2e-08" decay_modes="1" decay_energy="1029000.0" reactions="0">
    <decay type="n" target="N23" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="N25" half_life="2.6e-07" decay_modes="1" decay_energy="890000.0" reactions="0">
    <decay type="n" target="N24" branching_ratio="1.0"/>
  </nuclide>
</nuclides>
  "#;

    let nuclides: SerdeNuclideVec = serde_xml_rs::from_str(xml).unwrap();

    return nuclides;
}
