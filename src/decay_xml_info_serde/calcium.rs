
// notes, code was done from vibe coding using AI, then 
// modified


#[cfg(test)]
mod parsing_tests {
    use crate::prelude::calcium::get_calcium_xml_serde_data;

#[test]
fn serde_nuclide_test() {
    use crate::decay_xml_info_serde::SerdeNuclideVec;
    let xml = r#"
    <nuclides>
                <nuclide name="Ca34" half_life="3.5e-08" decay_modes="1" decay_energy="500000.0" reactions="0">
    <decay type="p" target="K33" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Ca35" half_life="0.0257" decay_modes="3" decay_energy="11687840.0" reactions="0">
    <decay type="ec/beta+" target="K35" branching_ratio="0.001"/>
    <decay type="ec/beta+,p" target="Ar34" branching_ratio="0.957"/>
    <decay type="ec/beta+,p,p" target="Cl33" branching_ratio="0.042"/>
  </nuclide>
  <nuclide name="Ca36" half_life="0.102" decay_modes="2" decay_energy="7136247.0" reactions="0">
    <decay type="ec/beta+" target="K36" branching_ratio="0.457"/>
    <decay type="ec/beta+,p" target="Ar35" branching_ratio="0.543"/>
  </nuclide>
  <nuclide name="Ca37" half_life="0.1811" decay_modes="2" decay_energy="7414493.0" reactions="0">
    <decay type="ec/beta+" target="K37" branching_ratio="0.179"/>
    <decay type="ec/beta+,p" target="Ar36" branching_ratio="0.821"/>
  </nuclide>
  <nuclide name="Ca38" half_life="0.44" decay_modes="2" decay_energy="3797364.0" reactions="0">
    <decay type="ec/beta+" target="K38" branching_ratio="0.0004017"/>
    <decay type="ec/beta+" target="K38_m1" branching_ratio="0.9995983"/>
    <source type="discrete" particle="photon">
      <parameters>237.3929 297.43 341.0853 3281.65 3284.66 3559.55 3559.83 328000.0 510998.9 1240000.0 1567700.0 1643000.0 1698000.0 2534000.0 2883000.0 3210000.0 3342000.0 3519000.0 3716000.0 3726000.0 3848000.0 2.4114511644955435e-06 1.8332073071239205e-08 2.7998357077737923e-07 6.290672990516796e-05 0.00012420849326709438 6.007246683720336e-06 1.1887206339737846e-05 0.04749633521336898 3.146582584848905 0.0007599413634139037 0.31664223475579323 0.0012665689390231728 0.0002533137878046346 3.324743464935828e-05 0.0022164956432905525 0.004369662839629946 0.00037997068170695183 0.0001266568939023173 6.332844695115865e-05 0.0006016202460360071 0.001773196514632442</parameters>
    </source>
    <source type="discrete" particle="positron">
      <parameters>2567700.0 2764200.0 2885700.0 3401000.0 3749300.0 5044100.0 6283900.0 6611900.0 6.301338005090412e-05 0.0018904014015271235 0.0005986271104835892 0.007876672506363015 3.623269352926987e-05 0.3150669002545206 0.044266899485760146 1.2051308934735414</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>40.57267 249.292 274.6179 2953.251 3238.985 3527.751 0.0006127569157593036 0.0026560754071911583 3.375381017144735e-05 0.001070941380119939 0.00021582618281165093 1.0990300668779799e-05</parameters>
    </source>
  </nuclide>
  <nuclide name="Ca39" half_life="0.8596" decay_modes="1" decay_energy="3581586.0" reactions="0">
    <decay type="ec/beta+" target="K39" branching_ratio="1.0"/>
    <source type="discrete" particle="photon">
      <parameters>234.3082 297.43 341.9012 3281.65 3284.66 3559.55 3559.83 510998.9 2522250.0 9.575567038748695e-07 7.1497357569019235e-09 1.247254487694145e-07 2.3312049188422222e-05 4.6029319070361396e-05 2.2261724781676806e-06 4.405174500578736e-06 1.6115373594765112 2.015900362261358e-05</parameters>
    </source>
    <source type="discrete" particle="positron">
      <parameters>2057600.0 2406600.0 2437600.0 2450600.0 2588600.0 2593600.0 2649600.0 2934600.0 3513600.0 3718600.0 4010260.0 6532600.0 2.015900362261358e-06 1.4514482608281775e-06 1.4514482608281775e-06 1.4111302535829504e-06 1.1692222101115876e-05 1.2498582246020418e-06 1.6127202898090861e-06 2.862578514411128e-06 1.3304942390924962e-06 1.4514482608281775e-06 2.015900362261358e-05 0.8063399859009205</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>40.57265 249.4437 275.2444 2953.251 3238.986 3527.751 0.0002777355117056312 0.0010378903333109846 1.3284259253208159e-05 0.0003968670788778139 7.998075817310343e-05 4.072801718810677e-06</parameters>
    </source>
  </nuclide>
  <nuclide name="Ca40" reactions="4">
    <reaction type="(n,2n)" Q="-15641200.0" target="Ca39"/>
    <reaction type="(n,gamma)" Q="8362701.0" target="Ca41"/>
    <reaction type="(n,p)" Q="-528741.0" target="K40"/>
    <reaction type="(n,a)" Q="1748330.0" target="Ar37"/>
  </nuclide>
  <nuclide name="Ca41" half_life="3218880000000.0" decay_modes="1" decay_energy="3111.7509" reactions="4">
    <decay type="ec/beta+" target="K41" branching_ratio="1.0"/>
    <source type="discrete" particle="positron">
      <parameters>421390.0 2.1533799972659598e-13</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>40.60326 249.2584 274.0613 2953.25 3239.025 3527.805 6.000768050501121e-14 3.2084069931264444e-13 4.72806448423701e-15 1.371671188234457e-13 2.75990100729589e-14 1.4026167661611669e-15</parameters>
    </source>
    <source type="discrete" particle="photon">
      <parameters>242.6217 297.43 338.9638 3281.65 3284.66 3559.55 3559.83 3.194496158344106e-16 2.4639469206116483e-18 3.601637867807178e-17 8.05698754229044e-15 1.5908411276661876e-14 7.693979355871335e-16 1.5224932772289656e-15</parameters>
    </source>
    <reaction type="(n,2n)" Q="-8362001.0" target="Ca40"/>
    <reaction type="(n,gamma)" Q="11481430.0" target="Ca42"/>
    <reaction type="(n,p)" Q="1204400.0" target="K41"/>
    <reaction type="(n,a)" Q="5224500.0" target="Ar38"/>
  </nuclide>
  <nuclide name="Ca42" reactions="4">
    <reaction type="(n,2n)" Q="-11480600.0" target="Ca41"/>
    <reaction type="(n,gamma)" Q="7933000.0" target="Ca43"/>
    <reaction type="(n,p)" Q="-2743090.0" target="K42"/>
    <reaction type="(n,a)" Q="341482.0" target="Ar39"/>
  </nuclide>
  <nuclide name="Ca43" reactions="5">
    <reaction type="(n,2n)" Q="-7933000.0" target="Ca42"/>
    <reaction type="(n,3n)" Q="-19413600.0" target="Ca41"/>
    <reaction type="(n,gamma)" Q="11132000.0" target="Ca44"/>
    <reaction type="(n,p)" Q="-1033050.0" target="K43"/>
    <reaction type="(n,a)" Q="2277860.0" target="Ar40"/>
  </nuclide>
  <nuclide name="Ca44" reactions="5">
    <reaction type="(n,2n)" Q="-11132000.0" target="Ca43"/>
    <reaction type="(n,3n)" Q="-19065000.0" target="Ca42"/>
    <reaction type="(n,gamma)" Q="7414770.0" target="Ca45"/>
    <reaction type="(n,p)" Q="-4876520.0" target="K44"/>
    <reaction type="(n,a)" Q="-2755410.0" target="Ar41"/>
  </nuclide>
  <nuclide name="Ca45" half_life="14049500.0" decay_modes="2" decay_energy="76860.28" reactions="6">
    <decay type="beta-" target="Sc45" branching_ratio="0.999981"/>
    <decay type="beta-" target="Sc45_m1" branching_ratio="1.9e-05"/>
    <source type="discrete" particle="electron">
      <parameters>243400.0 255800.0 9.373854180318845e-13 4.933513724783904e-08</parameters>
    </source>
    <reaction type="(n,2n)" Q="-7414820.0" target="Ca44"/>
    <reaction type="(n,3n)" Q="-18546000.0" target="Ca43"/>
    <reaction type="(n,4n)" Q="-26478900.0" target="Ca42"/>
    <reaction type="(n,gamma)" Q="10397600.0" target="Ca46"/>
    <reaction type="(n,p)" Q="-3414160.0" target="K45"/>
    <reaction type="(n,a)" Q="-743067.0" target="Ar42"/>
  </nuclide>
  <nuclide name="Ca46" reactions="5">
    <reaction type="(n,2n)" Q="-10393700.0" target="Ca45"/>
    <reaction type="(n,3n)" Q="-17808500.0" target="Ca44"/>
    <reaction type="(n,gamma)" Q="7276100.0" target="Ca47"/>
    <reaction type="(n,p)" Q="-6933630.0" target="K46"/>
    <reaction type="(n,a)" Q="-5510970.0" target="Ar43"/>
  </nuclide>
  <nuclide name="Ca47" half_life="391910.4" decay_modes="1" decay_energy="1346828.4" reactions="6">
    <decay type="beta-" target="Sc47" branching_ratio="1.0"/>
    <source type="discrete" particle="photon">
      <parameters>336.6881 401.8169 407.78 442.779 4052.12 4056.94 4426.2 4426.74 4458.72 4458.76 41060.0 489230.0 530600.0 731600.0 767100.0 807860.0 1146970.0 1297090.0 1878000.0 3.727950466882375e-13 1.3119820766246766e-14 2.4396806815479657e-15 4.925807535763675e-14 7.962353636851225e-12 1.5693367803342842e-11 8.787903802818563e-13 1.7375253028520534e-12 2.76952615064725e-17 4.081730898653019e-17 9.835389596943219e-11 1.0427882946156665e-07 1.516782973986424e-09 2.0144773873257195e-10 3.1876142187683443e-09 1.0427882946156665e-07 2.0144773873257195e-10 1.184986698426894e-06 4.502949454022197e-10</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>24.38053 41.68209 331.044 371.6687 3608.766 3991.67 4378.25 36567.2 40559.6 41060.0 113800.0 484737.2 488729.6 489230.0 526107.2 530099.6 530600.0 694880.0 727107.2 731099.6 731600.0 762607.2 766599.6 767100.0 803367.2 807359.6 807860.0 1225170.0 1292597.0 1296590.0 1297090.0 1873507.0 1877500.0 1878000.0 1992000.0 6.103648941262742e-10 5.079505616460233e-11 2.358268558092311e-10 6.198058777026962e-12 9.269648648924407e-11 2.104348900898651e-11 1.2040402233554802e-12 3.5702464236903885e-11 3.157160060618773e-12 4.2587236954764135e-13 6.543956394297772e-10 3.4724850210701694e-11 3.0657975861700595e-12 4.221207016604218e-13 2.5936988855167853e-13 2.275174460979636e-14 2.9788100118664634e-15 1.291104910226317e-06 2.7396892467629787e-14 2.3972280909176062e-15 3.3162326750156e-16 9.690347225055767e-13 8.574682248486846e-14 1.1806922712590574e-14 1.7383280871243162e-11 1.5297704282011828e-12 2.0037177081040032e-13 1.5387140710916382e-09 5.213941473078333e-11 4.5621987889435414e-12 2.4061036412888236e-11 1.031175424971083e-14 9.005898908044393e-16 9.017695735377767e-14 4.77531953097405e-07</parameters>
    </source>
    <reaction type="(n,2n)" Q="-7276380.0" target="Ca46"/>
    <reaction type="(n,3n)" Q="-17673900.0" target="Ca45"/>
    <reaction type="(n,4n)" Q="-25088800.0" target="Ca44"/>
    <reaction type="(n,gamma)" Q="9952630.0" target="Ca48"/>
    <reaction type="(n,p)" Q="-5849120.0" target="K47"/>
    <reaction type="(n,a)" Q="-4023790.0" target="Ar44"/>
  </nuclide>
  <nuclide name="Ca48" half_life="7.25824e+26" decay_modes="2" decay_energy="1649595.66" reactions="5">
    <decay type="beta-,beta-" target="Ti48" branching_ratio="0.75"/>
    <decay type="beta-" target="Sc48" branching_ratio="0.25"/>
    <reaction type="(n,2n)" Q="-9946370.0" target="Ca47"/>
    <reaction type="(n,3n)" Q="-17222500.0" target="Ca46"/>
    <reaction type="(n,gamma)" Q="5146630.0" target="Ca49"/>
    <reaction type="(n,p)" Q="-11307900.0" target="K48"/>
    <reaction type="(n,a)" Q="-8849000.0" target="Ar45"/>
  </nuclide>
  <nuclide name="Ca49" half_life="523.08" decay_modes="1" decay_energy="4042192.3" reactions="0">
    <decay type="beta-" target="Sc49" branching_ratio="1.0"/>
    <source type="discrete" particle="photon">
      <parameters>143200.0 712600.0 856100.0 976700.0 987300.0 1144500.0 1288400.0 1408900.0 2228900.0 2264700.0 2371700.0 2486300.0 3084400.0 4071900.0 4332000.0 4493000.0 4714400.0 4738200.0 4.107892214834883e-07 1.7889208032345458e-07 1.934684720535138e-06 9.938448906858586e-08 1.0203474211041482e-06 1.2986239904961887e-06 6.758145256663839e-07 7.937507860277724e-06 4.227153601717186e-06 5.565531387840809e-07 7.28819586502963e-06 4.107892214834883e-07 0.0012021547797736146 0.00010761352476346478 8.083271777578317e-08 4.637942823200674e-07 6.758145256663839e-08 3.723605523769684e-06</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>523649.9 547300.3 768660.1 929899.9 1190030.0 1745400.0 2177580.0 2890200.0 3033500.0 3.723605523769684e-06 4.770455475292122e-07 8.600071120734965e-06 8.083271777578317e-08 0.00010863387218456893 1.8816796596985592e-06 0.0011952641218648595 6.228094648298048e-06 2.385227737646061e-07</parameters>
    </source>
  </nuclide>
  <nuclide name="Ca50" half_life="13.9" decay_modes="2" decay_energy="2916307.0" reactions="0">
    <decay type="beta-" target="Sc50" branching_ratio="0.0035"/>
    <decay type="beta-" target="Sc50_m1" branching_ratio="0.9965"/>
    <source type="discrete" particle="photon">
      <parameters>336.9863 401.8169 407.78 442.6321 4052.12 4056.94 4426.2 4426.74 4458.72 4458.76 71552.0 328450.0 1519300.0 1590850.0 1.1656820695598603e-05 4.18222169799605e-07 7.64652542906731e-08 1.5355454051066472e-06 0.0002463432613034136 0.00048558076667201504 2.7193560161068503e-05 5.37668283955913e-05 8.570341020642805e-10 1.263093683113315e-09 0.02567137903253668 0.00017278812810361227 0.030657550729240913 0.018710485871791156</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>24.38613 41.69079 331.0748 371.6058 3608.885 3991.726 4378.25 67059.2 71051.6 71552.0 3118228.0 0.01885456075194035 0.0015477452941515385 0.007301427893215005 0.00019410160603857237 0.002863449845603426 0.0006505400716380727 3.725751367572926e-05 0.004364134435531235 0.00041074206452058686 5.3909895968327025e-05 0.04936803660103207</parameters>
    </source>
  </nuclide>
  <nuclide name="Ca51" half_life="10.0" decay_modes="1" decay_energy="4874199.0" reactions="0">
    <decay type="beta-" target="Sc51" branching_ratio="1.0"/>
    <source type="discrete" particle="photon">
      <parameters>352400.0 532200.0 547700.0 861600.0 1167500.0 1314800.0 1323700.0 1394000.0 1424000.0 1480100.0 1485300.0 1644400.0 1714800.0 1847100.0 1996500.0 2027500.0 2333400.0 2378700.0 2912000.0 3038900.0 3196500.0 3771700.0 0.00011956788864659056 0.002630493550224992 0.013630739305711324 0.02391357772931811 0.016261232855936315 0.0031087651048113544 0.009565431091727245 0.018413454851574945 0.0016739504410522678 0.015543825524056773 0.011239381532779511 0.0033479008821045357 0.005500122877743166 0.004304443991277261 0.002630493550224992 0.0006217530209622709 0.0008608887982554521 0.0007891480650674977 0.0009804566869020426 0.0016739504410522678 0.0021522219956386303 0.0014348146637590867</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>3582900.0 3964400.0 4159800.0 4316310.0 4646200.0 5007800.0 5639980.0 5960980.0 6187670.0 6493380.0 0.004505456673639645 0.0024953298500158027 0.017675253104278605 0.01330842586675095 0.006862157087543458 0.008941598629223294 0.0010743781298679152 0.010327892990343184 0.0017328679513998633 0.0035350506208557207</parameters>
    </source>
  </nuclide>
  <nuclide name="Ca52" half_life="4.6" decay_modes="2" decay_energy="5166534.65" reactions="0">
    <decay type="beta-" target="Sc52" branching_ratio="0.98"/>
    <decay type="beta-,n" target="Sc51" branching_ratio="0.02"/>
  </nuclide>
  <nuclide name="Ca53" half_life="0.09" decay_modes="2" decay_energy="5690609.0" reactions="0">
    <decay type="beta-" target="Sc53" branching_ratio="0.7"/>
    <decay type="beta-,n" target="Sc52" branching_ratio="0.3"/>
  </nuclide>
  <nuclide name="Ca54" half_life="0.086" decay_modes="1" decay_energy="7092000.0" reactions="0">
    <decay type="beta-" target="Sc54" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Ca55" half_life="0.022" decay_modes="1" decay_energy="8406000.0" reactions="0">
    <decay type="beta-" target="Sc55" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Ca56" half_life="0.01" decay_modes="1" decay_energy="8054000.0" reactions="0">
    <decay type="beta-" target="Sc56" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Ca57" half_life="0.005" decay_modes="2" decay_energy="8738666.0" reactions="0">
    <decay type="beta-" target="Sc57" branching_ratio="0.5"/>
    <decay type="beta-,n" target="Sc56" branching_ratio="0.5"/>
  </nuclide>
</nuclides>
  "#;

        let nuclides: SerdeNuclideVec = serde_xml_rs::from_str(xml).unwrap();
        assert_eq!(nuclides,get_calcium_xml_serde_data());
}
}
use crate::decay_xml_info_serde::SerdeNuclideVec;
pub fn get_calcium_xml_serde_data() -> SerdeNuclideVec {
    let xml = r#"
    <nuclides>
                <nuclide name="Ca34" half_life="3.5e-08" decay_modes="1" decay_energy="500000.0" reactions="0">
    <decay type="p" target="K33" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Ca35" half_life="0.0257" decay_modes="3" decay_energy="11687840.0" reactions="0">
    <decay type="ec/beta+" target="K35" branching_ratio="0.001"/>
    <decay type="ec/beta+,p" target="Ar34" branching_ratio="0.957"/>
    <decay type="ec/beta+,p,p" target="Cl33" branching_ratio="0.042"/>
  </nuclide>
  <nuclide name="Ca36" half_life="0.102" decay_modes="2" decay_energy="7136247.0" reactions="0">
    <decay type="ec/beta+" target="K36" branching_ratio="0.457"/>
    <decay type="ec/beta+,p" target="Ar35" branching_ratio="0.543"/>
  </nuclide>
  <nuclide name="Ca37" half_life="0.1811" decay_modes="2" decay_energy="7414493.0" reactions="0">
    <decay type="ec/beta+" target="K37" branching_ratio="0.179"/>
    <decay type="ec/beta+,p" target="Ar36" branching_ratio="0.821"/>
  </nuclide>
  <nuclide name="Ca38" half_life="0.44" decay_modes="2" decay_energy="3797364.0" reactions="0">
    <decay type="ec/beta+" target="K38" branching_ratio="0.0004017"/>
    <decay type="ec/beta+" target="K38_m1" branching_ratio="0.9995983"/>
    <source type="discrete" particle="photon">
      <parameters>237.3929 297.43 341.0853 3281.65 3284.66 3559.55 3559.83 328000.0 510998.9 1240000.0 1567700.0 1643000.0 1698000.0 2534000.0 2883000.0 3210000.0 3342000.0 3519000.0 3716000.0 3726000.0 3848000.0 2.4114511644955435e-06 1.8332073071239205e-08 2.7998357077737923e-07 6.290672990516796e-05 0.00012420849326709438 6.007246683720336e-06 1.1887206339737846e-05 0.04749633521336898 3.146582584848905 0.0007599413634139037 0.31664223475579323 0.0012665689390231728 0.0002533137878046346 3.324743464935828e-05 0.0022164956432905525 0.004369662839629946 0.00037997068170695183 0.0001266568939023173 6.332844695115865e-05 0.0006016202460360071 0.001773196514632442</parameters>
    </source>
    <source type="discrete" particle="positron">
      <parameters>2567700.0 2764200.0 2885700.0 3401000.0 3749300.0 5044100.0 6283900.0 6611900.0 6.301338005090412e-05 0.0018904014015271235 0.0005986271104835892 0.007876672506363015 3.623269352926987e-05 0.3150669002545206 0.044266899485760146 1.2051308934735414</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>40.57267 249.292 274.6179 2953.251 3238.985 3527.751 0.0006127569157593036 0.0026560754071911583 3.375381017144735e-05 0.001070941380119939 0.00021582618281165093 1.0990300668779799e-05</parameters>
    </source>
  </nuclide>
  <nuclide name="Ca39" half_life="0.8596" decay_modes="1" decay_energy="3581586.0" reactions="0">
    <decay type="ec/beta+" target="K39" branching_ratio="1.0"/>
    <source type="discrete" particle="photon">
      <parameters>234.3082 297.43 341.9012 3281.65 3284.66 3559.55 3559.83 510998.9 2522250.0 9.575567038748695e-07 7.1497357569019235e-09 1.247254487694145e-07 2.3312049188422222e-05 4.6029319070361396e-05 2.2261724781676806e-06 4.405174500578736e-06 1.6115373594765112 2.015900362261358e-05</parameters>
    </source>
    <source type="discrete" particle="positron">
      <parameters>2057600.0 2406600.0 2437600.0 2450600.0 2588600.0 2593600.0 2649600.0 2934600.0 3513600.0 3718600.0 4010260.0 6532600.0 2.015900362261358e-06 1.4514482608281775e-06 1.4514482608281775e-06 1.4111302535829504e-06 1.1692222101115876e-05 1.2498582246020418e-06 1.6127202898090861e-06 2.862578514411128e-06 1.3304942390924962e-06 1.4514482608281775e-06 2.015900362261358e-05 0.8063399859009205</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>40.57265 249.4437 275.2444 2953.251 3238.986 3527.751 0.0002777355117056312 0.0010378903333109846 1.3284259253208159e-05 0.0003968670788778139 7.998075817310343e-05 4.072801718810677e-06</parameters>
    </source>
  </nuclide>
  <nuclide name="Ca40" reactions="4">
    <reaction type="(n,2n)" Q="-15641200.0" target="Ca39"/>
    <reaction type="(n,gamma)" Q="8362701.0" target="Ca41"/>
    <reaction type="(n,p)" Q="-528741.0" target="K40"/>
    <reaction type="(n,a)" Q="1748330.0" target="Ar37"/>
  </nuclide>
  <nuclide name="Ca41" half_life="3218880000000.0" decay_modes="1" decay_energy="3111.7509" reactions="4">
    <decay type="ec/beta+" target="K41" branching_ratio="1.0"/>
    <source type="discrete" particle="positron">
      <parameters>421390.0 2.1533799972659598e-13</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>40.60326 249.2584 274.0613 2953.25 3239.025 3527.805 6.000768050501121e-14 3.2084069931264444e-13 4.72806448423701e-15 1.371671188234457e-13 2.75990100729589e-14 1.4026167661611669e-15</parameters>
    </source>
    <source type="discrete" particle="photon">
      <parameters>242.6217 297.43 338.9638 3281.65 3284.66 3559.55 3559.83 3.194496158344106e-16 2.4639469206116483e-18 3.601637867807178e-17 8.05698754229044e-15 1.5908411276661876e-14 7.693979355871335e-16 1.5224932772289656e-15</parameters>
    </source>
    <reaction type="(n,2n)" Q="-8362001.0" target="Ca40"/>
    <reaction type="(n,gamma)" Q="11481430.0" target="Ca42"/>
    <reaction type="(n,p)" Q="1204400.0" target="K41"/>
    <reaction type="(n,a)" Q="5224500.0" target="Ar38"/>
  </nuclide>
  <nuclide name="Ca42" reactions="4">
    <reaction type="(n,2n)" Q="-11480600.0" target="Ca41"/>
    <reaction type="(n,gamma)" Q="7933000.0" target="Ca43"/>
    <reaction type="(n,p)" Q="-2743090.0" target="K42"/>
    <reaction type="(n,a)" Q="341482.0" target="Ar39"/>
  </nuclide>
  <nuclide name="Ca43" reactions="5">
    <reaction type="(n,2n)" Q="-7933000.0" target="Ca42"/>
    <reaction type="(n,3n)" Q="-19413600.0" target="Ca41"/>
    <reaction type="(n,gamma)" Q="11132000.0" target="Ca44"/>
    <reaction type="(n,p)" Q="-1033050.0" target="K43"/>
    <reaction type="(n,a)" Q="2277860.0" target="Ar40"/>
  </nuclide>
  <nuclide name="Ca44" reactions="5">
    <reaction type="(n,2n)" Q="-11132000.0" target="Ca43"/>
    <reaction type="(n,3n)" Q="-19065000.0" target="Ca42"/>
    <reaction type="(n,gamma)" Q="7414770.0" target="Ca45"/>
    <reaction type="(n,p)" Q="-4876520.0" target="K44"/>
    <reaction type="(n,a)" Q="-2755410.0" target="Ar41"/>
  </nuclide>
  <nuclide name="Ca45" half_life="14049500.0" decay_modes="2" decay_energy="76860.28" reactions="6">
    <decay type="beta-" target="Sc45" branching_ratio="0.999981"/>
    <decay type="beta-" target="Sc45_m1" branching_ratio="1.9e-05"/>
    <source type="discrete" particle="electron">
      <parameters>243400.0 255800.0 9.373854180318845e-13 4.933513724783904e-08</parameters>
    </source>
    <reaction type="(n,2n)" Q="-7414820.0" target="Ca44"/>
    <reaction type="(n,3n)" Q="-18546000.0" target="Ca43"/>
    <reaction type="(n,4n)" Q="-26478900.0" target="Ca42"/>
    <reaction type="(n,gamma)" Q="10397600.0" target="Ca46"/>
    <reaction type="(n,p)" Q="-3414160.0" target="K45"/>
    <reaction type="(n,a)" Q="-743067.0" target="Ar42"/>
  </nuclide>
  <nuclide name="Ca46" reactions="5">
    <reaction type="(n,2n)" Q="-10393700.0" target="Ca45"/>
    <reaction type="(n,3n)" Q="-17808500.0" target="Ca44"/>
    <reaction type="(n,gamma)" Q="7276100.0" target="Ca47"/>
    <reaction type="(n,p)" Q="-6933630.0" target="K46"/>
    <reaction type="(n,a)" Q="-5510970.0" target="Ar43"/>
  </nuclide>
  <nuclide name="Ca47" half_life="391910.4" decay_modes="1" decay_energy="1346828.4" reactions="6">
    <decay type="beta-" target="Sc47" branching_ratio="1.0"/>
    <source type="discrete" particle="photon">
      <parameters>336.6881 401.8169 407.78 442.779 4052.12 4056.94 4426.2 4426.74 4458.72 4458.76 41060.0 489230.0 530600.0 731600.0 767100.0 807860.0 1146970.0 1297090.0 1878000.0 3.727950466882375e-13 1.3119820766246766e-14 2.4396806815479657e-15 4.925807535763675e-14 7.962353636851225e-12 1.5693367803342842e-11 8.787903802818563e-13 1.7375253028520534e-12 2.76952615064725e-17 4.081730898653019e-17 9.835389596943219e-11 1.0427882946156665e-07 1.516782973986424e-09 2.0144773873257195e-10 3.1876142187683443e-09 1.0427882946156665e-07 2.0144773873257195e-10 1.184986698426894e-06 4.502949454022197e-10</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>24.38053 41.68209 331.044 371.6687 3608.766 3991.67 4378.25 36567.2 40559.6 41060.0 113800.0 484737.2 488729.6 489230.0 526107.2 530099.6 530600.0 694880.0 727107.2 731099.6 731600.0 762607.2 766599.6 767100.0 803367.2 807359.6 807860.0 1225170.0 1292597.0 1296590.0 1297090.0 1873507.0 1877500.0 1878000.0 1992000.0 6.103648941262742e-10 5.079505616460233e-11 2.358268558092311e-10 6.198058777026962e-12 9.269648648924407e-11 2.104348900898651e-11 1.2040402233554802e-12 3.5702464236903885e-11 3.157160060618773e-12 4.2587236954764135e-13 6.543956394297772e-10 3.4724850210701694e-11 3.0657975861700595e-12 4.221207016604218e-13 2.5936988855167853e-13 2.275174460979636e-14 2.9788100118664634e-15 1.291104910226317e-06 2.7396892467629787e-14 2.3972280909176062e-15 3.3162326750156e-16 9.690347225055767e-13 8.574682248486846e-14 1.1806922712590574e-14 1.7383280871243162e-11 1.5297704282011828e-12 2.0037177081040032e-13 1.5387140710916382e-09 5.213941473078333e-11 4.5621987889435414e-12 2.4061036412888236e-11 1.031175424971083e-14 9.005898908044393e-16 9.017695735377767e-14 4.77531953097405e-07</parameters>
    </source>
    <reaction type="(n,2n)" Q="-7276380.0" target="Ca46"/>
    <reaction type="(n,3n)" Q="-17673900.0" target="Ca45"/>
    <reaction type="(n,4n)" Q="-25088800.0" target="Ca44"/>
    <reaction type="(n,gamma)" Q="9952630.0" target="Ca48"/>
    <reaction type="(n,p)" Q="-5849120.0" target="K47"/>
    <reaction type="(n,a)" Q="-4023790.0" target="Ar44"/>
  </nuclide>
  <nuclide name="Ca48" half_life="7.25824e+26" decay_modes="2" decay_energy="1649595.66" reactions="5">
    <decay type="beta-,beta-" target="Ti48" branching_ratio="0.75"/>
    <decay type="beta-" target="Sc48" branching_ratio="0.25"/>
    <reaction type="(n,2n)" Q="-9946370.0" target="Ca47"/>
    <reaction type="(n,3n)" Q="-17222500.0" target="Ca46"/>
    <reaction type="(n,gamma)" Q="5146630.0" target="Ca49"/>
    <reaction type="(n,p)" Q="-11307900.0" target="K48"/>
    <reaction type="(n,a)" Q="-8849000.0" target="Ar45"/>
  </nuclide>
  <nuclide name="Ca49" half_life="523.08" decay_modes="1" decay_energy="4042192.3" reactions="0">
    <decay type="beta-" target="Sc49" branching_ratio="1.0"/>
    <source type="discrete" particle="photon">
      <parameters>143200.0 712600.0 856100.0 976700.0 987300.0 1144500.0 1288400.0 1408900.0 2228900.0 2264700.0 2371700.0 2486300.0 3084400.0 4071900.0 4332000.0 4493000.0 4714400.0 4738200.0 4.107892214834883e-07 1.7889208032345458e-07 1.934684720535138e-06 9.938448906858586e-08 1.0203474211041482e-06 1.2986239904961887e-06 6.758145256663839e-07 7.937507860277724e-06 4.227153601717186e-06 5.565531387840809e-07 7.28819586502963e-06 4.107892214834883e-07 0.0012021547797736146 0.00010761352476346478 8.083271777578317e-08 4.637942823200674e-07 6.758145256663839e-08 3.723605523769684e-06</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>523649.9 547300.3 768660.1 929899.9 1190030.0 1745400.0 2177580.0 2890200.0 3033500.0 3.723605523769684e-06 4.770455475292122e-07 8.600071120734965e-06 8.083271777578317e-08 0.00010863387218456893 1.8816796596985592e-06 0.0011952641218648595 6.228094648298048e-06 2.385227737646061e-07</parameters>
    </source>
  </nuclide>
  <nuclide name="Ca50" half_life="13.9" decay_modes="2" decay_energy="2916307.0" reactions="0">
    <decay type="beta-" target="Sc50" branching_ratio="0.0035"/>
    <decay type="beta-" target="Sc50_m1" branching_ratio="0.9965"/>
    <source type="discrete" particle="photon">
      <parameters>336.9863 401.8169 407.78 442.6321 4052.12 4056.94 4426.2 4426.74 4458.72 4458.76 71552.0 328450.0 1519300.0 1590850.0 1.1656820695598603e-05 4.18222169799605e-07 7.64652542906731e-08 1.5355454051066472e-06 0.0002463432613034136 0.00048558076667201504 2.7193560161068503e-05 5.37668283955913e-05 8.570341020642805e-10 1.263093683113315e-09 0.02567137903253668 0.00017278812810361227 0.030657550729240913 0.018710485871791156</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>24.38613 41.69079 331.0748 371.6058 3608.885 3991.726 4378.25 67059.2 71051.6 71552.0 3118228.0 0.01885456075194035 0.0015477452941515385 0.007301427893215005 0.00019410160603857237 0.002863449845603426 0.0006505400716380727 3.725751367572926e-05 0.004364134435531235 0.00041074206452058686 5.3909895968327025e-05 0.04936803660103207</parameters>
    </source>
  </nuclide>
  <nuclide name="Ca51" half_life="10.0" decay_modes="1" decay_energy="4874199.0" reactions="0">
    <decay type="beta-" target="Sc51" branching_ratio="1.0"/>
    <source type="discrete" particle="photon">
      <parameters>352400.0 532200.0 547700.0 861600.0 1167500.0 1314800.0 1323700.0 1394000.0 1424000.0 1480100.0 1485300.0 1644400.0 1714800.0 1847100.0 1996500.0 2027500.0 2333400.0 2378700.0 2912000.0 3038900.0 3196500.0 3771700.0 0.00011956788864659056 0.002630493550224992 0.013630739305711324 0.02391357772931811 0.016261232855936315 0.0031087651048113544 0.009565431091727245 0.018413454851574945 0.0016739504410522678 0.015543825524056773 0.011239381532779511 0.0033479008821045357 0.005500122877743166 0.004304443991277261 0.002630493550224992 0.0006217530209622709 0.0008608887982554521 0.0007891480650674977 0.0009804566869020426 0.0016739504410522678 0.0021522219956386303 0.0014348146637590867</parameters>
    </source>
    <source type="discrete" particle="electron">
      <parameters>3582900.0 3964400.0 4159800.0 4316310.0 4646200.0 5007800.0 5639980.0 5960980.0 6187670.0 6493380.0 0.004505456673639645 0.0024953298500158027 0.017675253104278605 0.01330842586675095 0.006862157087543458 0.008941598629223294 0.0010743781298679152 0.010327892990343184 0.0017328679513998633 0.0035350506208557207</parameters>
    </source>
  </nuclide>
  <nuclide name="Ca52" half_life="4.6" decay_modes="2" decay_energy="5166534.65" reactions="0">
    <decay type="beta-" target="Sc52" branching_ratio="0.98"/>
    <decay type="beta-,n" target="Sc51" branching_ratio="0.02"/>
  </nuclide>
  <nuclide name="Ca53" half_life="0.09" decay_modes="2" decay_energy="5690609.0" reactions="0">
    <decay type="beta-" target="Sc53" branching_ratio="0.7"/>
    <decay type="beta-,n" target="Sc52" branching_ratio="0.3"/>
  </nuclide>
  <nuclide name="Ca54" half_life="0.086" decay_modes="1" decay_energy="7092000.0" reactions="0">
    <decay type="beta-" target="Sc54" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Ca55" half_life="0.022" decay_modes="1" decay_energy="8406000.0" reactions="0">
    <decay type="beta-" target="Sc55" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Ca56" half_life="0.01" decay_modes="1" decay_energy="8054000.0" reactions="0">
    <decay type="beta-" target="Sc56" branching_ratio="1.0"/>
  </nuclide>
  <nuclide name="Ca57" half_life="0.005" decay_modes="2" decay_energy="8738666.0" reactions="0">
    <decay type="beta-" target="Sc57" branching_ratio="0.5"/>
    <decay type="beta-,n" target="Sc56" branching_ratio="0.5"/>
  </nuclide>
</nuclides>
  "#;

    let nuclides: SerdeNuclideVec = serde_xml_rs::from_str(xml).unwrap();

    return nuclides;
}
