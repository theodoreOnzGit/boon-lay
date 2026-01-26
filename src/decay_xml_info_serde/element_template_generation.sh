#!/usr/bin/env bash
set -euo pipefail

elements=(
vanadium chromium manganese iron cobalt nickel copper zinc
gallium germanium arsenic selenium bromine krypton
rubidium strontium yttrium zirconium niobium molybdenum technetium ruthenium
rhodium palladium silver cadmium indium tin antimony tellurium iodine xenon
cesium barium lanthanum cerium praseodymium neodymium promethium samarium europium gadolinium
terbium dysprosium holmium erbium thulium ytterbium lutetium hafnium tantalum tungsten
rhenium osmium iridium platinum gold mercury thallium lead bismuth polonium
astatine radon francium radium actinium thorium protactinium uranium neptunium plutonium
americium curium berkelium californium einsteinium fermium mendelevium nobelium lawrencium rutherfordium
dubnium seaborgium bohrium hassium meitnerium darmstadtium roentgenium copernicium nihonium flerovium
moscovium livermorium tennessine oganesson
)

for e in "${elements[@]}"; do
  touch "${e}.rs"
  cat > "${e}.rs" <<EOF
#[cfg(test)]
mod parsing_tests {
#[test]
fn serde_nuclide_test() {
    use crate::decay_xml_info_serde::SerdeNuclideVec;
    let xml = r#"
    <nuclides>

</nuclides>
  "#;

    let nuclides: SerdeNuclideVec = serde_xml_rs::from_str(xml).unwrap();
    dbg!("{:#?}", nuclides);
}
}

EOF
done
