// notes, code was done from vibe coding using AI, then 
// modified


#[cfg(test)]
mod parsing_tests {
#[test]
fn serde_nuclide_test() {
    use crate::decay_xml_info_serde::SerdeNuclideVec;
    let xml = r#"
    <nuclides>
                <nuclide name="Si22" half_life="0.029" decay_modes="2" decay_energy="10061386.0" reactions="0">
    <decay type="ec/beta+" target="Al22" branching_ratio="0.68"/>
    <decay type="ec/beta+,p" target="Mg21" branching_ratio="0.32"/>
  </nuclide>
  <nuclide name="Si23" half_life="0.0423" decay_modes="3" decay_energy="12258296.0" reactions="0">
    <decay type="ec/beta+" target="Al23" branching_ratio="0.23"/>
    <decay type="ec/beta+,p" target="Mg22" branching_ratio="0.73"/>
    <decay type="ec/beta+,p,p" target="Na21" branching_ratio="0.04"/>
  </nuclide>
  <nuclide name="Si24" half_life="0.14" decay_modes="2" decay_energy="7013225.6" reactions="0">
    <decay type="ec/beta+" target="Al24" branching_ratio="0.62"/>
    <decay type="ec/beta+,p" target="Mg23" branching_ratio="0.38"/>
  </nuclide>
  <nuclide name="Si25" half_life="0.22" decay_modes="2" decay_energy="8270516.5" reactions="0">
    <decay type="ec/beta+" target="Al25" branching_ratio="0.65"/>
    <decay type="ec/beta+,p" target="Mg24" branching_ratio="0.35"/>
  </nuclide>
  <nuclide name="Si26" half_life="2.234" decay_modes="2" decay_energy="2874989.0" reactions="0">
    <decay type="ec/beta+" target="Al26" branching_ratio="0.0003944"/>
    <decay type="ec/beta+" target="Al26_m1" branching_ratio="0.9996056"/>
    <source type="discrete" particle="photon">
      <parameters>1486.0 1487.0 1554.0 416848.0 510998.9 829420.0 1342145.0 1433730.0 1622260.0 1654730.0 1843260.0 2323070.0 2511590.0 2665920.0 3495250.0 3.259206587997613e-06 6.568940937120646e-06 5.912045912593209e-08 0.0001223091399179635 0.620357728719219 0.06794952217664639 3.329526586655673e-05 3.261577064479026e-05 0.008459715510992475 9.512933104730496e-05 0.0008011248664626609 9.512933104730495e-06 0.0001916176525381428 5.096214163248479e-06 1.5288642489745436e-06</parameters>
    </source>
    <source type="discrete" particle="positron">
      <parameters>1342190.0 2325970.0 2994360.0 3215380.0 4008261.0 4837695.0 2.0167666399461258e-06 0.0001917479666902624 0.000899788193206733 0.008470419887773728 0.06794952217664639 0.2327038430707068</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>1390.0 415288.4 416848.0 827860.4 829420.0 1432170.0 1433730.0 1620700.0 1622260.0 1653170.0 1654730.0 1841700.0 1843260.0 2321510.0 2323070.0 2510030.0 2511590.0 3493690.0 3495250.0 0.00024287393182826898 4.219665327169741e-08 3.0266619143555662e-09 1.59681377115119e-06 1.1435904582329587e-07 3.4703186171492657e-10 2.4869525116652578e-11 5.9048817369445384e-08 4.232395778744663e-09 7.553268885156011e-10 5.410953867796379e-11 4.518345363827855e-09 3.239749443998994e-10 4.0525219134868207e-11 2.9054408976078016e-12 6.629971398363322e-10 4.743124865362527e-11 3.2564886071105475e-12 2.3269329382982093e-13</parameters>
    </source>
  </nuclide>
  <nuclide name="Si27" half_life="4.16" decay_modes="1" decay_energy="2743135.0" reactions="0">
    <decay type="ec/beta+" target="Al27" branching_ratio="1.0"/>
    <source type="discrete" particle="photon">
      <parameters>45.29212 114.1749 1468.7 1469.17 1545.02 1545.03 170700.0 510998.9 843760.0 1014460.0 1719500.0 2210500.0 2734000.0 2981100.0 1.4011790303347417e-08 2.6796986689488283e-10 1.2952631419506508e-06 2.5709445428066467e-06 7.914614437826167e-09 1.5710715703749234e-08 2.832572612865161e-07 0.333044556993255 8.33109592019165e-07 9.664071267422315e-06 2.0327874045267627e-05 0.0002999194531268994 5.49852330732649e-06 4.3321698784996576e-05</parameters>
    </source>
    <source type="discrete" particle="positron">
      <parameters>1808160.0 1831260.0 2078360.0 2601860.0 3797900.0 3968600.0 4812360.0 4.99865755211499e-07 4.282183302978508e-05 2.8325726128651613e-05 0.0003015856723109377 9.99731510422998e-06 7.497986328172486e-06 0.16623868799150418</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>28.50139 65.229 1371.324 1447.351 1530.974 4.3023095645025074e-05 0.00020039051611906423 9.578924134679587e-05 4.9492074991711e-06 8.263690689320563e-08</parameters>
    </source>
  </nuclide>
  <nuclide name="Si28" reactions="4">
    <reaction type="(n,2n)" Q="-17177000.0" target="Si27"/>
    <reaction type="(n,gamma)" Q="8473900.0" target="Si29"/>
    <reaction type="(n,p)" Q="-3860000.0" target="Al28"/>
    <reaction type="(n,a)" Q="-2650000.0" target="Mg25"/>
  </nuclide>
  <nuclide name="Si29" reactions="4">
    <reaction type="(n,2n)" Q="-8473900.0" target="Si28"/>
    <reaction type="(n,gamma)" Q="10610000.0" target="Si30"/>
    <reaction type="(n,p)" Q="-2898500.0" target="Al29"/>
    <reaction type="(n,a)" Q="-34700.0" target="Mg26"/>
  </nuclide>
  <nuclide name="Si30" reactions="4">
    <reaction type="(n,2n)" Q="-10610000.0" target="Si29"/>
    <reaction type="(n,gamma)" Q="6592000.0" target="Si31"/>
    <reaction type="(n,p)" Q="-7751600.0" target="Al30"/>
    <reaction type="(n,a)" Q="-4200200.0" target="Mg27"/>
  </nuclide>
  <nuclide name="Si31" half_life="9438.0" decay_modes="1" decay_energy="596447.405" reactions="4">
    <decay type="beta-" target="P31" branching_ratio="1.0"/>
    <source type="discrete" particle="photon">
      <parameters>86.02708 178.7859 1991.26 1992.22 2122.02 2122.07 1266150.0 3.3192775315905035e-16 1.595942373582957e-17 1.7482393971395534e-14 3.46440659956089e-14 5.399446150732465e-16 1.0704710065360875e-15 5.140951752404765e-08</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>34.01767 117.5728 1832.318 1966.379 2104.363 225350.0 1264004.0 1266150.0 1491500.0 3.9312351299680784e-13 1.6699875016728427e-12 7.401736695042284e-13 7.306702529790688e-14 1.8429944821531623e-15 5.140951752404765e-08 8.688208461564052e-13 6.95879229205509e-14 7.339075837397259e-05</parameters>
    </source>
    <reaction type="(n,2n)" Q="-6587000.0" target="Si30"/>
    <reaction type="(n,gamma)" Q="9203430.0" target="Si32"/>
    <reaction type="(n,p)" Q="-7212600.0" target="Al31"/>
    <reaction type="(n,a)" Q="-2283500.0" target="Mg28"/>
  </nuclide>
  <nuclide name="Si32" half_life="4828310000.0" decay_modes="1" decay_energy="68430.0" reactions="4">
    <decay type="beta-" target="P32" branching_ratio="1.0"/>
    <source type="discrete" particle="electron">
      <parameters>223930.0 1.4355896381134294e-10</parameters>
    </source>
    <reaction type="(n,2n)" Q="-9203000.0" target="Si31"/>
    <reaction type="(n,gamma)" Q="4482430.0" target="Si33"/>
    <reaction type="(n,p)" Q="-12236600.0" target="Al32"/>
    <reaction type="(n,a)" Q="-7773500.0" target="Mg29"/>
  </nuclide>
  <nuclide name="Si33" half_life="6.11" decay_modes="1" decay_energy="2757379.4" reactions="0">
    <decay type="beta-" target="P33" branching_ratio="1.0"/>
    <source type="discrete" particle="photon">
      <parameters>415800.0 1431500.0 1642900.0 1847000.0 2058900.0 2195800.0 2377100.0 2537500.0 2616300.0 3008600.0 3200000.0 3275100.0 3342000.0 3758000.0 4193400.0 0.00038764057544571736 0.0007579241101998355 4.3392601728998206e-05 0.005785680230533094 3.760692149846511e-05 1.4464200576332738e-05 9.257088368852952e-06 0.0005380682614395778 3.760692149846511e-05 4.049976161373166e-05 4.3392601728998206e-05 3.182124126793202e-05 3.471408138319857e-05 2.6035561037398926e-05 2.2274868887552413e-05</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>655000.0 797000.0 988899.9 1620300.0 1651300.0 1797200.0 2217700.0 2354600.0 2569700.0 3307400.0 3997500.0 4413600.0 5845000.0 6.239459072143534e-05 4.367621350500473e-05 3.9705648640913395e-05 9.075576832208777e-06 2.268894208052194e-05 3.9705648640913395e-05 1.4747812352339259e-05 7.941129728182679e-05 3.176451891273071e-05 0.0005331901388922656 0.006012569651338314 0.00024957836288574134 0.1062976936472453</parameters>
    </source>
  </nuclide>
  <nuclide name="Si34" half_life="2.77" decay_modes="1" decay_energy="3061296.0" reactions="0">
    <decay type="beta-" target="P34" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Si35" half_life="0.78" decay_modes="1" decay_energy="7208243.0" reactions="0">
    <decay type="beta-" target="P35" branching_ratio="1.0"/>
    <source type="discrete" particle="photon">
      <parameters>241300.0 392400.0 633700.0 768300.0 1009600.0 1458900.0 1473300.0 1714600.0 1934700.0 2386300.0 3173300.0 3859500.0 4100700.0 0.23993556250151954 0.1391626262508813 0.0527858237503343 0.03838969000024312 0.05758453500036468 0.02879226750018234 0.04078904562525832 0.0527858237503343 0.08397744687553182 0.28072460812677785 0.08397744687553182 0.2903220306268386 0.3239130093770513</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>4464200.0 4940100.0 5630700.0 6006600.0 6399000.0 6640300.0 0.08353312175978828 0.11019262870440155 0.0959742250006078 0.19017114953824138 0.4087791064840703 0.008886502314871094</parameters>
    </source>
  </nuclide>
  <nuclide name="Si36" half_life="0.45" decay_modes="2" decay_energy="5027694.8" reactions="0">
    <decay type="beta-" target="P36" branching_ratio="0.9"/>
    <decay type="beta-,n" target="P35" branching_ratio="0.1"/>
  </nuclide>
  <nuclide name="Si37" half_life="0.09" decay_modes="2" decay_energy="7576179.6" reactions="0">
    <decay type="beta-" target="P37" branching_ratio="0.83"/>
    <decay type="beta-,n" target="P36" branching_ratio="0.17"/>
  </nuclide>
  <nuclide name="Si38" half_life="1e-06" decay_modes="2" decay_energy="6061651.7" reactions="0">
    <decay type="beta-" target="P38" branching_ratio="0.5"/>
    <decay type="beta-,n" target="P37" branching_ratio="0.5"/>
  </nuclide>
  <nuclide name="Si39" half_life="0.0475" decay_modes="2" decay_energy="8269017.0" reactions="0">
    <decay type="beta-" target="P39" branching_ratio="0.5"/>
    <decay type="beta-,n" target="P38" branching_ratio="0.5"/>
  </nuclide>
  <nuclide name="Si40" half_life="0.033" decay_modes="2" decay_energy="8352418.0" reactions="0">
    <decay type="beta-" target="P40" branching_ratio="0.5"/>
    <decay type="beta-,n" target="P39" branching_ratio="0.5"/>
  </nuclide>
  <nuclide name="Si41" half_life="0.02" decay_modes="1" decay_energy="11399624.0" reactions="0">
    <decay type="beta-" target="P41" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Si42" half_life="0.0125" decay_modes="1" decay_energy="10368000.0" reactions="0">
    <decay type="beta-" target="P42" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Si43" half_life="6e-08" decay_modes="2" decay_energy="12772584.0" reactions="0">
    <decay type="beta-" target="P43" branching_ratio="0.5"/>
    <decay type="beta-,n" target="P42" branching_ratio="0.5"/>
  </nuclide>
  <nuclide name="Si44" half_life="3.6e-07" decay_modes="2" decay_energy="13151291.0" reactions="0">
    <decay type="beta-,n" target="P43" branching_ratio="0.5"/>
    <decay type="beta-" target="P44" branching_ratio="0.5"/>
  </nuclide>
</nuclides>
  "#;

        let nuclides: SerdeNuclideVec = serde_xml_rs::from_str(xml).unwrap();
        use crate::decay_xml_info_serde::silicon::get_silicon_xml_serde_data;
        assert_eq!(nuclides,get_silicon_xml_serde_data());

    }
}


