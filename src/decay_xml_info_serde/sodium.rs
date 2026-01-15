

// notes, code was done from vibe coding using AI, then 
// modified


#[cfg(test)]
mod parsing_tests {
    use crate::prelude::sodium::get_sodium_xml_serde_data;

#[test]
fn serde_nuclide_test() {
    use crate::decay_xml_info_serde::SerdeNuclideVec;
    let xml = r#"
    <nuclides>
           <nuclide name="Na18" half_life="1.3e-21" decay_modes="1" decay_energy="1251000.0" reactions="0">
    <decay type="p" target="Ne17" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Na19" half_life="4e-08" decay_modes="1" decay_energy="321250.0" reactions="0">
    <decay type="p" target="Ne18" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Na20" half_life="0.4479" decay_modes="2" decay_energy="8782298.6" reactions="0">
    <decay type="ec/beta+" target="Ne20" branching_ratio="0.7995"/>
    <decay type="ec/beta+,alpha" target="O16" branching_ratio="0.2005"/>
  </nuclide>
  <nuclide name="Na21" half_life="22.49" decay_modes="1" decay_energy="2139475.0" reactions="0">
    <decay type="ec/beta+" target="Ne21" branching_ratio="1.0"/>
    <source type="discrete" particle="photon">
      <parameters>848.0 849.0 350725.0 510998.9 2793940.0 1.6746423554232584e-07 3.349284710846517e-07 0.001562586129586004 0.06157883800617033 1.2328095696930996e-07</parameters>
    </source>
    <source type="discrete" particle="positron">
      <parameters>753060.1 3196473.0 3547200.0 1.2328095696930996e-07 0.001562586129586004 0.02925765311274149</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>820.0 2793073.0 2793940.0 2.7486460193843794e-05 1.6753882052129226e-13 9.27072796409211e-15</parameters>
    </source>
  </nuclide>
  <nuclide name="Na22" half_life="82134970.0" decay_modes="1" decay_energy="2387654.5" reactions="4">
    <decay type="ec/beta+" target="Ne22" branching_ratio="1.0"/>
    <source type="discrete" particle="photon">
      <parameters>848.0 849.0 510998.9 1274537.0 4.6975589383571496e-12 9.395116188889537e-12 1.5181139813033177e-08 8.43414472207654e-09</parameters>
    </source>
    <source type="discrete" particle="positron">
      <parameters>1568443.0 2843020.0 8.434397895790694e-09 4.725909330868074e-12</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>820.0 1273670.0 1274537.0 7.710259467707485e-10 5.364116380805631e-14 2.968818773388466e-15</parameters>
    </source>
    <reaction type="(n,2n)" Q="-10584000.0" target="Na21"/>
    <reaction type="(n,gamma)" Q="12420000.0" target="Na23"/>
    <reaction type="(n,p)" Q="3625000.0" target="Ne22"/>
    <reaction type="(n,a)" Q="1950000.0" target="F19"/>
  </nuclide>
  <nuclide name="Na23" reactions="5">
    <reaction type="(n,2n)" Q="-12414000.0" target="Na22"/>
    <reaction type="(n,p)" Q="-3597000.0" target="Ne23"/>
    <reaction type="(n,a)" Q="-3866000.0" target="F20"/>
    <reaction type="(n,gamma)" Q="6959490.0" target="Na24" branching_ratio="0.232"/>
    <reaction type="(n,gamma)" Q="6959490.0" target="Na24_m1" branching_ratio="0.768"/>
  </nuclide>
  <nuclide name="Na24" half_life="53989.2" decay_modes="1" decay_energy="4676923.1" reactions="0">
    <decay type="beta-" target="Mg24" branching_ratio="1.0"/>
    <source type="discrete" particle="photon">
      <parameters>1254.0 996600.0 1368626.0 2754007.0 2871000.0 3866220.0 4238900.0 7.231115080115924e-13 2.696111591162464e-10 1.283780495247919e-05 1.2820010615977518e-05 3.209656656145791e-11 9.50058370219154e-09 1.0784446364649855e-10</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>1180.0 280330.1 1367321.0 1368626.0 1392561.0 2752702.0 2754007.0 2869695.0 2871000.0 3864915.0 3866220.0 4146778.0 4237594.0 4238900.0 3.5103449948695035e-11 9.757356234683204e-09 3.578125240271327e-12 2.3845181229838305e-13 1.2820010615977518e-05 3.25628269645829e-11 2.1697873167185733e-12 7.638982841626982e-17 5.0828122806724745e-18 1.4402884892522373e-14 9.587039013881483e-16 8.216721039733224e-09 1.434331366498431e-16 9.539921254169263e-18</parameters>
    </source>
  </nuclide>
  <nuclide name="Na24_m1" half_life="0.02018" decay_modes="2" decay_energy="473350.45" reactions="0">
    <decay type="beta-" target="Mg24" branching_ratio="0.0005"/>
    <decay type="IT" target="Na24" branching_ratio="0.9995"/>
    <source type="discrete" particle="photon">
      <parameters>472202.4 34.331050890469044</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>5985807.0 0.017174112501485265</parameters>
    </source>
  </nuclide>
  <nuclide name="Na25" half_life="59.1" decay_modes="1" decay_energy="1942344.5" reactions="0">
    <decay type="beta-" target="Mg25" branching_ratio="1.0"/>
    <source type="discrete" particle="photon">
      <parameters>1254.0 389710.0 585028.0 836841.0 974742.0 989865.0 1379543.0 1611716.0 1964501.0 2216267.0 2801312.0 6.047407731053198e-09 0.001486571998916634 0.001524689229658086 1.2197513837264687e-05 0.001753392614106799 1.95160221396235e-05 2.7139468287913935e-05 0.0011114984484207446 1.7198494510543212e-05 1.0962515561241639e-05 5.793819072700727e-06</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>1180.0 388405.0 389710.0 583723.0 585028.0 835536.0 836841.0 973437.0 974742.0 988560.0 989865.0 1033540.0 1378238.0 1379543.0 1610411.0 1611716.0 1870380.0 1963196.0 1964501.0 2223228.0 2800007.0 2801312.0 2860251.0 3835000.0 2.9357140500830306e-07 1.3527805190141368e-07 9.033898037416386e-09 1.3493499682474062e-07 8.995666454982708e-09 2.2077500045449087e-10 1.467360914622942e-11 2.5126116160150432e-08 1.672911893119297e-09 2.6405177954910597e-10 1.7587839152228698e-11 2.8969095363503637e-05 2.4805474015153334e-10 1.651979434685321e-11 6.213276326671963e-09 4.137997538440455e-10 5.1604866234581375e-05 7.137375221875433e-11 4.744715598909067e-12 0.001111850299781435 1.3673413011573716e-11 9.121209366152755e-13 0.003220612788185465 0.0073302366810484906</parameters>
    </source>
  </nuclide>
  <nuclide name="Na26" half_life="1.077" decay_modes="1" decay_energy="5528135.0" reactions="0">
    <decay type="beta-" target="Mg26" branching_ratio="1.0"/>
    <source type="discrete" particle="photon">
      <parameters>1254.0 815200.0 1002400.0 1129650.0 1365500.0 1393800.0 1411600.0 1774600.0 1775000.0 1808630.0 1895800.0 1961800.0 2132000.0 2184000.0 2509600.0 2523400.0 2541200.0 2777000.0 2938200.0 3025400.0 3091400.0 4331800.0 4833800.0 8.306722150360562e-08 0.00027030809269747174 0.00901026975658239 0.034110306935633335 0.0016733358119367296 0.0010941041847278615 0.012871813937974843 0.0017376948816266039 0.009653860453481131 0.6371547899297547 0.014158995331772327 0.00019307720906962264 0.004955648366120315 0.0010941041847278615 0.0034753897632532078 0.00901026975658239 0.016089767422468555 0.0008366679059683648 0.0037971851117025784 0.0007079497665886164 0.0019951311603861005 0.0008366679059683648 0.0019307720906962264</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>1180.0 813895.0 815200.0 1001095.0 1002400.0 1128345.0 1129650.0 1364195.0 1365500.0 1410295.0 1411600.0 1773295.0 1773695.0 1774600.0 1775000.0 1807325.0 1808630.0 1894495.0 1895800.0 1960495.0 1961800.0 2130695.0 2132000.0 2508295.0 2509600.0 2539895.0 2541200.0 2775695.0 2777000.0 2936895.0 2938200.0 3090095.0 3091400.0 3229000.0 3638500.0 4330494.0 4331800.0 4453700.0 4519700.0 4832494.0 4833800.0 5004000.0 5021800.0 5035600.0 5413200.0 6415620.0 7545300.0 4.032499891028272e-06 5.16288457052171e-09 3.446428181892764e-10 1.1578196637208371e-07 7.709186803731893e-09 3.5372388292251763e-07 2.3559989000441943e-08 1.2416151724570535e-08 8.277992261651001e-10 9.151859709900113e-08 6.087080811268304e-09 8.236673738910103e-09 4.5662759944965754e-08 5.478604422792356e-10 3.0433795079599273e-09 3.3705488387284027e-06 2.246544073813322e-07 6.017573016003239e-08 4.008553168378064e-09 8.78501301266783e-10 5.846957122255383e-11 1.7443882248743506e-08 1.1615048640512793e-09 1.0287153699229494e-08 6.84790798951412e-10 4.328147436644041e-08 2.876689517463153e-09 2.1000364439805957e-09 1.3978210705013468e-10 8.695553905798904e-09 5.796782791525156e-10 4.229678060018533e-09 2.813932988608557e-10 0.010941041847278617 0.004505134878291195 1.0793015986991907e-09 7.184467308550349e-11 0.0019307720906962264 0.017376948816266037 2.139295476491419e-09 1.4237513396793975e-10 0.017376948816266037 0.010941041847278617 0.0034753897632532078 0.010297451150379874 0.0006435906968987421 0.5670034039677918</parameters>
    </source>
  </nuclide>
  <nuclide name="Na27" half_life="0.301" decay_modes="2" decay_energy="4858385.0" reactions="0">
    <decay type="beta-" target="Mg27" branching_ratio="0.9987"/>
    <decay type="beta-,n" target="Mg26" branching_ratio="0.0013"/>
    <source type="discrete" particle="photon">
      <parameters>955340.0 984660.0 1169400.0 1666700.0 1698000.0 1728900.0 1792700.0 1940000.0 2442300.0 2451800.0 2506000.0 2612800.0 2836100.0 3490700.0 4007600.0 0.02012659919632532 2.012659919632532 0.013686087453501219 0.0022139259115957852 0.2737217490700244 0.0074468417026403695 0.0034215218633753047 0.01066709757405242 0.009660767614236154 0.0006037979758897596 0.005836713766934344 0.004025319839265064 0.0014692417413317484 0.002817723887485545 0.004226585831228318</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>4076400.0 4292700.0 4516000.0 4919000.0 5578100.0 5641900.0 5959500.0 7128940.0 7370940.0 8084310.0 0.00414506619603954 0.00368450328536848 0.0039147847407040096 0.000598731783872378 0.011974635677447559 0.01704082769482922 0.0115140727667765 0.0115140727667765 0.26021804452914893 1.9758148867788474</parameters>
    </source>
  </nuclide>
  <nuclide name="Na28" half_life="0.0305" decay_modes="1" decay_energy="7341820.0" reactions="0">
    <decay type="beta-" target="Mg28" branching_ratio="1.0"/>
    <source type="discrete" particle="photon">
      <parameters>1474000.0 2389000.0 3083500.0 3086000.0 5271700.0 8.408670714989501 4.249787631629829 0.29543978187800946 0.5908795637560189 0.11363068533769596</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>8757800.0 9469800.0 9472300.0 10166800.0 12555960.0 14030000.0 0.11363068533769596 0.5908795637560189 0.29543978187800946 4.317966042832446 3.181659189455487 14.31746635254969</parameters>
    </source>
  </nuclide>
  <nuclide name="Na29" half_life="0.0449" decay_modes="2" decay_energy="8495924.7" reactions="0">
    <decay type="beta-" target="Mg29" branching_ratio="0.785"/>
    <decay type="beta-,n" target="Mg28" branching_ratio="0.215"/>
  </nuclide>
  <nuclide name="Na30" half_life="0.048" decay_modes="4" decay_energy="10445400.0" reactions="0">
    <decay type="beta-" target="Mg30" branching_ratio="0.6884995"/>
    <decay type="beta-,n" target="Mg29" branching_ratio="0.3"/>
    <decay type="beta-,n,n" target="Mg28" branching_ratio="0.0115"/>
    <decay type="beta-,alpha" target="Ne26" branching_ratio="4.999999999588667e-07"/>
  </nuclide>
  <nuclide name="Na31" half_life="0.017" decay_modes="2" decay_energy="10410200.0" reactions="0">
    <decay type="beta-" target="Mg31" branching_ratio="0.63"/>
    <decay type="beta-,n" target="Mg30" branching_ratio="0.37"/>
  </nuclide>
  <nuclide name="Na32" half_life="0.0132" decay_modes="3" decay_energy="12255000.0" reactions="0">
    <decay type="beta-" target="Mg32" branching_ratio="0.68"/>
    <decay type="beta-,n" target="Mg31" branching_ratio="0.24"/>
    <decay type="beta-,n,n" target="Mg30" branching_ratio="0.08"/>
  </nuclide>
  <nuclide name="Na33" half_life="0.008" decay_modes="3" decay_energy="12483000.0" reactions="0">
    <decay type="beta-" target="Mg33" branching_ratio="0.4"/>
    <decay type="beta-,n" target="Mg32" branching_ratio="0.47"/>
    <decay type="beta-,n,n" target="Mg31" branching_ratio="0.13"/>
  </nuclide>
  <nuclide name="Na34" half_life="0.0055" decay_modes="2" decay_energy="15106136.5" reactions="0">
    <decay type="beta-" target="Mg34" branching_ratio="0.85"/>
    <decay type="beta-,n" target="Mg33" branching_ratio="0.15"/>
  </nuclide>
  <nuclide name="Na35" half_life="0.0015" decay_modes="1" decay_energy="14800000.0" reactions="0">
    <decay type="beta-" target="Mg35" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Na36" half_life="1.8e-07" decay_modes="1" decay_energy="17000.0" reactions="0">
    <decay type="beta-" target="Mg36" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Na37" half_life="6e-08" decay_modes="2" decay_energy="17951459.0" reactions="0">
    <decay type="beta-,n" target="Mg36" branching_ratio="0.5"/>
    <decay type="beta-" target="Mg37" branching_ratio="0.5"/>
  </nuclide>
</nuclides>
  "#;

        let nuclides: SerdeNuclideVec = serde_xml_rs::from_str(xml).unwrap();
        assert_eq!(nuclides,get_sodium_xml_serde_data());
}
}


