// notes, code was done from vibe coding using AI, then 
// modified


#[cfg(test)]
mod parsing_tests {
#[test]
fn serde_nuclide_test() {
    use crate::decay_xml_info_serde::SerdeNuclideVec;
    let xml = r#"
    <nuclides>
       <nuclide name="S26" half_life="0.01" decay_modes="1" decay_energy="637000.0" reactions="0">
    <decay type="p,p" target="Si24" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="S27" half_life="0.0155" decay_modes="3" decay_energy="12177158.0" reactions="0">
    <decay type="ec/beta+" target="P27" branching_ratio="0.966"/>
    <decay type="ec/beta+,p" target="Si26" branching_ratio="0.023"/>
    <decay type="ec/beta+,p,p" target="Al25" branching_ratio="0.011"/>
  </nuclide>
  <nuclide name="S28" half_life="0.125" decay_modes="2" decay_energy="7356247.4" reactions="0">
    <decay type="ec/beta+" target="P28" branching_ratio="0.793"/>
    <decay type="ec/beta+,p" target="Si27" branching_ratio="0.207"/>
  </nuclide>
  <nuclide name="S29" half_life="0.187" decay_modes="2" decay_energy="8767506.0" reactions="0">
    <decay type="ec/beta+" target="P29" branching_ratio="0.53"/>
    <decay type="ec/beta+,p" target="Si28" branching_ratio="0.47"/>
  </nuclide>
  <nuclide name="S30" half_life="1.178" decay_modes="1" decay_energy="3688451.0" reactions="0">
    <decay type="ec/beta+" target="P30" branching_ratio="1.0"/>
    <source type="discrete" particle="photon">
      <parameters>2009.0 2010.0 2136.0 510998.9 677010.0 708700.0 2342200.0 7.898982880345724e-06 1.5875400539096956e-05 7.124570371171913e-07 1.1763496123730413 0.46131357347962404 0.0017063894937384052 0.013415751881805393</parameters>
    </source>
    <source type="discrete" particle="positron">
      <parameters>3118800.0 5429300.0 5460990.0 6138000.0 0.013415751881805393 0.0017063894937384052 0.4477801395637677 0.12533136626423458</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>1850.0 0.000362346336780767</parameters>
    </source>
  </nuclide>
  <nuclide name="S31" half_life="2.572" decay_modes="1" decay_energy="3034467.0" reactions="0">
    <decay type="ec/beta+" target="P31" branching_ratio="1.0"/>
    <source type="discrete" particle="photon">
      <parameters>2009.0 2010.0 2136.0 510998.9 1266150.0 1868900.0 2239600.0 3134100.0 3505800.0 3.5558908508217784e-06 7.14664387865648e-06 3.207273931644837e-07 0.5386314147435831 0.0032609179178753256 2.425476137262639e-06 1.3474867429236884e-05 9.432407200465819e-05 1.8864814400931635e-05</parameters>
    </source>
    <source type="discrete" particle="positron">
      <parameters>1890300.0 2262000.0 4129950.0 5396100.0 3.260917917875326e-05 8.785613563862448e-05 0.002937521099573641 0.26645202854573014</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>1850.0 0.00016311777083663387</parameters>
    </source>
  </nuclide>
  <nuclide name="S32" reactions="4">
    <reaction type="(n,2n)" Q="-15088000.0" target="S31"/>
    <reaction type="(n,gamma)" Q="8642990.0" target="S33"/>
    <reaction type="(n,p)" Q="-927629.0" target="P32"/>
    <reaction type="(n,a)" Q="1525750.0" target="Si29"/>
  </nuclide>
  <nuclide name="S33" reactions="4">
    <reaction type="(n,2n)" Q="-8643380.0" target="S32"/>
    <reaction type="(n,gamma)" Q="11417000.0" target="S34"/>
    <reaction type="(n,p)" Q="533469.0" target="P33"/>
    <reaction type="(n,a)" Q="3492040.0" target="Si30"/>
  </nuclide>
  <nuclide name="S34" reactions="4">
    <reaction type="(n,2n)" Q="-11414900.0" target="S33"/>
    <reaction type="(n,gamma)" Q="6986990.0" target="S35"/>
    <reaction type="(n,p)" Q="-4316720.0" target="P34"/>
    <reaction type="(n,a)" Q="-1334550.0" target="Si31"/>
  </nuclide>
  <nuclide name="S35" half_life="7560864.0" decay_modes="1" decay_energy="48758.0" reactions="4">
    <decay type="beta-" target="Cl35" branching_ratio="1.0"/>
    <source type="discrete" particle="electron">
      <parameters>167240.0 9.167565777667014e-08</parameters>
    </source>
    <reaction type="(n,2n)" Q="-6985000.0" target="S34"/>
    <reaction type="(n,gamma)" Q="9889430.0" target="S36"/>
    <reaction type="(n,p)" Q="-3205600.0" target="P35"/>
    <reaction type="(n,a)" Q="881500.1" target="Si32"/>
  </nuclide>
  <nuclide name="S36" reactions="4">
    <reaction type="(n,2n)" Q="-9891980.0" target="S35"/>
    <reaction type="(n,gamma)" Q="4313990.0" target="S37"/>
    <reaction type="(n,p)" Q="-8990920.0" target="P36"/>
    <reaction type="(n,a)" Q="-3954040.0" target="Si33"/>
  </nuclide>
  <nuclide name="S37" half_life="303.0" decay_modes="1" decay_energy="3731492.6" reactions="0">
    <decay type="beta-" target="Cl37" branching_ratio="1.0"/>
    <source type="discrete" particle="photon">
      <parameters>2621.0 2622.0 2816.0 906360.0 1169070.0 3086000.0 3103360.0 3741020.0 4009640.0 4396000.0 5.143243584333086e-10 1.045793014655358e-09 9.806451863333235e-11 1.2257038262178834e-06 7.741287323481369e-07 1.4192360093049176e-06 0.0021503575898559357 6.02100125159662e-06 6.236037010582215e-07 8.601430359423744e-08</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>2380.0 468820.3 592530.2 855250.0 903537.6 906089.8 906360.0 1123900.0 1166248.0 1168800.0 1169070.0 1761620.0 1778980.0 3083178.0 3085730.0 3086000.0 3100538.0 3103090.0 3103360.0 3738198.0 3740750.0 3741020.0 4006818.0 4009370.0 4009640.0 4865120.0 1.5448592134199642e-08 9.150457829174196e-08 1.2810640960843872e-06 2.058853011564194e-06 6.128519131089418e-11 4.878301228347176e-12 4.461557352204181e-13 6.107930600973776e-06 2.1675604505747835e-11 1.7263070731363454e-12 1.5792226139901995e-13 0.0021503575898559357 9.150457829174196e-07 8.032875812665834e-12 6.37237883223691e-13 5.833050847785383e-14 1.703083211165901e-08 1.3525749240193837e-09 1.238605971757019e-10 1.9086573967561287e-11 1.5172923154023484e-12 1.3848302878672226e-13 3.2365032084921694e-12 2.5692426731309576e-13 2.3509813777605802e-14 0.00012810640960843873</parameters>
    </source>
  </nuclide>
  <nuclide name="S38" half_life="10218.0" decay_modes="1" decay_energy="2185476.6" reactions="0">
    <decay type="beta-" target="Cl38" branching_ratio="1.0"/>
    <source type="discrete" particle="photon">
      <parameters>196190.0 755425.0 936800.0 1692640.0 1745770.0 1941945.0 2750980.0 5.630379329269471e-08 9.008606926831154e-09 9.008606926831154e-09 1.1260758658538942e-07 1.6553315228052245e-06 5.630379329269471e-05 9.346429686587323e-07</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>185909.9 995001.9 1191190.0 1244320.0 2937000.0 9.361353583604663e-07 5.630379329269471e-05 1.6551958510141579e-06 1.153210224067241e-07 8.818666419337726e-06</parameters>
    </source>
  </nuclide>
  <nuclide name="S39" half_life="11.5" decay_modes="1" decay_energy="3972966.0" reactions="0">
    <decay type="beta-" target="Cl39" branching_ratio="1.0"/>
    <source type="discrete" particle="photon">
      <parameters>396490.0 396500.0 484850.0 874310.0 903800.0 1300520.0 1696620.0 0.022111997796541005 0.002131276896052145 0.006393830688156434 0.007725878748189024 0.002131276896052145 0.031436334216769134 0.02664096120065181</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>4068900.0 4854700.0 4943220.0 5339570.0 0.007715029487971565 0.006389008794726452 0.04098609415484894 0.0048218934299822285</parameters>
    </source>
  </nuclide>
  <nuclide name="S40" half_life="8.8" decay_modes="1" decay_energy="2906122.0" reactions="0">
    <decay type="beta-" target="Cl40" branching_ratio="1.0"/>
    <source type="discrete" particle="photon">
      <parameters>211900.0 403800.0 431800.0 457800.0 677900.0 889200.0 1013700.0 1081600.0 1293100.0 1875600.0 0.03898952890649692 0.005146617815657593 0.017233371776671638 0.0032361308992392445 0.020547481733723876 0.013529366530554431 0.025343193789222996 0.012008774903201052 0.009045570706307285 0.010800099507099646</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>2382800.0 3396600.0 3800500.0 4258300.0 4478200.0 4690000.0 0.03623269352926987 0.0015753345012726027 0.032294357276088356 0.0031506690025452054 0.006301338005090411 0.0003938336253181507</parameters>
    </source>
  </nuclide>
  <nuclide name="S41" half_life="1.99" decay_modes="1" decay_energy="5478962.0" reactions="0">
    <decay type="beta-" target="Cl41" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="S42" half_life="1.013" decay_modes="1" decay_energy="4823658.0" reactions="0">
    <decay type="beta-" target="Cl42" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="S43" half_life="0.28" decay_modes="2" decay_energy="6398072.0" reactions="0">
    <decay type="beta-" target="Cl43" branching_ratio="0.6"/>
    <decay type="beta-,n" target="Cl42" branching_ratio="0.4"/>
  </nuclide>
  <nuclide name="S44" half_life="0.1" decay_modes="2" decay_energy="7232018.6" reactions="0">
    <decay type="beta-" target="Cl44" branching_ratio="0.82"/>
    <decay type="beta-,n" target="Cl43" branching_ratio="0.18"/>
  </nuclide>
  <nuclide name="S45" half_life="0.068" decay_modes="2" decay_energy="8014207.0" reactions="0">
    <decay type="beta-" target="Cl45" branching_ratio="0.46"/>
    <decay type="beta-,n" target="Cl44" branching_ratio="0.54"/>
  </nuclide>
  <nuclide name="S46" half_life="0.05" decay_modes="1" decay_energy="9232000.0" reactions="0">
    <decay type="beta-" target="Cl46" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="S48" half_life="2e-07" decay_modes="1" decay_energy="11215334.0" reactions="0">
    <decay type="beta-" target="Cl48" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="S49" half_life="2e-07" decay_modes="1" decay_energy="370000.0" reactions="0">
    <decay type="n" target="S48" branching_ratio="1.0"/>
  </nuclide>
</nuclides>
  "#;

    let nuclides: SerdeNuclideVec = serde_xml_rs::from_str(xml).unwrap();
    dbg!("{:#?}", nuclides);
}
}