use crate::decay_xml_info_serde::SerdeNuclideVec;
pub fn get_silicon_xml_serde_data() -> SerdeNuclideVec {
    
    let xml = r#"
    <nuclides>
                <nuclide name="Si22" half_life="0.029" decay_modes="2" decay_energy="10061386.0" reactions="0">
    <decay type="ec/beta+" target="Al22" branching_ratio="0.68"/>
    <decay type="ec/beta+,p" target="Mg21" branching_ratio="0.32"/>
  </nuclide>
  <nuclide name="Si23" half_life="0.0423" decay_modes="3" decay_energy="12258296.0" reactions="0">
    <decay type="ec/beta+" target="Al23" branching_ratio="0.23"/>
    <decay type="ec/beta+,p" target="Mg22" branching_ratio="0.73"/>
    <decay type="ec/beta+,p,p" target="Na21" branching_ratio="0.04"/>
  </nuclide>
  <nuclide name="Si24" half_life="0.14" decay_modes="2" decay_energy="7013225.6" reactions="0">
    <decay type="ec/beta+" target="Al24" branching_ratio="0.62"/>
    <decay type="ec/beta+,p" target="Mg23" branching_ratio="0.38"/>
  </nuclide>
  <nuclide name="Si25" half_life="0.22" decay_modes="2" decay_energy="8270516.5" reactions="0">
    <decay type="ec/beta+" target="Al25" branching_ratio="0.65"/>
    <decay type="ec/beta+,p" target="Mg24" branching_ratio="0.35"/>
  </nuclide>
  <nuclide name="Si26" half_life="2.234" decay_modes="2" decay_energy="2874989.0" reactions="0">
    <decay type="ec/beta+" target="Al26" branching_ratio="0.0003944"/>
    <decay type="ec/beta+" target="Al26_m1" branching_ratio="0.9996056"/>
    <source type="discrete" particle="photon">
      <parameters>1486.0 1487.0 1554.0 416848.0 510998.9 829420.0 1342145.0 1433730.0 1622260.0 1654730.0 1843260.0 2323070.0 2511590.0 2665920.0 3495250.0 3.259206587997613e-06 6.568940937120646e-06 5.912045912593209e-08 0.0001223091399179635 0.620357728719219 0.06794952217664639 3.329526586655673e-05 3.261577064479026e-05 0.008459715510992475 9.512933104730496e-05 0.0008011248664626609 9.512933104730495e-06 0.0001916176525381428 5.096214163248479e-06 1.5288642489745436e-06</parameters>
    </source>
    <source type="discrete" particle="positron">
      <parameters>1342190.0 2325970.0 2994360.0 3215380.0 4008261.0 4837695.0 2.0167666399461258e-06 0.0001917479666902624 0.000899788193206733 0.008470419887773728 0.06794952217664639 0.2327038430707068</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>1390.0 415288.4 416848.0 827860.4 829420.0 1432170.0 1433730.0 1620700.0 1622260.0 1653170.0 1654730.0 1841700.0 1843260.0 2321510.0 2323070.0 2510030.0 2511590.0 3493690.0 3495250.0 0.00024287393182826898 4.219665327169741e-08 3.0266619143555662e-09 1.59681377115119e-06 1.1435904582329587e-07 3.4703186171492657e-10 2.4869525116652578e-11 5.9048817369445384e-08 4.232395778744663e-09 7.553268885156011e-10 5.410953867796379e-11 4.518345363827855e-09 3.239749443998994e-10 4.0525219134868207e-11 2.9054408976078016e-12 6.629971398363322e-10 4.743124865362527e-11 3.2564886071105475e-12 2.3269329382982093e-13</parameters>
    </source>
  </nuclide>
  <nuclide name="Si27" half_life="4.16" decay_modes="1" decay_energy="2743135.0" reactions="0">
    <decay type="ec/beta+" target="Al27" branching_ratio="1.0"/>
    <source type="discrete" particle="photon">
      <parameters>45.29212 114.1749 1468.7 1469.17 1545.02 1545.03 170700.0 510998.9 843760.0 1014460.0 1719500.0 2210500.0 2734000.0 2981100.0 1.4011790303347417e-08 2.6796986689488283e-10 1.2952631419506508e-06 2.5709445428066467e-06 7.914614437826167e-09 1.5710715703749234e-08 2.832572612865161e-07 0.333044556993255 8.33109592019165e-07 9.664071267422315e-06 2.0327874045267627e-05 0.0002999194531268994 5.49852330732649e-06 4.3321698784996576e-05</parameters>
    </source>
    <source type="discrete" particle="positron">
      <parameters>1808160.0 1831260.0 2078360.0 2601860.0 3797900.0 3968600.0 4812360.0 4.99865755211499e-07 4.282183302978508e-05 2.8325726128651613e-05 0.0003015856723109377 9.99731510422998e-06 7.497986328172486e-06 0.16623868799150418</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>28.50139 65.229 1371.324 1447.351 1530.974 4.3023095645025074e-05 0.00020039051611906423 9.578924134679587e-05 4.9492074991711e-06 8.263690689320563e-08</parameters>
    </source>
  </nuclide>
  <nuclide name="Si28" reactions="4">
    <reaction type="(n,2n)" Q="-17177000.0" target="Si27"/>
    <reaction type="(n,gamma)" Q="8473900.0" target="Si29"/>
    <reaction type="(n,p)" Q="-3860000.0" target="Al28"/>
    <reaction type="(n,a)" Q="-2650000.0" target="Mg25"/>
  </nuclide>
  <nuclide name="Si29" reactions="4">
    <reaction type="(n,2n)" Q="-8473900.0" target="Si28"/>
    <reaction type="(n,gamma)" Q="10610000.0" target="Si30"/>
    <reaction type="(n,p)" Q="-2898500.0" target="Al29"/>
    <reaction type="(n,a)" Q="-34700.0" target="Mg26"/>
  </nuclide>
  <nuclide name="Si30" reactions="4">
    <reaction type="(n,2n)" Q="-10610000.0" target="Si29"/>
    <reaction type="(n,gamma)" Q="6592000.0" target="Si31"/>
    <reaction type="(n,p)" Q="-7751600.0" target="Al30"/>
    <reaction type="(n,a)" Q="-4200200.0" target="Mg27"/>
  </nuclide>
  <nuclide name="Si31" half_life="9438.0" decay_modes="1" decay_energy="596447.405" reactions="4">
    <decay type="beta-" target="P31" branching_ratio="1.0"/>
    <source type="discrete" particle="photon">
      <parameters>86.02708 178.7859 1991.26 1992.22 2122.02 2122.07 1266150.0 3.3192775315905035e-16 1.595942373582957e-17 1.7482393971395534e-14 3.46440659956089e-14 5.399446150732465e-16 1.0704710065360875e-15 5.140951752404765e-08</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>34.01767 117.5728 1832.318 1966.379 2104.363 225350.0 1264004.0 1266150.0 1491500.0 3.9312351299680784e-13 1.6699875016728427e-12 7.401736695042284e-13 7.306702529790688e-14 1.8429944821531623e-15 5.140951752404765e-08 8.688208461564052e-13 6.95879229205509e-14 7.339075837397259e-05</parameters>
    </source>
    <reaction type="(n,2n)" Q="-6587000.0" target="Si30"/>
    <reaction type="(n,gamma)" Q="9203430.0" target="Si32"/>
    <reaction type="(n,p)" Q="-7212600.0" target="Al31"/>
    <reaction type="(n,a)" Q="-2283500.0" target="Mg28"/>
  </nuclide>
  <nuclide name="Si32" half_life="4828310000.0" decay_modes="1" decay_energy="68430.0" reactions="4">
    <decay type="beta-" target="P32" branching_ratio="1.0"/>
    <source type="discrete" particle="electron">
      <parameters>223930.0 1.4355896381134294e-10</parameters>
    </source>
    <reaction type="(n,2n)" Q="-9203000.0" target="Si31"/>
    <reaction type="(n,gamma)" Q="4482430.0" target="Si33"/>
    <reaction type="(n,p)" Q="-12236600.0" target="Al32"/>
    <reaction type="(n,a)" Q="-7773500.0" target="Mg29"/>
  </nuclide>
  <nuclide name="Si33" half_life="6.11" decay_modes="1" decay_energy="2757379.4" reactions="0">
    <decay type="beta-" target="P33" branching_ratio="1.0"/>
    <source type="discrete" particle="photon">
      <parameters>415800.0 1431500.0 1642900.0 1847000.0 2058900.0 2195800.0 2377100.0 2537500.0 2616300.0 3008600.0 3200000.0 3275100.0 3342000.0 3758000.0 4193400.0 0.00038764057544571736 0.0007579241101998355 4.3392601728998206e-05 0.005785680230533094 3.760692149846511e-05 1.4464200576332738e-05 9.257088368852952e-06 0.0005380682614395778 3.760692149846511e-05 4.049976161373166e-05 4.3392601728998206e-05 3.182124126793202e-05 3.471408138319857e-05 2.6035561037398926e-05 2.2274868887552413e-05</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>655000.0 797000.0 988899.9 1620300.0 1651300.0 1797200.0 2217700.0 2354600.0 2569700.0 3307400.0 3997500.0 4413600.0 5845000.0 6.239459072143534e-05 4.367621350500473e-05 3.9705648640913395e-05 9.075576832208777e-06 2.268894208052194e-05 3.9705648640913395e-05 1.4747812352339259e-05 7.941129728182679e-05 3.176451891273071e-05 0.0005331901388922656 0.006012569651338314 0.00024957836288574134 0.1062976936472453</parameters>
    </source>
  </nuclide>
  <nuclide name="Si34" half_life="2.77" decay_modes="1" decay_energy="3061296.0" reactions="0">
    <decay type="beta-" target="P34" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Si35" half_life="0.78" decay_modes="1" decay_energy="7208243.0" reactions="0">
    <decay type="beta-" target="P35" branching_ratio="1.0"/>
    <source type="discrete" particle="photon">
      <parameters>241300.0 392400.0 633700.0 768300.0 1009600.0 1458900.0 1473300.0 1714600.0 1934700.0 2386300.0 3173300.0 3859500.0 4100700.0 0.23993556250151954 0.1391626262508813 0.0527858237503343 0.03838969000024312 0.05758453500036468 0.02879226750018234 0.04078904562525832 0.0527858237503343 0.08397744687553182 0.28072460812677785 0.08397744687553182 0.2903220306268386 0.3239130093770513</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>4464200.0 4940100.0 5630700.0 6006600.0 6399000.0 6640300.0 0.08353312175978828 0.11019262870440155 0.0959742250006078 0.19017114953824138 0.4087791064840703 0.008886502314871094</parameters>
    </source>
  </nuclide>
  <nuclide name="Si36" half_life="0.45" decay_modes="2" decay_energy="5027694.8" reactions="0">
    <decay type="beta-" target="P36" branching_ratio="0.9"/>
    <decay type="beta-,n" target="P35" branching_ratio="0.1"/>
  </nuclide>
  <nuclide name="Si37" half_life="0.09" decay_modes="2" decay_energy="7576179.6" reactions="0">
    <decay type="beta-" target="P37" branching_ratio="0.83"/>
    <decay type="beta-,n" target="P36" branching_ratio="0.17"/>
  </nuclide>
  <nuclide name="Si38" half_life="1e-06" decay_modes="2" decay_energy="6061651.7" reactions="0">
    <decay type="beta-" target="P38" branching_ratio="0.5"/>
    <decay type="beta-,n" target="P37" branching_ratio="0.5"/>
  </nuclide>
  <nuclide name="Si39" half_life="0.0475" decay_modes="2" decay_energy="8269017.0" reactions="0">
    <decay type="beta-" target="P39" branching_ratio="0.5"/>
    <decay type="beta-,n" target="P38" branching_ratio="0.5"/>
  </nuclide>
  <nuclide name="Si40" half_life="0.033" decay_modes="2" decay_energy="8352418.0" reactions="0">
    <decay type="beta-" target="P40" branching_ratio="0.5"/>
    <decay type="beta-,n" target="P39" branching_ratio="0.5"/>
  </nuclide>
  <nuclide name="Si41" half_life="0.02" decay_modes="1" decay_energy="11399624.0" reactions="0">
    <decay type="beta-" target="P41" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Si42" half_life="0.0125" decay_modes="1" decay_energy="10368000.0" reactions="0">
    <decay type="beta-" target="P42" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Si43" half_life="6e-08" decay_modes="2" decay_energy="12772584.0" reactions="0">
    <decay type="beta-" target="P43" branching_ratio="0.5"/>
    <decay type="beta-,n" target="P42" branching_ratio="0.5"/>
  </nuclide>
  <nuclide name="Si44" half_life="3.6e-07" decay_modes="2" decay_energy="13151291.0" reactions="0">
    <decay type="beta-,n" target="P43" branching_ratio="0.5"/>
    <decay type="beta-" target="P44" branching_ratio="0.5"/>
  </nuclide>
</nuclides>
  "#;

    let nuclides: SerdeNuclideVec = serde_xml_rs::from_str(xml).unwrap();

    return nuclides;
}