use crate::decay_xml_info_serde::SerdeNuclideVec;
pub fn get_sodium_xml_serde_data() -> SerdeNuclideVec {
    let xml = r#"
    <nuclides>
           <nuclide name="Na18" half_life="1.3e-21" decay_modes="1" decay_energy="1251000.0" reactions="0">
    <decay type="p" target="Ne17" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Na19" half_life="4e-08" decay_modes="1" decay_energy="321250.0" reactions="0">
    <decay type="p" target="Ne18" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Na20" half_life="0.4479" decay_modes="2" decay_energy="8782298.6" reactions="0">
    <decay type="ec/beta+" target="Ne20" branching_ratio="0.7995"/>
    <decay type="ec/beta+,alpha" target="O16" branching_ratio="0.2005"/>
  </nuclide>
  <nuclide name="Na21" half_life="22.49" decay_modes="1" decay_energy="2139475.0" reactions="0">
    <decay type="ec/beta+" target="Ne21" branching_ratio="1.0"/>
    <source type="discrete" particle="photon">
      <parameters>848.0 849.0 350725.0 510998.9 2793940.0 1.6746423554232584e-07 3.349284710846517e-07 0.001562586129586004 0.06157883800617033 1.2328095696930996e-07</parameters>
    </source>
    <source type="discrete" particle="positron">
      <parameters>753060.1 3196473.0 3547200.0 1.2328095696930996e-07 0.001562586129586004 0.02925765311274149</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>820.0 2793073.0 2793940.0 2.7486460193843794e-05 1.6753882052129226e-13 9.27072796409211e-15</parameters>
    </source>
  </nuclide>
  <nuclide name="Na22" half_life="82134970.0" decay_modes="1" decay_energy="2387654.5" reactions="4">
    <decay type="ec/beta+" target="Ne22" branching_ratio="1.0"/>
    <source type="discrete" particle="photon">
      <parameters>848.0 849.0 510998.9 1274537.0 4.6975589383571496e-12 9.395116188889537e-12 1.5181139813033177e-08 8.43414472207654e-09</parameters>
    </source>
    <source type="discrete" particle="positron">
      <parameters>1568443.0 2843020.0 8.434397895790694e-09 4.725909330868074e-12</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>820.0 1273670.0 1274537.0 7.710259467707485e-10 5.364116380805631e-14 2.968818773388466e-15</parameters>
    </source>
    <reaction type="(n,2n)" Q="-10584000.0" target="Na21"/>
    <reaction type="(n,gamma)" Q="12420000.0" target="Na23"/>
    <reaction type="(n,p)" Q="3625000.0" target="Ne22"/>
    <reaction type="(n,a)" Q="1950000.0" target="F19"/>
  </nuclide>
  <nuclide name="Na23" reactions="5">
    <reaction type="(n,2n)" Q="-12414000.0" target="Na22"/>
    <reaction type="(n,p)" Q="-3597000.0" target="Ne23"/>
    <reaction type="(n,a)" Q="-3866000.0" target="F20"/>
    <reaction type="(n,gamma)" Q="6959490.0" target="Na24" branching_ratio="0.232"/>
    <reaction type="(n,gamma)" Q="6959490.0" target="Na24_m1" branching_ratio="0.768"/>
  </nuclide>
  <nuclide name="Na24" half_life="53989.2" decay_modes="1" decay_energy="4676923.1" reactions="0">
    <decay type="beta-" target="Mg24" branching_ratio="1.0"/>
    <source type="discrete" particle="photon">
      <parameters>1254.0 996600.0 1368626.0 2754007.0 2871000.0 3866220.0 4238900.0 7.231115080115924e-13 2.696111591162464e-10 1.283780495247919e-05 1.2820010615977518e-05 3.209656656145791e-11 9.50058370219154e-09 1.0784446364649855e-10</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>1180.0 280330.1 1367321.0 1368626.0 1392561.0 2752702.0 2754007.0 2869695.0 2871000.0 3864915.0 3866220.0 4146778.0 4237594.0 4238900.0 3.5103449948695035e-11 9.757356234683204e-09 3.578125240271327e-12 2.3845181229838305e-13 1.2820010615977518e-05 3.25628269645829e-11 2.1697873167185733e-12 7.638982841626982e-17 5.0828122806724745e-18 1.4402884892522373e-14 9.587039013881483e-16 8.216721039733224e-09 1.434331366498431e-16 9.539921254169263e-18</parameters>
    </source>
  </nuclide>
  <nuclide name="Na24_m1" half_life="0.02018" decay_modes="2" decay_energy="473350.45" reactions="0">
    <decay type="beta-" target="Mg24" branching_ratio="0.0005"/>
    <decay type="IT" target="Na24" branching_ratio="0.9995"/>
    <source type="discrete" particle="photon">
      <parameters>472202.4 34.331050890469044</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>5985807.0 0.017174112501485265</parameters>
    </source>
  </nuclide>
  <nuclide name="Na25" half_life="59.1" decay_modes="1" decay_energy="1942344.5" reactions="0">
    <decay type="beta-" target="Mg25" branching_ratio="1.0"/>
    <source type="discrete" particle="photon">
      <parameters>1254.0 389710.0 585028.0 836841.0 974742.0 989865.0 1379543.0 1611716.0 1964501.0 2216267.0 2801312.0 6.047407731053198e-09 0.001486571998916634 0.001524689229658086 1.2197513837264687e-05 0.001753392614106799 1.95160221396235e-05 2.7139468287913935e-05 0.0011114984484207446 1.7198494510543212e-05 1.0962515561241639e-05 5.793819072700727e-06</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>1180.0 388405.0 389710.0 583723.0 585028.0 835536.0 836841.0 973437.0 974742.0 988560.0 989865.0 1033540.0 1378238.0 1379543.0 1610411.0 1611716.0 1870380.0 1963196.0 1964501.0 2223228.0 2800007.0 2801312.0 2860251.0 3835000.0 2.9357140500830306e-07 1.3527805190141368e-07 9.033898037416386e-09 1.3493499682474062e-07 8.995666454982708e-09 2.2077500045449087e-10 1.467360914622942e-11 2.5126116160150432e-08 1.672911893119297e-09 2.6405177954910597e-10 1.7587839152228698e-11 2.8969095363503637e-05 2.4805474015153334e-10 1.651979434685321e-11 6.213276326671963e-09 4.137997538440455e-10 5.1604866234581375e-05 7.137375221875433e-11 4.744715598909067e-12 0.001111850299781435 1.3673413011573716e-11 9.121209366152755e-13 0.003220612788185465 0.0073302366810484906</parameters>
    </source>
  </nuclide>
  <nuclide name="Na26" half_life="1.077" decay_modes="1" decay_energy="5528135.0" reactions="0">
    <decay type="beta-" target="Mg26" branching_ratio="1.0"/>
    <source type="discrete" particle="photon">
      <parameters>1254.0 815200.0 1002400.0 1129650.0 1365500.0 1393800.0 1411600.0 1774600.0 1775000.0 1808630.0 1895800.0 1961800.0 2132000.0 2184000.0 2509600.0 2523400.0 2541200.0 2777000.0 2938200.0 3025400.0 3091400.0 4331800.0 4833800.0 8.306722150360562e-08 0.00027030809269747174 0.00901026975658239 0.034110306935633335 0.0016733358119367296 0.0010941041847278615 0.012871813937974843 0.0017376948816266039 0.009653860453481131 0.6371547899297547 0.014158995331772327 0.00019307720906962264 0.004955648366120315 0.0010941041847278615 0.0034753897632532078 0.00901026975658239 0.016089767422468555 0.0008366679059683648 0.0037971851117025784 0.0007079497665886164 0.0019951311603861005 0.0008366679059683648 0.0019307720906962264</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>1180.0 813895.0 815200.0 1001095.0 1002400.0 1128345.0 1129650.0 1364195.0 1365500.0 1410295.0 1411600.0 1773295.0 1773695.0 1774600.0 1775000.0 1807325.0 1808630.0 1894495.0 1895800.0 1960495.0 1961800.0 2130695.0 2132000.0 2508295.0 2509600.0 2539895.0 2541200.0 2775695.0 2777000.0 2936895.0 2938200.0 3090095.0 3091400.0 3229000.0 3638500.0 4330494.0 4331800.0 4453700.0 4519700.0 4832494.0 4833800.0 5004000.0 5021800.0 5035600.0 5413200.0 6415620.0 7545300.0 4.032499891028272e-06 5.16288457052171e-09 3.446428181892764e-10 1.1578196637208371e-07 7.709186803731893e-09 3.5372388292251763e-07 2.3559989000441943e-08 1.2416151724570535e-08 8.277992261651001e-10 9.151859709900113e-08 6.087080811268304e-09 8.236673738910103e-09 4.5662759944965754e-08 5.478604422792356e-10 3.0433795079599273e-09 3.3705488387284027e-06 2.246544073813322e-07 6.017573016003239e-08 4.008553168378064e-09 8.78501301266783e-10 5.846957122255383e-11 1.7443882248743506e-08 1.1615048640512793e-09 1.0287153699229494e-08 6.84790798951412e-10 4.328147436644041e-08 2.876689517463153e-09 2.1000364439805957e-09 1.3978210705013468e-10 8.695553905798904e-09 5.796782791525156e-10 4.229678060018533e-09 2.813932988608557e-10 0.010941041847278617 0.004505134878291195 1.0793015986991907e-09 7.184467308550349e-11 0.0019307720906962264 0.017376948816266037 2.139295476491419e-09 1.4237513396793975e-10 0.017376948816266037 0.010941041847278617 0.0034753897632532078 0.010297451150379874 0.0006435906968987421 0.5670034039677918</parameters>
    </source>
  </nuclide>
  <nuclide name="Na27" half_life="0.301" decay_modes="2" decay_energy="4858385.0" reactions="0">
    <decay type="beta-" target="Mg27" branching_ratio="0.9987"/>
    <decay type="beta-,n" target="Mg26" branching_ratio="0.0013"/>
    <source type="discrete" particle="photon">
      <parameters>955340.0 984660.0 1169400.0 1666700.0 1698000.0 1728900.0 1792700.0 1940000.0 2442300.0 2451800.0 2506000.0 2612800.0 2836100.0 3490700.0 4007600.0 0.02012659919632532 2.012659919632532 0.013686087453501219 0.0022139259115957852 0.2737217490700244 0.0074468417026403695 0.0034215218633753047 0.01066709757405242 0.009660767614236154 0.0006037979758897596 0.005836713766934344 0.004025319839265064 0.0014692417413317484 0.002817723887485545 0.004226585831228318</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>4076400.0 4292700.0 4516000.0 4919000.0 5578100.0 5641900.0 5959500.0 7128940.0 7370940.0 8084310.0 0.00414506619603954 0.00368450328536848 0.0039147847407040096 0.000598731783872378 0.011974635677447559 0.01704082769482922 0.0115140727667765 0.0115140727667765 0.26021804452914893 1.9758148867788474</parameters>
    </source>
  </nuclide>
  <nuclide name="Na28" half_life="0.0305" decay_modes="1" decay_energy="7341820.0" reactions="0">
    <decay type="beta-" target="Mg28" branching_ratio="1.0"/>
    <source type="discrete" particle="photon">
      <parameters>1474000.0 2389000.0 3083500.0 3086000.0 5271700.0 8.408670714989501 4.249787631629829 0.29543978187800946 0.5908795637560189 0.11363068533769596</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>8757800.0 9469800.0 9472300.0 10166800.0 12555960.0 14030000.0 0.11363068533769596 0.5908795637560189 0.29543978187800946 4.317966042832446 3.181659189455487 14.31746635254969</parameters>
    </source>
  </nuclide>
  <nuclide name="Na29" half_life="0.0449" decay_modes="2" decay_energy="8495924.7" reactions="0">
    <decay type="beta-" target="Mg29" branching_ratio="0.785"/>
    <decay type="beta-,n" target="Mg28" branching_ratio="0.215"/>
  </nuclide>
  <nuclide name="Na30" half_life="0.048" decay_modes="4" decay_energy="10445400.0" reactions="0">
    <decay type="beta-" target="Mg30" branching_ratio="0.6884995"/>
    <decay type="beta-,n" target="Mg29" branching_ratio="0.3"/>
    <decay type="beta-,n,n" target="Mg28" branching_ratio="0.0115"/>
    <decay type="beta-,alpha" target="Ne26" branching_ratio="4.999999999588667e-07"/>
  </nuclide>
  <nuclide name="Na31" half_life="0.017" decay_modes="2" decay_energy="10410200.0" reactions="0">
    <decay type="beta-" target="Mg31" branching_ratio="0.63"/>
    <decay type="beta-,n" target="Mg30" branching_ratio="0.37"/>
  </nuclide>
  <nuclide name="Na32" half_life="0.0132" decay_modes="3" decay_energy="12255000.0" reactions="0">
    <decay type="beta-" target="Mg32" branching_ratio="0.68"/>
    <decay type="beta-,n" target="Mg31" branching_ratio="0.24"/>
    <decay type="beta-,n,n" target="Mg30" branching_ratio="0.08"/>
  </nuclide>
  <nuclide name="Na33" half_life="0.008" decay_modes="3" decay_energy="12483000.0" reactions="0">
    <decay type="beta-" target="Mg33" branching_ratio="0.4"/>
    <decay type="beta-,n" target="Mg32" branching_ratio="0.47"/>
    <decay type="beta-,n,n" target="Mg31" branching_ratio="0.13"/>
  </nuclide>
  <nuclide name="Na34" half_life="0.0055" decay_modes="2" decay_energy="15106136.5" reactions="0">
    <decay type="beta-" target="Mg34" branching_ratio="0.85"/>
    <decay type="beta-,n" target="Mg33" branching_ratio="0.15"/>
  </nuclide>
  <nuclide name="Na35" half_life="0.0015" decay_modes="1" decay_energy="14800000.0" reactions="0">
    <decay type="beta-" target="Mg35" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Na36" half_life="1.8e-07" decay_modes="1" decay_energy="17000.0" reactions="0">
    <decay type="beta-" target="Mg36" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Na37" half_life="6e-08" decay_modes="2" decay_energy="17951459.0" reactions="0">
    <decay type="beta-,n" target="Mg36" branching_ratio="0.5"/>
    <decay type="beta-" target="Mg37" branching_ratio="0.5"/>
  </nuclide>
</nuclides>
  "#;

    let nuclides: SerdeNuclideVec = serde_xml_rs::from_str(xml).unwrap();

    return nuclides;
}
