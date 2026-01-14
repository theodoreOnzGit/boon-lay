
// notes, code was done from vibe coding using AI, then 
// modified


#[cfg(test)]
mod parsing_tests {
#[test]
fn serde_nuclide_test() {
    use crate::decay_xml_info_serde::SerdeNuclideVec;
    let xml = r#"
    <nuclides>
          <nuclide name="O12" half_life="1.13925e-21" decay_modes="1" decay_energy="455410.0" reactions="0">
    <decay type="p" target="N11" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="O13" half_life="0.00858" decay_modes="1" decay_energy="11867595.0" reactions="0">
    <decay type="ec/beta+,p" target="C12" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="O14" half_life="70.606" decay_modes="1" decay_energy="4096344.1" reactions="0">
    <decay type="ec/beta+" target="N14" branching_ratio="1.0"/>
    <source type="discrete" particle="photon">
      <parameters>392.0 510998.9 1635200.0 2312593.0 3947500.0 5.8742696991163446e-08 0.019610275075763028 5.1048994970848306e-06 0.009757033677235908 2.071411142086345e-07</parameters>
    </source>
    <source type="discrete" particle="positron">
      <parameters>1194940.0 2830242.0 5143040.0 5.301241785434248e-06 0.009751928777738823 5.988439794657206e-05</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>380.0 1.1257137845797537e-05</parameters>
    </source>
  </nuclide>
  <nuclide name="O15" half_life="122.24" decay_modes="1" decay_energy="1755626.2" reactions="0">
    <decay type="ec/beta+" target="N15" branching_ratio="1.0"/>
    <source type="discrete" particle="positron">
      <parameters>2754200.0 0.0056703794221199715</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>380.0 5.322802174682284e-06</parameters>
    </source>
    <source type="discrete" particle="photon">
      <parameters>392.0 510998.9 2.777578656131247e-08 0.011329452107672236</parameters>
    </source>
  </nuclide>
  <nuclide name="O16" reactions="4">
    <reaction type="(n,2n)" Q="-15663800.0" target="O15"/>
    <reaction type="(n,gamma)" Q="4143194.0" target="O17"/>
    <reaction type="(n,p)" Q="-9636800.0" target="N16"/>
    <reaction type="(n,a)" Q="-2214300.0" target="C13"/>
  </nuclide>
  <nuclide name="O17" reactions="4">
    <reaction type="(n,2n)" Q="-4144300.0" target="O16"/>
    <reaction type="(n,gamma)" Q="8047000.0" target="O18"/>
    <reaction type="(n,p)" Q="-7896000.0" target="N17"/>
    <reaction type="(n,a)" Q="1819400.0" target="C14"/>
  </nuclide>
  <nuclide name="O18" reactions="5">
    <reaction type="(n,2n)" Q="-8042100.0" target="O17"/>
    <reaction type="(n,3n)" Q="-12185000.0" target="O16"/>
    <reaction type="(n,gamma)" Q="3955900.0" target="O19"/>
    <reaction type="(n,p)" Q="-13270000.0" target="N18"/>
    <reaction type="(n,a)" Q="-5008000.0" target="C15"/>
  </nuclide>
  <nuclide name="O19" half_life="26.88" decay_modes="1" decay_energy="2724997.9" reactions="0">
    <decay type="beta-" target="F19" branching_ratio="1.0"/>
    <source type="discrete" particle="photon">
      <parameters>109894.0 197142.0 1149000.0 1236000.0 1356843.0 1444085.0 1553970.0 2353980.0 2582517.0 3710640.0 3797870.0 3907740.0 4180063.0 0.0006549828268684007 0.024729469723102214 1.2893362733629935e-07 4.383743329434178e-06 0.012996509635498974 0.0006807695523356605 0.0003584354839949122 4.6673973095740365e-07 4.873691113312115e-06 2.8365398013985855e-07 3.4296344871455626e-07 9.90210257942779e-07 2.0423086570069815e-05</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>269600.1 441899.9 786600.1 820600.1 911430.2 2039751.0 3265562.0 3360600.0 3473600.0 4622457.0 4709710.0 4819600.0 1.2893362733629935e-07 2.5374137859783713e-05 1.2893362733629935e-07 1.2893362733629935e-07 2.0887247628480496e-06 2.578672546725987e-07 0.01402797865418937 1.2893362733629935e-06 4.383743329434178e-06 0.01170717336213598 1.4182699006992929e-05 0.0005157345093451974</parameters>
    </source>
  </nuclide>
  <nuclide name="O20" half_life="13.51" decay_modes="1" decay_energy="2254842.0" reactions="0">
    <decay type="beta-" target="F20" branching_ratio="1.0"/>
    <source type="discrete" particle="photon">
      <parameters>325730.0 653200.0 656000.0 983530.0 1056780.0 1187700.0 1309170.0 1644500.0 1843740.0 2179090.0 2431430.0 2504540.0 3488130.0 5.130623098149114e-08 5.130623098149114e-08 1.0261246196298228e-07 5.643685407964026e-07 0.05129340442374577 5.130623098149114e-08 1.1800433125742963e-06 1.0261246196298228e-06 9.235121576668405e-07 1.2826557745372786e-06 9.748183886483317e-07 5.130623098149114e-07 1.0056021272372263e-05</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>325890.1 2757452.0 1.3852682365002609e-05 0.05129237829912614</parameters>
    </source>
  </nuclide>
  <nuclide name="O21" half_life="3.42" decay_modes="1" decay_energy="5333701.0" reactions="0">
    <decay type="beta-" target="F21" branching_ratio="1.0"/>
    <source type="discrete" particle="photon">
      <parameters>279920.0 933200.0 1450500.0 1729200.0 1730280.0 1754740.0 1787160.0 1884010.0 3179430.0 3459380.0 3517400.0 4572200.0 4583500.0 0.029943958200189635 0.01173729225748174 0.019962638800126425 0.008317766166719344 0.09241962407465938 0.022920066770515522 0.028742503087219064 0.013862943611198907 0.010628256768585828 0.006007275564852859 0.031237832937234866 0.009611640903764575 0.010720676392660488</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>3526000.0 3537620.0 4471050.0 4592330.0 4650360.0 6355170.0 6379600.0 0.010741754552537163 0.021280834490875512 0.002026746141988144 0.05999168580284906 0.02492897754645417 0.009120357638946648 0.07539495648195896</parameters>
    </source>
  </nuclide>
  <nuclide name="O22" half_life="2.25" decay_modes="2" decay_energy="3581841.45" reactions="0">
    <decay type="beta-" target="F22" branching_ratio="0.78"/>
    <decay type="beta-,n" target="F21" branching_ratio="0.22"/>
  </nuclide>
  <nuclide name="O23" half_life="0.0905" decay_modes="2" decay_energy="6074558.9" reactions="0">
    <decay type="beta-" target="F23" branching_ratio="0.69"/>
    <decay type="beta-,n" target="F22" branching_ratio="0.31"/>
  </nuclide>
  <nuclide name="O24" half_life="0.065" decay_modes="2" decay_energy="6399608.0" reactions="0">
    <decay type="beta-" target="F24" branching_ratio="0.42"/>
    <decay type="beta-,n" target="F23" branching_ratio="0.58"/>
  </nuclide>
  <nuclide name="O25" half_life="5e-08" decay_modes="1" decay_energy="776000.0" reactions="0">
    <decay type="n" target="O24" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="O26" half_life="4e-08" decay_modes="1" decay_energy="280000.0" reactions="0">
    <decay type="n" target="O25" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="O27" half_life="2.6e-07" decay_modes="1" decay_energy="883000.0" reactions="0">
    <decay type="n" target="O26" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="O28" half_life="1e-07" decay_modes="1" decay_energy="731000.0" reactions="0">
    <decay type="n" target="O27" branching_ratio="1.0"/>
  </nuclide>
</nuclides>
  "#;

    let nuclides: SerdeNuclideVec = serde_xml_rs::from_str(xml).unwrap();
    dbg!("{:#?}", nuclides);
}
}
