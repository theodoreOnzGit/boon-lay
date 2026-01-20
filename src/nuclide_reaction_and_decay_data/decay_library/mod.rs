use crate::prelude::NuclideReactionAndDecayData;
use fission_yields_data::prelude::Nuclide::*;

/// this is a full decay library constructed at start 
/// incorporating all decays from all radionuclides 

pub struct DecayLibrary {
    pub hydrogen_data: Vec<NuclideReactionAndDecayData>,
    pub helium_data: Vec<NuclideReactionAndDecayData>,
    pub lithium_data: Vec<NuclideReactionAndDecayData>,
    pub beryllium_data: Vec<NuclideReactionAndDecayData>,
    pub boron_data: Vec<NuclideReactionAndDecayData>,
    pub carbon_data: Vec<NuclideReactionAndDecayData>,
    pub nitrogen_data: Vec<NuclideReactionAndDecayData>,
    pub oxygen_data: Vec<NuclideReactionAndDecayData>,
    pub fluorine_data: Vec<NuclideReactionAndDecayData>,
    pub neon_data: Vec<NuclideReactionAndDecayData>,
    pub sodium_data: Vec<NuclideReactionAndDecayData>,
    pub magnesium_data: Vec<NuclideReactionAndDecayData>,
    pub aluminium_data: Vec<NuclideReactionAndDecayData>,
    pub silicon_data: Vec<NuclideReactionAndDecayData>,
    pub phosphorus_data: Vec<NuclideReactionAndDecayData>,
    pub sulfur_data: Vec<NuclideReactionAndDecayData>,
    pub chlorine_data: Vec<NuclideReactionAndDecayData>,
    pub argon_data: Vec<NuclideReactionAndDecayData>,
    pub potassium_data: Vec<NuclideReactionAndDecayData>,
    pub calcium_data: Vec<NuclideReactionAndDecayData>,
    pub scandium_data: Vec<NuclideReactionAndDecayData>,
    pub titanium_data: Vec<NuclideReactionAndDecayData>,
    pub vanadium_data: Vec<NuclideReactionAndDecayData>,
    pub chromium_data: Vec<NuclideReactionAndDecayData>,
    pub manganese_data: Vec<NuclideReactionAndDecayData>,
    pub iron_data: Vec<NuclideReactionAndDecayData>,
    pub cobalt_data: Vec<NuclideReactionAndDecayData>,
    pub nickel_data: Vec<NuclideReactionAndDecayData>,
    pub copper_data: Vec<NuclideReactionAndDecayData>,
    pub zinc_data: Vec<NuclideReactionAndDecayData>,
    pub gallium_data: Vec<NuclideReactionAndDecayData>,
    pub germanium_data: Vec<NuclideReactionAndDecayData>,
    pub arsenic_data: Vec<NuclideReactionAndDecayData>,
    pub selenium_data: Vec<NuclideReactionAndDecayData>,
    pub bromine_data: Vec<NuclideReactionAndDecayData>,
    pub krypton_data: Vec<NuclideReactionAndDecayData>,
    pub rubidium_data: Vec<NuclideReactionAndDecayData>,
    pub strontium_data: Vec<NuclideReactionAndDecayData>,
    pub yttrium_data: Vec<NuclideReactionAndDecayData>,
    pub zirconium_data: Vec<NuclideReactionAndDecayData>,
    pub niobium_data: Vec<NuclideReactionAndDecayData>,
    pub molybdenum_data: Vec<NuclideReactionAndDecayData>,
    pub technetium_data: Vec<NuclideReactionAndDecayData>,
    pub ruthenium_data: Vec<NuclideReactionAndDecayData>,
    pub rhodium_data: Vec<NuclideReactionAndDecayData>,
    pub palladium_data: Vec<NuclideReactionAndDecayData>,
    pub silver_data: Vec<NuclideReactionAndDecayData>,
    pub cadmium_data: Vec<NuclideReactionAndDecayData>,
    pub indium_data: Vec<NuclideReactionAndDecayData>,
    pub tin_data: Vec<NuclideReactionAndDecayData>,
    pub antimony_data: Vec<NuclideReactionAndDecayData>,
    pub tellurium_data: Vec<NuclideReactionAndDecayData>,
    pub iodine_data: Vec<NuclideReactionAndDecayData>,
    pub xenon_data: Vec<NuclideReactionAndDecayData>,
    pub cesium_data: Vec<NuclideReactionAndDecayData>,
    pub barium_data: Vec<NuclideReactionAndDecayData>,
    pub lanthanum_data: Vec<NuclideReactionAndDecayData>,
    pub cerium_data: Vec<NuclideReactionAndDecayData>,
    pub praseodymium_data: Vec<NuclideReactionAndDecayData>,
    pub neodymium_data: Vec<NuclideReactionAndDecayData>,
    pub promethium_data: Vec<NuclideReactionAndDecayData>,
    pub samarium_data: Vec<NuclideReactionAndDecayData>,
    pub europium_data: Vec<NuclideReactionAndDecayData>,
    pub gadolinium_data: Vec<NuclideReactionAndDecayData>,
    pub terbium_data: Vec<NuclideReactionAndDecayData>,
    pub dysprosium_data: Vec<NuclideReactionAndDecayData>,
    pub holmium_data: Vec<NuclideReactionAndDecayData>,
    pub erbium_data: Vec<NuclideReactionAndDecayData>,
    pub thulium_data: Vec<NuclideReactionAndDecayData>,
    pub ytterbium_data: Vec<NuclideReactionAndDecayData>,
    pub lutetium_data: Vec<NuclideReactionAndDecayData>,
    pub hafnium_data: Vec<NuclideReactionAndDecayData>,
    pub tantalum_data: Vec<NuclideReactionAndDecayData>,
    pub tungsten_data: Vec<NuclideReactionAndDecayData>,
    pub rhenium_data: Vec<NuclideReactionAndDecayData>,
    pub osmium_data: Vec<NuclideReactionAndDecayData>,
    pub iridium_data: Vec<NuclideReactionAndDecayData>,
    pub platinum_data: Vec<NuclideReactionAndDecayData>,
    pub gold_data: Vec<NuclideReactionAndDecayData>,
    pub mercury_data: Vec<NuclideReactionAndDecayData>,
    pub thallium_data: Vec<NuclideReactionAndDecayData>,
    pub lead_data: Vec<NuclideReactionAndDecayData>,
    pub bismuth_data: Vec<NuclideReactionAndDecayData>,
    pub polonium_data: Vec<NuclideReactionAndDecayData>,
    pub astatine_data: Vec<NuclideReactionAndDecayData>,
    pub radon_data: Vec<NuclideReactionAndDecayData>,
    pub francium_data: Vec<NuclideReactionAndDecayData>,
    pub radium_data: Vec<NuclideReactionAndDecayData>,
    pub actinium_data: Vec<NuclideReactionAndDecayData>,
    pub thorium_data: Vec<NuclideReactionAndDecayData>,
    pub protactinium_data: Vec<NuclideReactionAndDecayData>,
    pub uranium_data: Vec<NuclideReactionAndDecayData>,
    pub neptunium_data: Vec<NuclideReactionAndDecayData>,
    pub plutonium_data: Vec<NuclideReactionAndDecayData>,
    pub americium_data: Vec<NuclideReactionAndDecayData>,
    pub curium_data: Vec<NuclideReactionAndDecayData>,
    pub berkelium_data: Vec<NuclideReactionAndDecayData>,
    pub californium_data: Vec<NuclideReactionAndDecayData>,
    pub einsteinium_data: Vec<NuclideReactionAndDecayData>,
    pub fermium_data: Vec<NuclideReactionAndDecayData>,
    pub mendelevium_data: Vec<NuclideReactionAndDecayData>,
    pub nobelium_data: Vec<NuclideReactionAndDecayData>,
    pub lawrencium_data: Vec<NuclideReactionAndDecayData>,
    pub rutherfordium_data: Vec<NuclideReactionAndDecayData>,
    pub dubnium_data: Vec<NuclideReactionAndDecayData>,
    pub seaborgium_data: Vec<NuclideReactionAndDecayData>,
    pub bohrium_data: Vec<NuclideReactionAndDecayData>,
    pub hassium_data: Vec<NuclideReactionAndDecayData>,
    pub meitnerium_data: Vec<NuclideReactionAndDecayData>,
    pub darmstadtium_data: Vec<NuclideReactionAndDecayData>,
    pub roentgenium_data: Vec<NuclideReactionAndDecayData>,
    pub copernicium_data: Vec<NuclideReactionAndDecayData>,
    pub nihonium_data: Vec<NuclideReactionAndDecayData>,
    pub flerovium_data: Vec<NuclideReactionAndDecayData>,
    pub moscovium_data: Vec<NuclideReactionAndDecayData>,
    pub livermorium_data: Vec<NuclideReactionAndDecayData>,
    pub tennessine_data: Vec<NuclideReactionAndDecayData>,
    pub oganesson_data: Vec<NuclideReactionAndDecayData>,
}


