



// notes, code was done from vibe coding using AI, then 
// modified


#[cfg(test)]
mod parsing_tests {
#[test]
fn serde_nuclide_test() {
    use crate::decay_xml_info_serde::SerdeNuclideVec;
    let xml = r#"
    <nuclides>
      <nuclide name="P24" half_life="0.0074" decay_modes="2" decay_energy="8589000.0" reactions="0">
    <decay type="ec/beta+" target="Si24" branching_ratio="0.5"/>
    <decay type="p" target="Si23" branching_ratio="0.5"/>
  </nuclide>
  <nuclide name="P25" half_life="3e-08" decay_modes="1" decay_energy="1695000.0" reactions="0">
    <decay type="p" target="Si24" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="P26" half_life="0.0437" decay_modes="1" decay_energy="12076000.0" reactions="0">
    <decay type="ec/beta+" target="Si26" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="P27" half_life="0.26" decay_modes="2" decay_energy="7775773.7543" reactions="0">
    <decay type="ec/beta+" target="Si27" branching_ratio="0.9993"/>
    <decay type="ec/beta+,p" target="Al26" branching_ratio="0.0007"/>
  </nuclide>
  <nuclide name="P28" half_life="0.2703" decay_modes="3" decay_energy="9321019.52801" reactions="0">
    <decay type="ec/beta+" target="Si28" branching_ratio="0.9999784"/>
    <decay type="ec/beta+,alpha" target="Mg24" branching_ratio="8.6e-06"/>
    <decay type="ec/beta+,p" target="Al27" branching_ratio="1.3e-05"/>
    <source type="discrete" particle="photon">
      <parameters>1739.0 1740.0 1836.0 510998.9 911340.0 1352200.0 1516500.0 1522790.0 1657100.0 1658810.0 1778969.0 2063140.0 2098290.0 2312300.0 2734200.0 2838290.0 2953400.0 3039200.0 3104600.0 3181500.0 3200700.0 3251900.0 3278800.0 3287250.0 3315800.0 3641230.0 3780120.0 3970900.0 4391270.0 4496920.0 4499180.0 4861170.0 5636620.0 6019500.0 6153800.0 6479200.0 6808900.0 7414920.0 7535700.0 7601000.0 7699320.0 7932400.0 8015300.0 8257800.0 8428400.0 8887500.0 9379500.0 9477400.0 9793800.0 5.262656500429801e-06 1.0589496429576713e-05 2.6955065502070347e-07 5.12756075582655 0.00010257449952792384 0.003590107483477334 0.005231299475924116 0.022053517398503624 0.010257449952792383 0.020514899905584767 2.5002534259931433 7.693087464594287e-06 1.0257449952792382e-05 0.005026150476868267 7.693087464594287e-05 0.06026251847265525 0.0020771336154404574 0.06923778718134858 0.0006154469971675429 0.0008205959962233907 0.004769714228048458 0.0008205959962233907 0.0025130752384341336 0.00015386174929188574 0.0025900061130800767 5.128724976396192e-05 0.0005128724976396192 0.002974660486309791 0.00038465437322971434 0.28207987370179055 7.693087464594287e-05 0.00010257449952792384 0.0002564362488198096 0.04487634354346668 0.002872085986781867 0.009872795579562669 0.08539327085699659 0.005385161225216001 0.21797081149683817 0.014103993685089525 5.128724976396192e-05 0.055133793496259055 0.0010257449952792385 0.0013334684938630097 0.0007436651215774478 0.0022053517398503623 0.0005180012226160154 0.0014103993685089529 0.00033336712346575243</parameters>
    </source>
    <source type="discrete" particle="positron">
      <parameters>1250601.0 1444300.0 1618400.0 1629500.0 1770800.0 1793400.0 1903400.0 2054200.0 2272400.0 2687100.0 2828400.0 3675700.0 4134690.0 4547750.0 4863300.0 4962500.0 5027600.0 5754800.0 6084300.0 6410300.0 6544900.0 6927500.0 8067200.0 9725900.0 12564720.0 5.89803372285562e-07 4.102979981116953e-06 4.8722887275763816e-06 6.923778718134859e-07 9.744577455152763e-06 1.820697366620648e-06 3.077234985837715e-05 1.820697366620648e-05 2.282282614496305e-06 6.15446997167543e-06 1.9745591159125337e-06 0.007693087464594287 0.0007436651215774478 0.0013591121187449906 0.001666835617328762 0.015129738680368765 0.28977296116638485 0.09334279457041068 0.013770626561623773 0.07282789466482592 0.06257044471203355 0.004872288727576382 0.19489154910305528 0.033080276097755434 1.771974479344884</parameters>
    </source>
    <source type="discrete" particle="proton">
      <parameters>469000.0 679000.0 828000.0 953000.0 1089000.0 1267000.0 1452000.0 1.0334380827438326e-06 1.7368427132565703e-05 1.0334380827438326e-06 9.734320005199972e-06 7.000709592780802e-07 3.1336509605780726e-06 3.3336712346575247e-07</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>1610.0 2260830.0 2400550.0 4495081.0 4496920.0 4497342.0 4499180.0 4859331.0 4861170.0 0.0003047206301100915 1.0289889138268088e-12 4.878699633796877e-13 5.303101625593662e-07 4.029793075703779e-08 1.469379705737509e-10 1.1168824381097986e-11 1.7601784118991727e-10 1.3382894953408224e-11</parameters>
    </source>
  </nuclide>
  <nuclide name="P29" half_life="4.142" decay_modes="1" decay_energy="2815758.0" reactions="0">
    <decay type="ec/beta+" target="Si29" branching_ratio="1.0"/>
    <source type="discrete" particle="photon">
      <parameters>61.4589 145.0132 1719.83 1720.52 1821.95 1821.98 397860.0 510998.9 754740.0 1152593.0 1273368.0 2028070.0 2425906.0 3.159874250251918e-08 1.0358042047815343e-09 2.0126269730389948e-06 3.993389591985662e-06 3.431006584986688e-08 6.807096902766097e-08 7.44020368123978e-07 0.3344644328716391 6.581718641096728e-06 2.5182227844196178e-05 0.0025926248212320155 0.00010587982161764301 0.0001631121576271798</parameters>
    </source>
    <source type="discrete" particle="positron">
      <parameters>2517084.0 2914820.0 3669702.0 4943100.0 0.0007630978134604902 4.183650293094793e-06 0.0021085597477197757 0.16448439492331487</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>31.10372 90.2625 1593.173 1696.938 1805.758 6.08943687840885e-05 0.000248491088732588 0.00011141080812032839 8.264438013163323e-06 1.6825938625578018e-07</parameters>
    </source>
  </nuclide>
  <nuclide name="P30" half_life="149.88" decay_modes="1" decay_energy="2460431.0" reactions="0">
    <decay type="ec/beta+" target="Si30" branching_ratio="1.0"/>
    <source type="discrete" particle="photon">
      <parameters>1739.0 1740.0 1836.0 510998.9 1263130.0 1534120.0 1552360.0 2235230.0 3498330.0 3769220.0 9.904776305684725e-08 1.993034465306311e-07 5.073178807131735e-09 0.0092359503222329 3.7459915682783275e-08 4.624680948491762e-09 1.5723915224871991e-07 2.72856175961014e-06 3.468510711368822e-08 3.69974475879341e-09</parameters>
    </source>
    <source type="discrete" particle="positron">
      <parameters>444689.9 462879.9 733849.9 1997080.0 4232400.0 1.5723915224871991e-07 8.324425707285172e-09 7.214502279647149e-08 2.5435745216704697e-06 0.004621859893113182</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>1610.0 1261291.0 1263130.0 1532281.0 1534120.0 1550521.0 1552360.0 2233391.0 2235230.0 3496491.0 3498330.0 3767381.0 3769220.0 5.735108466353172e-06 5.090802541290247e-13 3.872980682442962e-14 4.439693710552092e-14 3.376942028588685e-15 1.7626508967081502e-12 1.340778251224835e-13 1.5416373941797292e-11 1.172190131928516e-12 9.53840445626426e-14 7.246412578191743e-15 8.768395078340382e-15 6.668789927725121e-16</parameters>
    </source>
  </nuclide>
  <nuclide name="P31" reactions="4">
    <reaction type="(n,2n)" Q="-12310000.0" target="P30"/>
    <reaction type="(n,gamma)" Q="7930000.0" target="P32"/>
    <reaction type="(n,p)" Q="-710000.0" target="Si31"/>
    <reaction type="(n,a)" Q="-1940000.0" target="Al28"/>
  </nuclide>
  <nuclide name="P32" half_life="1232323.0" decay_modes="1" decay_energy="694900.0" reactions="0">
    <decay type="beta-" target="S32" branching_ratio="1.0"/>
    <source type="discrete" particle="electron">
      <parameters>1710480.0 5.62471998461398e-07</parameters>
    </source>
  </nuclide>
  <nuclide name="P33" half_life="2189376.0" decay_modes="1" decay_energy="76430.0" reactions="0">
    <decay type="beta-" target="S33" branching_ratio="1.0"/>
    <source type="discrete" particle="electron">
      <parameters>248500.0 3.165957700093293e-07</parameters>
    </source>
  </nuclide>
  <nuclide name="P34" half_life="12.43" decay_modes="1" decay_energy="2639205.3" reactions="0">
    <decay type="beta-" target="S34" branching_ratio="1.0"/>
    <source type="discrete" particle="photon">
      <parameters>1788800.0 1947043.0 1987180.0 2127492.0 4074403.0 4114540.0 2.509382391407686e-05 2.342090231980507e-05 7.27720893508229e-05 0.008364607971358953 3.847719666825118e-05 0.00010037529565630743</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>1259190.0 1299333.0 1457590.0 3246436.0 5374000.0 0.00018402137536989698 6.134045845663232e-05 2.509382391407686e-05 0.00830884391821656 0.04739944517103407</parameters>
    </source>
  </nuclide>
  <nuclide name="P35" half_life="47.3" decay_modes="1" decay_energy="2601371.0" reactions="0">
    <decay type="beta-" target="S35" branching_ratio="1.0"/>
    <source type="discrete" particle="photon">
      <parameters>1572334.0 2938510.0 0.014585563468493358 6.938476548777205e-05</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>1049760.0 2416028.0 7.034051726612553e-05 0.014583933913176693</parameters>
    </source>
  </nuclide>
  <nuclide name="P36" half_life="5.6" decay_modes="1" decay_energy="8026552.0" reactions="0">
    <decay type="beta-" target="S36" branching_ratio="1.0"/>
    <source type="discrete" particle="photon">
      <parameters>116.6069 215.6555 2283.17 2284.5 2445.56 2445.65 184600.0 579700.0 757500.0 809400.0 828800.0 901800.0 1013400.0 1058500.0 1255700.0 1284300.0 1441000.0 1638200.0 1730600.0 1960200.0 2020600.0 2065700.0 2250300.0 2321600.0 2539900.0 3079100.0 3290700.0 1.57380800214087e-09 1.0203497826689123e-10 6.744954563650529e-08 1.3353814629449405e-07 3.1460316678690313e-09 6.230010572161256e-09 0.0030944070560711846 0.0004951051289713896 0.002104196798128406 0.006683919241113758 0.01992798144109843 0.08713850269896455 0.0007426576934570843 0.006560142958870911 0.005569932700928132 0.005074827571956743 0.0007426576934570843 0.043693027631725126 0.000618881411214237 0.016709798102784397 0.006312590394385216 0.0009902102579427792 0.0019804205158855583 0.002475525644856948 0.021537073110255444 0.0034657359027997266 0.12377628224284738</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>37.20996 147.4941 2088.833 2256.009 2426.497 899328.0 901800.0 1281828.0 1284300.0 3141100.0 3288228.0 3290700.0 3898600.0 4582100.0 5161800.0 5206900.0 5391500.0 5837800.0 6220300.0 7122100.0 1.2002634361838986e-06 5.137361825271474e-06 2.2250259670910507e-06 2.7761510036427306e-07 8.686594732546581e-09 2.0738963642353563e-06 1.7382386042863806e-07 9.946662041035216e-08 8.325762114552233e-09 0.014062693775482414 5.483289303358139e-07 4.5772469173404964e-08 0.00042430571440989207 0.06946489581923979 0.014426385625596573 0.0023033738782251285 0.012971630602768163 0.00024246040823422405 0.00848611428819784 0.001394146728465377</parameters>
    </source>
  </nuclide>
  <nuclide name="P37" half_life="2.31" decay_modes="1" decay_energy="5267014.0" reactions="0">
    <decay type="beta-" target="S37" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="P38" half_life="0.64" decay_modes="2" decay_energy="7544568.2" reactions="0">
    <decay type="beta-" target="S38" branching_ratio="0.88"/>
    <decay type="beta-,n" target="S37" branching_ratio="0.12"/>
  </nuclide>
  <nuclide name="P39" half_life="0.28" decay_modes="2" decay_energy="6268125.1" reactions="0">
    <decay type="beta-" target="S39" branching_ratio="0.74"/>
    <decay type="beta-,n" target="S38" branching_ratio="0.26"/>
  </nuclide>
  <nuclide name="P40" half_life="0.125" decay_modes="2" decay_energy="9166735.5" reactions="0">
    <decay type="beta-" target="S40" branching_ratio="0.842"/>
    <decay type="beta-,n" target="S39" branching_ratio="0.158"/>
  </nuclide>
  <nuclide name="P41" half_life="0.1" decay_modes="2" decay_energy="8725907.5" reactions="0">
    <decay type="beta-" target="S41" branching_ratio="0.7"/>
    <decay type="beta-,n" target="S40" branching_ratio="0.3"/>
  </nuclide>
  <nuclide name="P42" half_life="0.0485" decay_modes="2" decay_energy="10686575.0" reactions="0">
    <decay type="beta-" target="S42" branching_ratio="0.5"/>
    <decay type="beta-,n" target="S41" branching_ratio="0.5"/>
  </nuclide>
  <nuclide name="P43" half_life="0.0365" decay_modes="1" decay_energy="11529096.0" reactions="0">
    <decay type="beta-,n" target="S42" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="P44" half_life="0.0185" decay_modes="2" decay_energy="12511750.0" reactions="0">
    <decay type="beta-" target="S44" branching_ratio="0.5"/>
    <decay type="beta-,n" target="S43" branching_ratio="0.5"/>
  </nuclide>
  <nuclide name="P45" half_life="2e-07" decay_modes="1" decay_energy="12875334.0" reactions="0">
    <decay type="beta-" target="S45" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="P46" half_life="2e-07" decay_modes="1" decay_energy="15164666.0" reactions="0">
    <decay type="beta-" target="S46" branching_ratio="1.0"/>
  </nuclide>
</nuclides>
  "#;

        let nuclides: SerdeNuclideVec = serde_xml_rs::from_str(xml).unwrap();
        use crate::decay_xml_info_serde::phosphorous::get_phosphorous_xml_serde_data;
        assert_eq!(nuclides,get_phosphorous_xml_serde_data());

    }
}


use crate::decay_xml_info_serde::SerdeNuclideVec;
pub fn get_phosphorous_xml_serde_data() -> SerdeNuclideVec {
    
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
