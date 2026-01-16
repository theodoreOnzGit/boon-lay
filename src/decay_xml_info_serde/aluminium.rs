


// notes, code was done from vibe coding using AI, then 
// modified


#[cfg(test)]
mod parsing_tests {
#[test]
fn serde_nuclide_test() {
    use crate::decay_xml_info_serde::SerdeNuclideVec;
    let xml = r#"
    <nuclides>
    <nuclide name="Al21" half_life="3.5e-08" decay_modes="1" decay_energy="2287000.0" reactions="0">
    <decay type="p" target="Mg20" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Al22" half_life="0.059" decay_modes="3" decay_energy="10839791.0" reactions="0">
    <decay type="ec/beta+" target="Mg22" branching_ratio="0.3969"/>
    <decay type="ec/beta+,p" target="Na21" branching_ratio="0.6"/>
    <decay type="ec/beta+,alpha" target="Ne18" branching_ratio="0.0031000000000001027"/>
  </nuclide>
  <nuclide name="Al23" half_life="0.47" decay_modes="2" decay_energy="8135024.27" reactions="0">
    <decay type="ec/beta+" target="Mg23" branching_ratio="0.9878"/>
    <decay type="ec/beta+,p" target="Na22" branching_ratio="0.0122"/>
  </nuclide>
  <nuclide name="Al24" half_life="2.053" decay_modes="1" decay_energy="11503434.0" reactions="0">
    <decay type="ec/beta+" target="Mg24" branching_ratio="1.0"/>
    <source type="discrete" particle="photon">
      <parameters>1254.0 510998.9 775400.0 822000.0 863000.0 996830.0 1059780.0 1076860.0 1090670.0 1274710.0 1368626.0 1704800.0 1771920.0 1887520.0 1899700.0 1952380.0 2136580.0 2381000.0 2428970.0 2577400.0 2754007.0 2869500.0 3203880.0 3378300.0 3493300.0 3505610.0 3866140.0 4200540.0 4237960.0 4280620.0 4316000.0 4641190.0 5060700.0 5177510.0 5340300.0 5392680.0 5979500.0 6246890.0 7069500.0 7348200.0 7615200.0 7930870.0 8085700.0 8146000.0 9450100.0 9943500.0 2.986724946203902e-06 0.6752509509190489 0.0001789420388196644 7.090156255118779e-05 7.427782743457768e-05 0.00046254828902441553 0.00096223549176612 0.050103770869506035 0.00047267708367458523 0.0003578840776393288 0.3241214288054299 5.4020238134238316e-05 0.0013505059533559579 0.0001890708334698341 0.002768537204379714 0.00031736889903865006 0.0005672125004095023 0.0001249218006854261 0.0026132290197437786 0.00010128794650169682 0.13910211319566365 0.0037037625770787144 0.010398895840840876 0.00014517938998576547 0.0001350505953355958 0.006685004469111992 0.017759153286630844 0.013572584831227376 0.01218831622903752 0.0022283348230373303 0.047942961344136496 0.01154682590119344 0.0001215455358020362 0.0033087395857220965 0.0003882704615898379 0.06178564736603507 0.00031399263415526023 0.001823183037030543 0.14517938998576546 0.0005165685271586538 0.0007562833338793364 0.004524194943742459 6.75252976677979e-05 9.453541673491704e-05 0.0003713891371728884 9.115915185152715e-05</parameters>
    </source>
    <source type="discrete" particle="positron">
      <parameters>906000.0 1717400.0 1758500.0 1828900.0 2182400.0 2563600.0 2661210.0 3057300.0 3301980.0 4361720.0 4420190.0 4576850.0 5438640.0 6065650.0 7867160.0 8642880.0 9755111.0 6.703979077756643e-08 2.513992154158741e-06 3.3519895388783213e-06 3.6871884927661533e-07 8.715172801083635e-05 8.715172801083635e-05 2.4134324679923914e-05 0.0003687188492766153 0.002245832991048475 0.12402361293849788 0.0001106156547829846 0.008379973847195802 0.16759947694391605 0.00016759947694391605 0.004022387446653985 0.004692785354429649 0.025810319449363073</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>1180.0 774095.1 775400.0 820695.0 822000.0 1058475.0 1059780.0 1075555.0 1076860.0 1089365.0 1090670.0 1273405.0 1274710.0 1367321.0 1368626.0 1770615.0 1771920.0 1898395.0 1899700.0 1951075.0 1952380.0 2379695.0 2381000.0 2752702.0 2754007.0 2868195.0 2869500.0 3376995.0 3378300.0 3491995.0 3493300.0 3864835.0 3866140.0 4199235.0 4200540.0 4236655.0 4237960.0 4639885.0 4641190.0 5059395.0 5060700.0 0.00014499055549083747 3.739888611330986e-09 2.4940941370684823e-10 8.933596881449662e-10 5.948641098044654e-11 1.1142686994651669e-08 7.424609054467381e-10 5.636674222819429e-07 3.751770362708612e-08 7.364308963650038e-09 4.906860805625868e-10 2.9740166851828223e-09 1.9819620219666032e-10 3.0110880736024434e-06 2.0066357657344162e-07 7.427782743457767e-09 4.944202295236162e-10 8.001072520657373e-09 5.311992334043356e-10 1.4567232465874038e-09 9.708949359390384e-11 2.623357814393948e-10 1.747655991589111e-11 3.533193675169857e-07 2.3543032658366074e-08 8.81495493344734e-09 5.869352555896638e-10 1.9439520319093993e-10 1.2932580059931987e-11 1.7354001500624058e-10 1.1541423877380015e-11 2.6922876382532363e-08 1.7920761581539185e-09 1.8282271767663275e-08 1.2161036008779727e-09 1.6210460584619902e-08 1.0781784536206589e-09 1.3532879956198711e-08 9.005369520340763e-10 1.2652890276991968e-10 8.420674720365069e-12</parameters>
    </source>
  </nuclide>
  <nuclide name="Al24_m1" half_life="0.13" decay_modes="3" decay_energy="2019101.3436" reactions="0">
    <decay type="ec/beta+" target="Mg24" branching_ratio="0.1747"/>
    <decay type="ec/beta+,alpha" target="Ne20" branching_ratio="0.0003"/>
    <decay type="IT" target="Al24" branching_ratio="0.825"/>
  </nuclide>
  <nuclide name="Al25" half_life="7.183" decay_modes="1" decay_energy="2486168.0" reactions="0">
    <decay type="ec/beta+" target="Mg25" branching_ratio="1.0"/>
    <source type="discrete" particle="photon">
      <parameters>1254.0 389710.0 510998.9 585028.0 974742.0 1611708.0 1.4729942101865602e-06 2.122962268177474e-05 0.1927263746365478 2.122962268177474e-05 2.4124571229289478e-05 0.0007604064851472043</parameters>
    </source>
    <source type="discrete" particle="positron">
      <parameters>2664833.0 3301851.0 4276600.0 0.0007604064851472043 4.5354193911064214e-05 0.0956394501813952</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>1180.0 388405.0 389710.0 583723.0 585028.0 973437.0 974742.0 1610403.0 1611708.0 7.15065222006319e-05 1.9318956640415015e-09 1.2901241703714508e-10 1.8788216073370648e-09 1.2525477382247095e-10 3.4570510571571823e-10 2.301725340986509e-11 4.250672251972872e-09 2.830917110557957e-10</parameters>
    </source>
  </nuclide>
  <nuclide name="Al26" half_life="22626800000000.0" decay_modes="1" decay_energy="3119363.6" reactions="0">
    <decay type="ec/beta+" target="Mg26" branching_ratio="1.0"/>
    <source type="discrete" particle="photon">
      <parameters>34.49461 1237.95 1238.26 510998.9 1129670.0 1808650.0 2938000.0 4.3133330919754344e-19 4.74046771852375e-17 9.415688288538889e-17 5.0080303479917556e-14 7.658475574981276e-16 3.056038093440528e-14 7.352136551982024e-17</parameters>
    </source>
    <source type="discrete" particle="positron">
      <parameters>1065980.0 2195670.0 8.393689230179478e-16 2.978840659644717e-14</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>26.22608 42.65604 1167.054 1217.512 1280.72 1128365.0 1129670.0 1807345.0 1808650.0 2936695.0 2938000.0 2.181195724237313e-15 9.922097327458951e-15 4.812681016415363e-15 1.5520081492723124e-16 2.4571590887330274e-18 7.941839171255583e-21 5.289709079639566e-22 1.6166441514300395e-19 1.0775285203804396e-20 1.6836392704038836e-22 1.123112379680774e-23</parameters>
    </source>
  </nuclide>
  <nuclide name="Al26_m1" half_life="6.3452" decay_modes="1" decay_energy="2459531.0" reactions="4">
    <decay type="ec/beta+" target="Mg26" branching_ratio="1.0"/>
    <source type="discrete" particle="positron">
      <parameters>4232695.0 0.10923961113281619</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>1180.0 7.998040395667482e-05</parameters>
    </source>
    <source type="discrete" particle="photon">
      <parameters>1254.0 510998.9 1.6475518151051339e-06 0.21829919538648548</parameters>
    </source>
    <reaction type="(n,2n)" Q="-11365000.0" target="Al25"/>
    <reaction type="(n,gamma)" Q="13286740.0" target="Al27"/>
    <reaction type="(n,p)" Q="5014705.0" target="Mg26"/>
    <reaction type="(n,a)" Q="3193805.0" target="Na23"/>
  </nuclide>
  <nuclide name="Al27" reactions="4">
    <reaction type="(n,2n)" Q="-13058200.0" target="Al26"/>
    <reaction type="(n,gamma)" Q="7725200.0" target="Al28"/>
    <reaction type="(n,p)" Q="-1828549.0" target="Mg27"/>
    <reaction type="(n,a)" Q="-3130330.0" target="Na24"/>
  </nuclide>
  <nuclide name="Al28" half_life="134.484" decay_modes="1" decay_energy="3020144.0" reactions="0">
    <decay type="beta-" target="Si28" branching_ratio="1.0"/>
    <source type="discrete" particle="photon">
      <parameters>1739.0 1740.0 1836.0 1778850.0 7.24244070890322e-10 1.4573202482662296e-09 3.7095419347198437e-11 0.005153000167117153</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>1610.0 1777011.0 1778850.0 2863390.0 4.193548688985695e-08 4.416121277226618e-08 3.3613021193087672e-09 0.0051536083537215525</parameters>
    </source>
  </nuclide>
  <nuclide name="Al29" half_life="393.6" decay_modes="1" decay_energy="2356072.1" reactions="0">
    <decay type="beta-" target="Si29" branching_ratio="1.0"/>
    <source type="discrete" particle="photon">
      <parameters>397860.0 754740.0 1038860.0 1152593.0 1273368.0 1793570.0 2028070.0 2425906.0 4.578716131747605e-07 4.050402731930574e-06 8.805223330283858e-08 1.549719306129959e-05 0.001595506467447435 3.874298265324897e-07 6.515865264410054e-05 0.00010037954596523597</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>612669.9 1254100.0 1651500.0 2406400.0 5.811447397987345e-07 0.0001109458139615766 6.691969731015732e-05 0.0015831791547850375</parameters>
    </source>
  </nuclide>
  <nuclide name="Al30" half_life="3.62" decay_modes="1" decay_energy="5656012.0" reactions="0">
    <decay type="beta-" target="Si30" branching_ratio="1.0"/>
    <source type="discrete" particle="photon">
      <parameters>1739.0 1740.0 1836.0 1263130.0 1332480.0 1534120.0 2235230.0 2595390.0 3498330.0 2.0628060093463968e-08 4.1507682829419554e-08 1.0565592689223602e-09 0.0794553464213135 0.001820851688821768 0.0001591653574144902 0.12733228593159215 0.01134530667650486 0.06417547210952244</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>1610.0 1261291.0 1263130.0 1532281.0 1534120.0 2609400.0 2946980.0 3329440.0 3496491.0 3498330.0 3730160.0 3750690.0 5062500.0 6325674.0 1.1944151374626747e-06 1.0797980812748018e-06 8.214888381385876e-08 1.5279874311791056e-09 1.162224961147803e-10 0.0003063633947226277 0.000574431365104927 0.0049784051642427 1.764825483011867e-07 1.3407539594826004e-08 0.012637490032308395 0.010914195936993613 0.1288641029052053 0.03274258781098084</parameters>
    </source>
  </nuclide>
  <nuclide name="Al31" half_life="0.644" decay_modes="1" decay_energy="4100267.3" reactions="0">
    <decay type="beta-" target="Si31" branching_ratio="1.0"/>
    <source type="discrete" particle="photon">
      <parameters>621800.0 752430.0 1564400.0 1694980.0 2316800.0 0.025831571946333367 0.05596840588372229 0.04520525090608339 0.11193681176744458 0.19373678959750024</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>5678200.0 6300020.0 7242570.0 7995000.0 0.27984202941861147 0.08610523982111122 0.010763154977638903 0.6996050735465287</parameters>
    </source>
  </nuclide>
  <nuclide name="Al32" half_life="0.033" decay_modes="1" decay_energy="6422175.6" reactions="0">
    <decay type="beta-" target="Si32" branching_ratio="1.0"/>
    <source type="discrete" particle="photon">
      <parameters>1941400.0 2289400.0 3042300.0 3844000.0 4230000.0 2.5205352020361644 0.2772588722239781 0.9073926727330194 0.35287492828506306 0.35287492828506306</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>7234300.0 8036100.0 8789200.0 11078600.0 13020000.0 0.3570758202884567 0.9031917807296256 0.6301338005090411 0.9872096207974979 17.853791014422832</parameters>
    </source>
  </nuclide>
  <nuclide name="Al33" half_life="0.0417" decay_modes="2" decay_energy="7843900.5" reactions="0">
    <decay type="beta-" target="Si33" branching_ratio="0.915"/>
    <decay type="beta-,n" target="Si32" branching_ratio="0.085"/>
  </nuclide>
  <nuclide name="Al34" half_life="0.042" decay_modes="2" decay_energy="10150843.3" reactions="0">
    <decay type="beta-" target="Si34" branching_ratio="0.73"/>
    <decay type="beta-,n" target="Si33" branching_ratio="0.27"/>
  </nuclide>
  <nuclide name="Al35" half_life="0.0386" decay_modes="2" decay_energy="9176700.0" reactions="0">
    <decay type="beta-" target="Si35" branching_ratio="0.59"/>
    <decay type="beta-,n" target="Si34" branching_ratio="0.41"/>
  </nuclide>
  <nuclide name="Al36" half_life="0.09" decay_modes="2" decay_energy="11255793.4" reactions="0">
    <decay type="beta-" target="Si36" branching_ratio="0.69"/>
    <decay type="beta-,n" target="Si35" branching_ratio="0.31"/>
  </nuclide>
  <nuclide name="Al37" half_life="0.0107" decay_modes="1" decay_energy="10935902.0" reactions="0">
    <decay type="beta-" target="Si37" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Al38" half_life="0.0076" decay_modes="2" decay_energy="12253111.0" reactions="0">
    <decay type="beta-" target="Si38" branching_ratio="0.5"/>
    <decay type="beta-,n" target="Si37" branching_ratio="0.5"/>
  </nuclide>
  <nuclide name="Al39" half_life="7.6e-06" decay_modes="2" decay_energy="12635959.0" reactions="0">
    <decay type="beta-" target="Si39" branching_ratio="0.5"/>
    <decay type="beta-,n" target="Si38" branching_ratio="0.5"/>
  </nuclide>
  <nuclide name="Al40" half_life="2.6e-07" decay_modes="2" decay_energy="14750334.0" reactions="0">
    <decay type="beta-" target="Si40" branching_ratio="0.5"/>
    <decay type="beta-,n" target="Si39" branching_ratio="0.5"/>
  </nuclide>
  <nuclide name="Al41" half_life="2.6e-07" decay_modes="1" decay_energy="14512000.0" reactions="0">
    <decay type="beta-" target="Si41" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Al42" half_life="1.7e-07" decay_modes="2" decay_energy="16563166.0" reactions="0">
    <decay type="beta-" target="Si42" branching_ratio="0.5"/>
    <decay type="beta-,n" target="Si41" branching_ratio="0.5"/>
  </nuclide>
</nuclides>
  "#;

        let nuclides: SerdeNuclideVec = serde_xml_rs::from_str(xml).unwrap();
        use crate::decay_xml_info_serde::aluminium::get_aluminium_xml_serde_data;
        assert_eq!(nuclides,get_aluminium_xml_serde_data());

    }
}


use crate::decay_xml_info_serde::SerdeNuclideVec;
pub fn get_aluminium_xml_serde_data() -> SerdeNuclideVec {
    
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