impl DecayLibrary {
    
    pub fn new() -> Self {

        let hydrogen_data: Vec<NuclideReactionAndDecayData>;
        let helium_data: Vec<NuclideReactionAndDecayData>;
        let lithium_data: Vec<NuclideReactionAndDecayData>;
        let beryllium_data: Vec<NuclideReactionAndDecayData>;
        let boron_data: Vec<NuclideReactionAndDecayData>;
        let carbon_data: Vec<NuclideReactionAndDecayData>;
        let nitrogen_data: Vec<NuclideReactionAndDecayData>;
        let oxygen_data: Vec<NuclideReactionAndDecayData>;
        let fluorine_data: Vec<NuclideReactionAndDecayData>;
        let neon_data: Vec<NuclideReactionAndDecayData>;
        let sodium_data: Vec<NuclideReactionAndDecayData>;
        let magnesium_data: Vec<NuclideReactionAndDecayData>;
        let aluminium_data: Vec<NuclideReactionAndDecayData>;
        let silicon_data: Vec<NuclideReactionAndDecayData>;
        let phosphorus_data: Vec<NuclideReactionAndDecayData>;
        let sulfur_data: Vec<NuclideReactionAndDecayData>;
        let chlorine_data: Vec<NuclideReactionAndDecayData>;
        let argon_data: Vec<NuclideReactionAndDecayData>;
        let potassium_data: Vec<NuclideReactionAndDecayData>;
        let calcium_data: Vec<NuclideReactionAndDecayData>;
        let scandium_data: Vec<NuclideReactionAndDecayData>;
        let titanium_data: Vec<NuclideReactionAndDecayData>;
        let vanadium_data: Vec<NuclideReactionAndDecayData>;
        let chromium_data: Vec<NuclideReactionAndDecayData>;
        let manganese_data: Vec<NuclideReactionAndDecayData>;
        let iron_data: Vec<NuclideReactionAndDecayData>;
        let cobalt_data: Vec<NuclideReactionAndDecayData>;
        let nickel_data: Vec<NuclideReactionAndDecayData>;
        let copper_data: Vec<NuclideReactionAndDecayData>;
        let zinc_data: Vec<NuclideReactionAndDecayData>;
        let gallium_data: Vec<NuclideReactionAndDecayData>;
        let germanium_data: Vec<NuclideReactionAndDecayData>;
        let arsenic_data: Vec<NuclideReactionAndDecayData>;
        let selenium_data: Vec<NuclideReactionAndDecayData>;
        let bromine_data: Vec<NuclideReactionAndDecayData>;
        let krypton_data: Vec<NuclideReactionAndDecayData>;
        let rubidium_data: Vec<NuclideReactionAndDecayData>;
        let strontium_data: Vec<NuclideReactionAndDecayData>;
        let yttrium_data: Vec<NuclideReactionAndDecayData>;
        let zirconium_data: Vec<NuclideReactionAndDecayData>;
        let niobium_data: Vec<NuclideReactionAndDecayData>;
        let molybdenum_data: Vec<NuclideReactionAndDecayData>;
        let technetium_data: Vec<NuclideReactionAndDecayData>;
        let ruthenium_data: Vec<NuclideReactionAndDecayData>;
        let rhodium_data: Vec<NuclideReactionAndDecayData>;
        let palladium_data: Vec<NuclideReactionAndDecayData>;
        let silver_data: Vec<NuclideReactionAndDecayData>;
        let cadmium_data: Vec<NuclideReactionAndDecayData>;
        let indium_data: Vec<NuclideReactionAndDecayData>;
        let tin_data: Vec<NuclideReactionAndDecayData>;
        let antimony_data: Vec<NuclideReactionAndDecayData>;
        let tellurium_data: Vec<NuclideReactionAndDecayData>;
        let iodine_data: Vec<NuclideReactionAndDecayData>;
        let xenon_data: Vec<NuclideReactionAndDecayData>;
        let cesium_data: Vec<NuclideReactionAndDecayData>;
        let barium_data: Vec<NuclideReactionAndDecayData>;
        let lanthanum_data: Vec<NuclideReactionAndDecayData>;
        let cerium_data: Vec<NuclideReactionAndDecayData>;
        let praseodymium_data: Vec<NuclideReactionAndDecayData>;
        let neodymium_data: Vec<NuclideReactionAndDecayData>;
        let promethium_data: Vec<NuclideReactionAndDecayData>;
        let samarium_data: Vec<NuclideReactionAndDecayData>;
        let europium_data: Vec<NuclideReactionAndDecayData>;
        let gadolinium_data: Vec<NuclideReactionAndDecayData>;
        let terbium_data: Vec<NuclideReactionAndDecayData>;
        let dysprosium_data: Vec<NuclideReactionAndDecayData>;
        let holmium_data: Vec<NuclideReactionAndDecayData>;
        let erbium_data: Vec<NuclideReactionAndDecayData>;
        let thulium_data: Vec<NuclideReactionAndDecayData>;
        let ytterbium_data: Vec<NuclideReactionAndDecayData>;
        let lutetium_data: Vec<NuclideReactionAndDecayData>;
        let hafnium_data: Vec<NuclideReactionAndDecayData>;
        let tantalum_data: Vec<NuclideReactionAndDecayData>;
        let tungsten_data: Vec<NuclideReactionAndDecayData>;
        let rhenium_data: Vec<NuclideReactionAndDecayData>;
        let osmium_data: Vec<NuclideReactionAndDecayData>;
        let iridium_data: Vec<NuclideReactionAndDecayData>;
        let platinum_data: Vec<NuclideReactionAndDecayData>;
        let gold_data: Vec<NuclideReactionAndDecayData>;
        let mercury_data: Vec<NuclideReactionAndDecayData>;
        let thallium_data: Vec<NuclideReactionAndDecayData>;
        let lead_data: Vec<NuclideReactionAndDecayData>;
        let bismuth_data: Vec<NuclideReactionAndDecayData>;
        let polonium_data: Vec<NuclideReactionAndDecayData>;
        let astatine_data: Vec<NuclideReactionAndDecayData>;
        let radon_data: Vec<NuclideReactionAndDecayData>;
        let francium_data: Vec<NuclideReactionAndDecayData>;
        let radium_data: Vec<NuclideReactionAndDecayData>;
        let actinium_data: Vec<NuclideReactionAndDecayData>;
        let thorium_data: Vec<NuclideReactionAndDecayData>;
        let protactinium_data: Vec<NuclideReactionAndDecayData>;
        let uranium_data: Vec<NuclideReactionAndDecayData>;
        let neptunium_data: Vec<NuclideReactionAndDecayData>;
        let plutonium_data: Vec<NuclideReactionAndDecayData>;
        let americium_data: Vec<NuclideReactionAndDecayData>;
        let curium_data: Vec<NuclideReactionAndDecayData>;
        let berkelium_data: Vec<NuclideReactionAndDecayData>;
        let californium_data: Vec<NuclideReactionAndDecayData>;
        let einsteinium_data: Vec<NuclideReactionAndDecayData>;
        let fermium_data: Vec<NuclideReactionAndDecayData>;
        let mendelevium_data: Vec<NuclideReactionAndDecayData>;
        let nobelium_data: Vec<NuclideReactionAndDecayData>;
        let lawrencium_data: Vec<NuclideReactionAndDecayData>;
        let rutherfordium_data: Vec<NuclideReactionAndDecayData>;
        let dubnium_data: Vec<NuclideReactionAndDecayData>;
        let seaborgium_data: Vec<NuclideReactionAndDecayData>;
        let bohrium_data: Vec<NuclideReactionAndDecayData>;
        let hassium_data: Vec<NuclideReactionAndDecayData>;
        let meitnerium_data: Vec<NuclideReactionAndDecayData>;
        let darmstadtium_data: Vec<NuclideReactionAndDecayData>;
        let roentgenium_data: Vec<NuclideReactionAndDecayData>;
        let copernicium_data: Vec<NuclideReactionAndDecayData> = vec![];
        let nihonium_data: Vec<NuclideReactionAndDecayData> = vec![];
        let flerovium_data: Vec<NuclideReactionAndDecayData> = vec![];
        let moscovium_data: Vec<NuclideReactionAndDecayData> = vec![];
        let livermorium_data: Vec<NuclideReactionAndDecayData> = vec![];
        let tennessine_data: Vec<NuclideReactionAndDecayData> = vec![];
        let oganesson_data: Vec<NuclideReactionAndDecayData> = vec![];

        hydrogen_data = NuclideReactionAndDecayData::
            parse_nuclides_to_decay_data_vec_by_element(
                &H1
            );
        // vibe coded for speed
        helium_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&He4);
        lithium_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&Li7);
        beryllium_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&Be9);
        boron_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&B11);
        carbon_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&C12);
        nitrogen_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&N14);
        oxygen_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&O16);
        fluorine_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&F19);
        neon_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&Ne20);
        sodium_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&Na23);
        magnesium_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&Mg24);
        aluminium_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&Al27);
        silicon_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&Si28);
        phosphorus_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&P31);
        sulfur_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&S32);
        chlorine_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&Cl35);
        argon_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&Ar40);
        potassium_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&K39);
        calcium_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&Ca40);
        scandium_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&Sc45);
        titanium_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&Ti48);
        vanadium_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&V51);
        chromium_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&Cr52);
        manganese_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&Mn55);
        iron_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&Fe56);
        cobalt_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&Co59);
        nickel_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&Ni58);
        copper_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&Cu63);
        zinc_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&Zn64);
        gallium_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&Ga69);
        germanium_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&Ge74);
        arsenic_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&As75);
        selenium_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&Se80);
        bromine_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&Br79);
        krypton_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&Kr84);
        rubidium_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&Rb85);
        strontium_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&Sr88);
        yttrium_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&Y89);
        zirconium_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&Zr90);
        niobium_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&Nb93);
        molybdenum_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&Mo98);
        technetium_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&Tc99);
        ruthenium_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&Ru102);
        rhodium_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&Rh103);
        palladium_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&Pd106);
        silver_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&Ag107);
        cadmium_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&Cd114);
        indium_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&In115);
        tin_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&Sn120);
        antimony_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&Sb121);
        tellurium_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&Te130);
        iodine_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&I127);
        xenon_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&Xe132);
        cesium_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&Cs133);
        barium_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&Ba138);
        lanthanum_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&La139);
        cerium_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&Ce140);
        praseodymium_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&Pr141);
        neodymium_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&Nd142);
        promethium_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&Pm147);
        samarium_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&Sm152);
        europium_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&Eu153);
        gadolinium_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&Gd158);
        terbium_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&Tb159);
        dysprosium_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&Dy164);
        holmium_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&Ho165);
        erbium_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&Er166);
        thulium_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&Tm169);
        ytterbium_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&Yb174);
        lutetium_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&Lu175);
        hafnium_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&Hf180);
        tantalum_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&Ta181);
        tungsten_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&W184);
        rhenium_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&Re187);
        osmium_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&Os192);
        iridium_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&Ir193);
        platinum_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&Pt195);
        gold_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&Au197);
        mercury_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&Hg202);
        thallium_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&Tl205);
        lead_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&Pb208);
        bismuth_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&Bi209);
        polonium_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&Po209);
        astatine_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&At210);
        radon_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&Rn222);
        francium_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&Fr223);
        // vibe coded for speed 
        radium_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&Ra226);
        actinium_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&Ac227);
        thorium_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&Th232);
        protactinium_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&Pa231);
        uranium_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&U238);
        neptunium_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&Np237);
        plutonium_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&Pu239);
        americium_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&Am241);
        curium_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&Cm244);
        berkelium_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&Bk249);
        californium_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&Cf252);
        einsteinium_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&Es253);
        fermium_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&Fm257);
        mendelevium_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&Md256);
        nobelium_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&No259);
        lawrencium_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&Lr262);
        rutherfordium_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&Rf267);
        dubnium_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&Db268);
        seaborgium_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&Sg271);
        bohrium_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&Bh270);
        hassium_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&Hs277);
        meitnerium_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&Mt278);
        darmstadtium_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&Ds281);
        roentgenium_data = NuclideReactionAndDecayData::parse_nuclides_to_decay_data_vec_by_element(&Rg281);

        return Self {
            hydrogen_data,
            helium_data,
            lithium_data,
            beryllium_data,
            boron_data,
            carbon_data,
            nitrogen_data,
            oxygen_data,
            fluorine_data,
            neon_data,
            sodium_data,
            magnesium_data,
            aluminium_data,
            silicon_data,
            phosphorus_data,
            sulfur_data,
            chlorine_data,
            argon_data,
            potassium_data,
            calcium_data,
            scandium_data,
            titanium_data,
            vanadium_data,
            chromium_data,
            manganese_data,
            iron_data,
            cobalt_data,
            nickel_data,
            copper_data,
            zinc_data,
            gallium_data,
            germanium_data,
            arsenic_data,
            selenium_data,
            bromine_data,
            krypton_data,
            rubidium_data,
            strontium_data,
            yttrium_data,
            zirconium_data,
            niobium_data,
            molybdenum_data,
            technetium_data,
            ruthenium_data,
            rhodium_data,
            palladium_data,
            silver_data,
            cadmium_data,
            indium_data,
            tin_data,
            antimony_data,
            tellurium_data,
            iodine_data,
            xenon_data,
            cesium_data,
            barium_data,
            lanthanum_data,
            cerium_data,
            praseodymium_data,
            neodymium_data,
            promethium_data,
            samarium_data,
            europium_data,
            gadolinium_data,
            terbium_data,
            dysprosium_data,
            holmium_data,
            erbium_data,
            thulium_data,
            ytterbium_data,
            lutetium_data,
            hafnium_data,
            tantalum_data,
            tungsten_data,
            rhenium_data,
            osmium_data,
            iridium_data,
            platinum_data,
            gold_data,
            mercury_data,
            thallium_data,
            lead_data,
            bismuth_data,
            polonium_data,
            astatine_data,
            radon_data,
            francium_data,
            radium_data,
            actinium_data,
            thorium_data,
            protactinium_data,
            uranium_data,
            neptunium_data,
            plutonium_data,
            americium_data,
            curium_data,
            berkelium_data,
            californium_data,
            einsteinium_data,
            fermium_data,
            mendelevium_data,
            nobelium_data,
            lawrencium_data,
            rutherfordium_data,
            dubnium_data,
            seaborgium_data,
            bohrium_data,
            hassium_data,
            meitnerium_data,
            darmstadtium_data,
            roentgenium_data,
            copernicium_data,
            nihonium_data,
            flerovium_data,
            moscovium_data,
            livermorium_data,
            tennessine_data,
            oganesson_data,
        };

    }
}

/// this allows users to use nuclides to get appropriate decay data
pub mod indexing_using_nuclide;
