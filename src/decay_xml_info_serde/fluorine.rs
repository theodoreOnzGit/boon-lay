
// notes, code was done from vibe coding using AI, then 
// modified


#[cfg(test)]
mod parsing_tests {
#[test]
fn serde_nuclide_test() {
    use crate::decay_xml_info_serde::SerdeNuclideVec;
    let xml = r#"
    <nuclides>
          <nuclide name="F14" half_life="5.0007e-22" decay_modes="1" decay_energy="1560000.0" reactions="0">
    <decay type="p" target="O13" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="F15" half_life="4.557e-22" decay_modes="1" decay_energy="1516490.0" reactions="0">
    <decay type="p" target="O14" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="F16" half_life="1.13925e-20" decay_modes="1" decay_energy="535680.0" reactions="0">
    <decay type="p" target="O15" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="F17" half_life="64.49" decay_modes="1" decay_energy="1759269.2" reactions="0">
    <decay type="ec/beta+" target="O17" branching_ratio="1.0"/>
    <source type="discrete" particle="positron">
      <parameters>1890070.0 2760800.0 1.8271828298215338e-06 0.010748134293067845</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>520.0 1.4703093024485143e-05</parameters>
    </source>
    <source type="discrete" particle="photon">
      <parameters>525.0 510998.9 1.2274906769398133e-07 0.021468538399659575</parameters>
    </source>
  </nuclide>
  <nuclide name="F18" half_life="6586.2" decay_modes="1" decay_energy="1230197.1" reactions="0">
    <decay type="ec/beta+" target="O18" branching_ratio="1.0"/>
    <source type="discrete" particle="positron">
      <parameters>1655300.0 0.00010524235227596267</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>520.0 3.233088211282006e-06</parameters>
    </source>
    <source type="discrete" particle="photon">
      <parameters>525.0 510998.9 2.6991506088216148e-08 0.0002036018547130774</parameters>
    </source>
  </nuclide>
  <nuclide name="F19" reactions="4">
    <reaction type="(n,2n)" Q="-10431000.0" target="F18"/>
    <reaction type="(n,gamma)" Q="6601300.0" target="F20"/>
    <reaction type="(n,p)" Q="-4036000.0" target="O19"/>
    <reaction type="(n,a)" Q="-1523000.0" target="N16"/>
  </nuclide>
  <nuclide name="F20" half_life="11.163" decay_modes="1" decay_energy="4115248.0" reactions="0">
    <decay type="beta-" target="Ne20" branching_ratio="1.0"/>
    <source type="discrete" particle="photon">
      <parameters>848.0 849.0 1633602.0 3332540.0 4965850.0 1.4045764828051245e-09 2.809152965610249e-09 0.0620929602099832 5.0916481954596e-06 3.104663533816829e-08</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>820.0 1632735.0 1633602.0 2058020.0 3331673.0 3332540.0 4964984.0 4965850.0 5390856.0 7024530.0 2.3053778419589948e-07 2.3409045067764607e-07 1.2977425268756603e-08 5.0916481954596e-06 5.60081301500556e-12 3.156821881184952e-13 2.3719629398360573e-14 1.3132726748045187e-15 0.062087868561787746 3.1046635338168295e-07</parameters>
    </source>
  </nuclide>
  <nuclide name="F21" half_life="4.158" decay_modes="1" decay_energy="2900338.3" reactions="0">
    <decay type="beta-" target="Ne21" branching_ratio="1.0"/>
    <source type="discrete" particle="photon">
      <parameters>350725.0 1395131.0 1745800.0 1890400.0 1989000.0 2779400.0 2793940.0 3384600.0 3533200.0 3735200.0 3883900.0 4174100.0 4333520.0 4525840.0 4684270.0 0.1492816979777371 0.02557194652848319 0.0012897938705276487 2.9856339595547425e-06 3.284197355510217e-07 2.642286054205947e-06 2.9856339595547425e-06 5.821986221131748e-07 4.86658335407423e-06 4.150031203781092e-06 1.5973141683617873e-06 5.3293566178052154e-05 7.926858162617841e-05 1.5823859985640135e-05 4.672517146703172e-05</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>999640.1 1158360.0 1799240.0 1948610.0 3938290.0 5333473.0 5684200.0 0.00012836058899258243 7.168188735949409e-05 6.501380481442487e-06 5.001061908801914e-06 0.026839032243903604 0.12352622914740727 0.016003398108166124</parameters>
    </source>
  </nuclide>
  <nuclide name="F22" half_life="4.23" decay_modes="1" decay_energy="7536090.0" reactions="0">
    <decay type="beta-" target="Ne22" branching_ratio="1.0"/>
    <source type="discrete" particle="photon">
      <parameters>1274537.0 1431100.0 1900000.0 2082600.0 2166100.0 2283900.0 2987700.0 3983500.0 4247900.0 4366100.0 0.16386458169265844 0.0004752072869087094 0.014256218607261284 0.13420509240628725 0.1009405823226776 0.00835709366632558 0.011470520718486091 0.0019663749803119014 0.0016386458169265845 0.018516697731270404</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>3395900.0 3477300.0 4471500.0 5176600.0 5294300.0 7460300.0 9543423.0 0.014256218607261284 0.002441582267220611 0.011470520718486091 0.026873791397595985 0.0883230095323429 0.0050798020324724115 0.001392848944387597</parameters>
    </source>
  </nuclide>
  <nuclide name="F23" half_life="2.23" decay_modes="1" decay_energy="5655862.0" reactions="0">
    <decay type="beta-" target="Ne23" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="F24" half_life="0.39" decay_modes="2" decay_energy="8681425.19" reactions="0">
    <decay type="beta-" target="Ne24" branching_ratio="0.941"/>
    <decay type="beta-,n" target="Ne23" branching_ratio="0.059"/>
  </nuclide>
  <nuclide name="F25" half_life="0.05" decay_modes="2" decay_energy="8657085.9" reactions="0">
    <decay type="beta-" target="Ne25" branching_ratio="0.86"/>
    <decay type="beta-,n" target="Ne24" branching_ratio="0.14"/>
  </nuclide>
  <nuclide name="F26" half_life="0.0096" decay_modes="2" decay_energy="11805584.6" reactions="0">
    <decay type="beta-" target="Ne26" branching_ratio="0.89"/>
    <decay type="beta-,n" target="Ne25" branching_ratio="0.11"/>
  </nuclide>
  <nuclide name="F27" half_life="0.005" decay_modes="2" decay_energy="12183691.0" reactions="0">
    <decay type="beta-" target="Ne27" branching_ratio="0.23"/>
    <decay type="beta-,n" target="Ne26" branching_ratio="0.77"/>
  </nuclide>
  <nuclide name="F28" half_life="4e-08" decay_modes="1" decay_energy="14533334.0" reactions="0">
    <decay type="beta-" target="Ne28" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="F29" half_life="0.0025" decay_modes="1" decay_energy="15735750.0" reactions="0">
    <decay type="beta-,n" target="Ne28" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="F30" half_life="2.6e-07" decay_modes="1" decay_energy="377000.0" reactions="0">
    <decay type="n" target="F29" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="F31" half_life="2.5e-07" decay_modes="2" decay_energy="17793166.0" reactions="0">
    <decay type="beta-" target="Ne31" branching_ratio="0.5"/>
    <decay type="beta-,n" target="Ne30" branching_ratio="0.5"/>
  </nuclide>
</nuclides>
  "#;

    let nuclides: SerdeNuclideVec = serde_xml_rs::from_str(xml).unwrap();
    dbg!("{:#?}", nuclides);
}
}
