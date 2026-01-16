#[cfg(test)]
mod parsing_tests {
#[test]
fn serde_nuclide_test() {
    use crate::decay_xml_info_serde::SerdeNuclideVec;
    let xml = r#"
    <nuclides>
  <nuclide name="Fe45" half_life="0.00203" decay_modes="1" decay_energy="14813250.0" reactions="0">
    <decay type="ec/beta+,p" target="Cr44" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Fe46" half_life="0.013" decay_modes="2" decay_energy="9142927.0" reactions="0">
    <decay type="ec/beta+" target="Mn46" branching_ratio="0.213"/>
    <decay type="ec/beta+,p" target="Cr45" branching_ratio="0.787"/>
  </nuclide>
  <nuclide name="Fe47" half_life="0.0219" decay_modes="2" decay_energy="11585477.0" reactions="0">
    <decay type="ec/beta+" target="Mn47" branching_ratio="0.116"/>
    <decay type="ec/beta+,p" target="Cr46" branching_ratio="0.884"/>
  </nuclide>
  <nuclide name="Fe48" half_life="0.044" decay_modes="2" decay_energy="7420031.0" reactions="0">
    <decay type="ec/beta+" target="Mn48" branching_ratio="0.964"/>
    <decay type="ec/beta+,p" target="Cr47" branching_ratio="0.036"/>
  </nuclide>
  <nuclide name="Fe49" half_life="0.0647" decay_modes="2" decay_energy="8347683.0" reactions="0">
    <decay type="ec/beta+" target="Mn49" branching_ratio="0.433"/>
    <decay type="ec/beta+,p" target="Cr48" branching_ratio="0.567"/>
  </nuclide>
  <nuclide name="Fe50" half_life="0.155" decay_modes="1" decay_energy="5425954.0" reactions="0">
    <decay type="ec/beta+" target="Mn50" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Fe51" half_life="0.305" decay_modes="1" decay_energy="4331509.0" reactions="0">
    <decay type="ec/beta+" target="Mn51" branching_ratio="1.0"/>
    <source type="discrete" particle="photon">
      <parameters>557.1359 640.4894 655.72 678.3801 5847.93 5858.68 6448.81 6450.12 6499.05 6499.18 237000.0 510998.9 1825000.0 2140000.0 2914000.0 3426000.0 3555000.0 1.4166008012063075e-05 2.3793560999464443e-06 9.290928899929559e-08 2.754001014732241e-06 0.00022940830884416233 0.00044982706690988885 2.7337224826268757e-05 5.380181144141814e-05 9.636975243829565e-09 1.4102272560657163e-08 0.11363068533769596 4.5402276633529794 0.011135807163094204 0.005454272896209406 0.002272613706753919 0.004545227413507838 0.0036361819308062707</parameters>
    </source>
    <source type="discrete" particle="positron">
      <parameters>4464000.0 4593000.0 5105000.0 5879000.0 6194000.0 7782000.0 8019000.0 0.0036361819308062707 0.004545227413507838 0.002272613706753919 0.005454272896209406 0.011135807163094204 0.11363068533769596 2.131711656935176</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>36.42889 64.8982 559.8315 609.1767 5151.121 5762.267 6377.456 230461.0 236231.0 237000.0 1818461.0 1824231.0 1825000.0 2133461.0 2139231.0 2140000.0 2907461.0 2913231.0 2914000.0 0.008136252509960909 0.0009701260667752514 0.004178170755888892 6.658596805215803e-05 0.0013841760578838253 0.0003347080468556398 2.040442581426456e-05 0.00035907296566711923 3.431646697198418e-05 5.102017771662549e-06 4.521137708216247e-07 4.209335107649609e-08 2.0215821830002424e-06 1.6799160520324972e-07 1.570830594108309e-08 1.7094231229737003e-06 4.22706149456229e-08 3.933894326391034e-09 1.4618246776638572e-06</parameters>
    </source>
  </nuclide>
  <nuclide name="Fe52" half_life="29790.0" decay_modes="1" decay_energy="927924.2" reactions="0">
    <decay type="ec/beta+" target="Mn52_m1" branching_ratio="1.0"/>
    <source type="discrete" particle="photon">
      <parameters>558.3704 640.4894 655.72 675.9247 5847.93 5858.68 6448.81 6450.12 6499.05 6499.18 168688.0 510998.9 1039928.0 5.135701736444446e-08 9.012023220408282e-09 3.419684311669675e-10 9.447761272273478e-09 8.577271722435526e-07 1.6819834095858872e-06 1.0221880328910919e-07 2.0117457957897818e-07 3.6034369544704824e-11 5.27309332663479e-11 2.307586782862326e-05 2.5822582778967013e-05 2.2136733672807196e-08</parameters>
    </source>
    <source type="discrete" particle="positron">
      <parameters>956312.0 1827562.0 2.2104391457937162e-08 2.324451270155708e-05</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>36.54812 65.028 560.4106 607.0813 5152.046 5762.736 6377.503 162149.0 167919.0 168688.0 2.8011406156692914e-05 2.846682642406787e-06 1.4716235944482582e-05 2.3695651697323333e-07 5.13034549337748e-06 1.2444765410484478e-06 7.61753162022457e-08 1.6268487749890617e-07 1.566851406949295e-08 2.2284365413187688e-09</parameters>
    </source>
  </nuclide>
  <nuclide name="Fe52_m1" half_life="45.9" decay_modes="1" decay_energy="6867769.0" reactions="0">
    <decay type="ec/beta+" target="Mn52" branching_ratio="1.0"/>
    <source type="discrete" particle="photon">
      <parameters>557.1275 640.4894 655.72 678.4035 5847.93 5858.68 6448.81 6450.12 6499.05 6499.18 510998.9 621700.0 869900.0 929500.0 1416100.0 2037600.0 2285900.0 3.3492509334758223e-07 5.6284653452408313e-08 2.1943303093265027e-09 6.573350292683877e-08 5.390243193316362e-06 1.0569263752009413e-05 6.423234849043916e-07 1.2641428003355658e-06 2.2643306239401754e-10 3.3135108151265583e-10 0.030075641063250263 0.007701635339554948 0.014044158560364905 0.01509808507309486 0.00724859796663995 0.007550622881916615 0.0007550622881916615</parameters>
    </source>
    <source type="discrete" particle="positron">
      <parameters>5494800.0 0.01510124576383323</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>36.41895 64.89911 559.9026 609.5078 5151.134 5762.273 6377.456 922961.0 928731.0 929500.0 0.00019290905186059593 2.3374931394167944e-05 9.882595005882152e-05 1.5795993676444142e-06 3.251897732410119e-05 7.863810638061904e-06 4.79422873563397e-07 2.854135449364481e-06 2.6850014968095484e-07 3.9888430560589093e-08</parameters>
    </source>
  </nuclide>
  <nuclide name="Fe53" half_life="510.6" decay_modes="1" decay_energy="2273447.0" reactions="0">
    <decay type="ec/beta+" target="Mn53" branching_ratio="1.0"/>
    <source type="discrete" particle="photon">
      <parameters>558.2305 640.4894 655.72 675.7012 5847.93 5858.68 6448.81 6450.12 6499.05 6499.18 377900.0 510998.9 1288000.0 1397600.0 1619900.0 2273500.0 2307700.0 2685600.0 2748800.0 2946600.0 3248800.0 1.8979743608954034e-07 3.193233763283703e-08 1.2555298241565345e-09 3.4294753185068614e-08 3.2202727478947355e-06 6.314391622915528e-06 3.837431666451109e-07 7.552363890521943e-07 1.352778672242404e-10 1.979589522748996e-10 0.000570156317734385 0.002629664108534193 5.701563177343851e-07 1.14031263546877e-07 6.84187581281262e-06 5.131406859609465e-06 1.7104689532031551e-07 1.0832970036953316e-06 1.8815158485234706e-06 6.841875812812621e-07 5.131406859609466e-07</parameters>
    </source>
    <source type="discrete" particle="positron">
      <parameters>493700.2 615800.1 795900.1 1056900.0 1469000.0 2122670.0 3364700.0 3742600.0 5.430060168898905e-07 1.9005210591146166e-06 6.787575211123631e-07 1.3575150422247262e-06 5.25358321340969e-06 1.3846653430692209e-05 0.000570156317734385 0.0007602084236458468</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>36.46876 64.90659 559.4135 606.6364 5151.179 5762.296 6377.458 0.0001071447254891752 1.1036598650802066e-05 5.563683089535169e-05 8.699700308819185e-07 1.9419442731130618e-05 4.696745475754573e-06 2.8639521996115893e-07</parameters>
    </source>
  </nuclide>
  <nuclide name="Fe53_m1" half_life="152.4" decay_modes="1" decay_energy="3054686.0" reactions="0">
    <decay type="IT" target="Fe53" branching_ratio="1.0"/>
    <source type="discrete" particle="photon">
      <parameters>701100.0 1011500.0 1328100.0 1712600.0 2339700.0 3040600.0 0.004548209846193867 0.003911460467726726 0.003956942566188664 5.912672800052027e-05 0.0005912672800052028 2.72892590771632e-06</parameters>
    </source>
  </nuclide>
  <nuclide name="Fe54" reactions="4">
    <reaction type="(n,2n)" Q="-13378000.0" target="Fe53"/>
    <reaction type="(n,gamma)" Q="9298429.0" target="Fe55"/>
    <reaction type="(n,p)" Q="85400.0" target="Mn54"/>
    <reaction type="(n,a)" Q="843499.9" target="Cr51"/>
  </nuclide>
  <nuclide name="Fe55" half_life="86594050.0" decay_modes="1" decay_energy="5842.102" reactions="5">
    <decay type="ec/beta+" target="Mn55" branching_ratio="1.0"/>
    <source type="discrete" particle="photon">
      <parameters>557.6039 640.4896 655.72 678.8113 5847.93 5858.68 6448.81 6450.12 6499.05 6499.18 126000.0 4.281895544855688e-11 8.113092795419058e-12 2.881383486075155e-13 8.900165070824795e-12 6.680021392530635e-10 1.3098276096378766e-09 7.960187308411459e-11 1.5666266690172673e-10 2.8061406316612144e-14 4.106367573258232e-14 1.0245835494664239e-17</parameters>
    </source>
    <source type="discrete" particle="positron">
      <parameters>105210.0 231210.0 1.040592667426837e-17 8.004558980206438e-09</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>36.67648 65.20168 562.7812 610.8647 5155.619 5764.491 6377.571 2.257651440763611e-08 2.654244519541021e-09 1.1943434358627441e-08 2.0711244046714524e-10 3.863926291321637e-09 9.499586470057554e-10 5.918783030777615e-11</parameters>
    </source>
    <reaction type="(n,2n)" Q="-9298090.0" target="Fe54"/>
    <reaction type="(n,3n)" Q="-22676500.0" target="Fe53"/>
    <reaction type="(n,gamma)" Q="11197100.0" target="Fe56"/>
    <reaction type="(n,p)" Q="1013440.0" target="Mn55"/>
    <reaction type="(n,a)" Q="3583860.0" target="Cr52"/>
  </nuclide>
  <nuclide name="Fe56" reactions="4">
    <reaction type="(n,2n)" Q="-11197000.0" target="Fe55"/>
    <reaction type="(n,gamma)" Q="7646431.0" target="Fe57"/>
    <reaction type="(n,p)" Q="-2913195.0" target="Mn56"/>
    <reaction type="(n,a)" Q="326500.0" target="Cr53"/>
  </nuclide>
  <nuclide name="Fe57" reactions="4">
    <reaction type="(n,2n)" Q="-7646000.0" target="Fe56"/>
    <reaction type="(n,gamma)" Q="10044430.0" target="Fe58"/>
    <reaction type="(n,p)" Q="-1908600.0" target="Mn57"/>
    <reaction type="(n,a)" Q="2398500.0" target="Cr54"/>
  </nuclide>
  <nuclide name="Fe58" reactions="4">
    <reaction type="(n,2n)" Q="-10044000.0" target="Fe57"/>
    <reaction type="(n,gamma)" Q="6580430.0" target="Fe59"/>
    <reaction type="(n,p)" Q="-5464601.0" target="Mn58"/>
    <reaction type="(n,a)" Q="-1399500.0" target="Cr55"/>
  </nuclide>
  <nuclide name="Fe59" half_life="3844368.0" decay_modes="1" decay_energy="1306009.3" reactions="0">
    <decay type="beta-" target="Co59" branching_ratio="1.0"/>
    <source type="discrete" particle="photon">
      <parameters>780.0 6915.0 6930.0 7649.0 142651.0 189000.0 192343.0 334800.0 382000.0 1099245.0 1291590.0 1481700.0 1.0526870077721752e-12 1.0971242349369013e-11 2.1648606887732874e-11 3.9280951004469605e-12 1.8390802445841403e-09 1.622717862868359e-12 5.553301130705051e-09 4.868153588605077e-10 3.245435725736718e-11 1.018706213911803e-07 7.789045741768123e-08 1.0637817101025909e-10</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>750.0 6070.0 83569.95 130946.9 134942.1 141725.4 184634.1 191417.4 273599.0 327091.1 333874.4 465943.0 1091536.0 1283881.0 1565200.0 1.4515384373262843e-10 5.994999523918275e-11 1.4063554811525778e-10 2.3619560003972782e-09 2.703447959538686e-11 2.7218387619845277e-12 4.520385317374064e-11 4.498173915871091e-12 8.16767990977074e-08 8.519268780058884e-13 8.275861100628631e-14 9.574035390923319e-08 1.6299297619569002e-11 8.567950315944936e-12 3.245435725736718e-10</parameters>
    </source>
  </nuclide>
  <nuclide name="Fe60" half_life="47336400000000.0" decay_modes="1" decay_energy="50180.0" reactions="0">
    <decay type="beta-" target="Co60_m1" branching_ratio="1.0"/>
    <source type="discrete" particle="electron">
      <parameters>178400.0 1.4643005817086752e-14</parameters>
    </source>
  </nuclide>
  <nuclide name="Fe61" half_life="358.8" decay_modes="1" decay_energy="2515414.0" reactions="0">
    <decay type="beta-" target="Co61" branching_ratio="1.0"/>
    <source type="discrete" particle="photon">
      <parameters>120340.0 177610.0 297900.0 333000.0 349700.0 440500.0 542600.0 561400.0 603300.0 618400.0 657300.0 686000.0 696900.0 748100.0 769400.0 806300.0 925600.0 945400.0 978000.0 984100.0 989200.0 1027420.0 1097800.0 1205070.0 1275000.0 1285700.0 1381400.0 1403900.0 1538800.0 1618900.0 1645950.0 1659300.0 1837200.0 1879400.0 1889000.0 1899300.0 1972700.0 1999800.0 2011600.0 2177100.0 2230800.0 2484400.0 2754400.0 2920000.0 3191000.0 3204200.0 3239100.0 3364900.0 0.00010275887633317896 3.874515009283797e-05 0.00042956579450755135 4.2956579450755134e-06 3.116457724858706e-06 4.21142935791717e-06 1.263428807375151e-06 1.0949716330584643e-06 1.431885981691838e-06 1.802491765188549e-05 4.2956579450755134e-06 7.749030018567594e-06 2.1899432661169287e-06 1.558228862429353e-05 3.116457724858706e-06 3.7060578349671097e-06 6.569829798350785e-06 2.105714678958585e-06 1.431885981691838e-06 1.1792002202168078e-05 1.1792002202168078e-05 0.0008254401541517653 1.3476573945334946e-05 0.000842285871583434 1.1792002202168078e-05 7.159429908459189e-06 7.749030018567594e-06 2.2741718532752718e-06 5.306400990975635e-06 7.075201321300846e-06 0.00013476573945334945 1.4992688514185127e-05 2.6953147890669893e-06 5.053715229500604e-06 3.4533720734920796e-06 1.431885981691838e-06 1.1792002202168076e-06 2.526857614750302e-06 8.507087302992683e-05 4.042972183600483e-06 2.105714678958585e-06 2.3584004404336153e-06 1.4824231339868439e-05 1.3476573945334947e-06 1.6003431560085245e-06 8.422858715834341e-07 1.0949716330584643e-06 8.422858715834341e-07</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>613000.0 738800.1 773300.1 786899.9 873600.1 977699.9 1057900.0 1113610.0 1223500.0 1493500.0 1546600.0 1675000.0 1747100.0 1966500.0 2024800.0 2089000.0 2332110.0 2359120.0 2652590.0 2692270.0 2772900.0 2950510.0 3978000.0 7.969724371709087e-07 1.032441566335041e-06 1.1773456458206606e-05 1.5214928345990077e-06 1.267910695499173e-06 2.535821390998346e-05 1.267910695499173e-06 2.1735611922842964e-05 1.3947017650490904e-05 2.227900222091404e-06 2.173561192284297e-06 2.8980815897123955e-05 1.0867805961421485e-06 9.056504967851236e-05 1.7931879836345447e-05 4.709382583282643e-06 0.00014490407948561978 2.4452563413198337e-06 0.0004890512682639667 3.622601987140494e-07 0.0006882943775566939 0.00028980815897123956 9.056504967851236e-05</parameters>
    </source>
  </nuclide>
  <nuclide name="Fe62" half_life="68.0" decay_modes="1" decay_energy="1332100.4" reactions="0">
    <decay type="beta-" target="Co62" branching_ratio="1.0"/>
    <source type="discrete" particle="photon">
      <parameters>684.0995 778.5525 799.73 809.9629 6873.16 6888.41 7606.53 7608.44 7666.77 7666.98 506100.0 4.296873396997821e-08 1.4614233608198762e-08 3.259515442576341e-10 1.1413514375213417e-08 7.629883846729386e-07 1.4918820827350404e-06 9.262607320549674e-08 1.8188385884710775e-07 6.110022062583773e-11 8.900391029339005e-11 0.010185723406939894</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>43.92516 74.92521 698.1456 744.279 6030.791 6775.11 7523.545 498391.1 505174.4 506100.0 2024900.0 1.6776404304546605e-05 1.9389151248281085e-06 1.0437451018235219e-05 1.2130065466458152e-07 3.427985884145525e-06 8.483264230084832e-07 5.427544251934005e-08 6.86011841936534e-06 6.656251601553592e-07 1.0113832831640848e-07 0.01019334089058743</parameters>
    </source>
  </nuclide>
  <nuclide name="Fe63" half_life="6.1" decay_modes="1" decay_energy="3019611.7" reactions="0">
    <decay type="beta-" target="Co63" branching_ratio="1.0"/>
    <source type="discrete" particle="photon">
      <parameters>149700.0 432100.0 461300.0 499600.0 582000.0 893800.0 994800.0 1299000.0 1427200.0 1494600.0 1799300.0 2427000.0 2796000.0 3.499825108401035e-05 0.000699965021680207 0.00011135807163094202 0.00022271614326188405 0.00023862443920916147 0.00020680784731460662 0.015908295947277433 0.001399930043360414 0.005249737662601553 0.0012726636757821946 0.00039770739868193585 0.00023862443920916147 0.00012726636757821945</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>2868000.0 3496100.0 4401400.0 4713100.0 4795270.0 4862860.0 5295080.0 6290000.0 0.00022726137067539192 0.0013635682240523514 0.00034089205601308786 0.00022726137067539192 9.090454827015677e-05 0.0056815342668847975 0.014771989093900473 0.09090454827015676</parameters>
    </source>
  </nuclide>
  <nuclide name="Fe64" half_life="2.0" decay_modes="1" decay_energy="2284256.0" reactions="0">
    <decay type="beta-" target="Co64" branching_ratio="1.0"/>
    <source type="discrete" particle="photon">
      <parameters>310800.0 1250000.0 0.024260151319598088 0.006931471805599453</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>3459000.0 4709200.0 5020000.0 0.006931471805599453 0.01732867951399863 0.32231343896037457</parameters>
    </source>
  </nuclide>
  <nuclide name="Fe65" half_life="0.81" decay_modes="1" decay_energy="4742364.0" reactions="0">
    <decay type="beta-" target="Co65" branching_ratio="1.0"/>
    <source type="discrete" particle="photon">
      <parameters>127600.0 212500.0 340070.0 736100.0 774000.0 864000.0 882500.0 960500.0 1076200.0 1088700.0 1113500.0 1222700.0 1996600.0 0.017427945036152893 0.05689711467685211 0.24091571079387827 0.11276905611628345 0.030755197122622757 0.009226559136786827 0.5125866187103792 0.046132795683934136 0.042544689352961476 0.019990878129704792 0.07688799280655689 0.11789492230338725 0.2255381122325669</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>6106810.0 6293720.0 6331200.0 7067380.0 7195010.0 7407460.0 0.06845898079604398 0.3337375313807144 0.16259007939060444 0.18826219718912093 0.012836058899258245 0.09413109859456047</parameters>
    </source>
  </nuclide>
  <nuclide name="Fe65_m1" half_life="1.12" decay_modes="1" decay_energy="5165200.0" reactions="0">
    <decay type="beta-" target="Co65" branching_ratio="1.0"/>
    <source type="discrete" particle="photon">
      <parameters>413000.0 836600.0 999700.0 1412500.0 1441100.0 1479500.0 1625500.0 1641900.0 2443300.0 2557500.0 2896000.0 0.00933273168111069 0.05025317059059602 0.06461121933076633 0.07896926807093661 0.028716097480340586 0.16870707269700094 0.02512658529529801 0.08255878025597918 0.11127487773631976 0.11127487773631976 0.05025317059059602</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>5469900.0 5474100.0 5808400.0 5887100.0 5922700.0 6723830.0 6740500.0 6886590.0 6924900.0 8366000.0 0.049510512897138946 0.08664339756999316 0.11139865401856262 0.09902102579427789 0.11139865401856262 0.04332169878499658 0.025993019270997945 0.030944070560711842 0.030944070560711842 0.049510512897138946</parameters>
    </source>
  </nuclide>
  <nuclide name="Fe66" half_life="0.44" decay_modes="1" decay_energy="4227130.0" reactions="0">
    <decay type="beta-" target="Co66" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Fe67" half_life="0.416" decay_modes="1" decay_energy="6419696.0" reactions="0">
    <decay type="beta-" target="Co67" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Fe68" half_life="0.187" decay_modes="1" decay_energy="5855992.0" reactions="0">
    <decay type="beta-" target="Co68" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Fe69" half_life="0.109" decay_modes="1" decay_energy="7737334.0" reactions="0">
    <decay type="beta-" target="Co69" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Fe70" half_life="0.094" decay_modes="1" decay_energy="6222666.0" reactions="0">
    <decay type="beta-" target="Co70" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Fe71" half_life="0.028" decay_modes="1" decay_energy="8582000.0" reactions="0">
    <decay type="beta-" target="Co71" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Fe72" half_life="1.5e-07" decay_modes="2" decay_energy="5658132.0" reactions="0">
    <decay type="beta-" target="Co72" branching_ratio="0.724"/>
    <decay type="beta-,n" target="Co71" branching_ratio="0.276"/>
  </nuclide>
</nuclides>
  "#;

        let nuclides: SerdeNuclideVec = serde_xml_rs::from_str(xml).unwrap();
        use crate::decay_xml_info_serde::iron::get_iron_xml_serde_data;
        assert_eq!(nuclides,get_iron_xml_serde_data());

    }
}


use crate::decay_xml_info_serde::SerdeNuclideVec;
pub fn get_iron_xml_serde_data() -> SerdeNuclideVec {
    
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
