/// this is for hydrogen isotopes 
pub mod hydrogen;

/// this is for helium isotopes 
pub mod helium;

/// this is for lithium isotopes 
pub mod lithium;

/// this is for beryllium isotopes 
pub mod beryllium;

/// this is for boron isotopes 
pub mod boron;


/// this is for carbon isotopes 
pub mod carbon;

/// this is for nitrogen isotopes 
pub mod nitrogen;

/// this is for oxygen isotopes 
pub mod oxygen;

/// this is for fluorine isotopes 
pub mod fluorine;

/// this is for neon isotopes 
pub mod neon;


/// this is for sodium isotopes 
pub mod sodium;

/// this is for magnesium isotopes 
pub mod magnesium;

/// this is for aluminium isotopes 
pub mod aluminium;

/// this is for silicon isotopes 
pub mod silicon;

/// this is for phosphorous isotopes 
pub mod phosphorous;

/// this is for sulfur isotopes 
pub mod sulfur;

/// this is for chlorine isotopes 
pub mod chlorine;

/// this is for argon isotopes 
pub mod argon;

/// this is for potassium isotopes 
pub mod potassium;

/// this is for calcium isotopes
pub mod calcium;

/// this is for scandium isotopes
pub mod scandium;

/// this is for titanium isotopes
pub mod titanium;


// 23–30
pub mod vanadium;
pub mod chromium;
pub mod manganese;
pub mod iron;
pub mod cobalt;
pub mod nickel;
pub mod copper;
pub mod zinc;

// 31–36
pub mod gallium;
pub mod germanium;
pub mod arsenic;
pub mod selenium;
pub mod bromine;
pub mod krypton;

// 37–44
pub mod rubidium;
pub mod strontium;
pub mod yttrium;
pub mod zirconium;
pub mod niobium;
pub mod molybdenum;
pub mod technetium;
pub mod ruthenium;

// 45–54
pub mod rhodium;
pub mod palladium;
pub mod silver;
pub mod cadmium;
pub mod indium;
pub mod tin;
pub mod antimony;
pub mod tellurium;
pub mod iodine;
pub mod xenon;

// 55–64
pub mod cesium;
pub mod barium;
pub mod lanthanum;
pub mod cerium;
pub mod praseodymium;
pub mod neodymium;
pub mod promethium;
pub mod samarium;
pub mod europium;
pub mod gadolinium;

// 65–74
pub mod terbium;
pub mod dysprosium;
pub mod holmium;
pub mod erbium;
pub mod thulium;
pub mod ytterbium;
pub mod lutetium;
pub mod hafnium;
pub mod tantalum;
pub mod tungsten;

// 75–84
pub mod rhenium;
pub mod osmium;
pub mod iridium;
pub mod platinum;
pub mod gold;
pub mod mercury;
pub mod thallium;
pub mod lead;
pub mod bismuth;
pub mod polonium;

// 85–94
pub mod astatine;
pub mod radon;
pub mod francium;
pub mod radium;
pub mod actinium;
pub mod thorium;
pub mod protactinium;
pub mod uranium;
pub mod neptunium;
pub mod plutonium;

// 95–104
pub mod americium;
pub mod curium;
pub mod berkelium;
pub mod californium;
pub mod einsteinium;
pub mod fermium;
pub mod mendelevium;
pub mod nobelium;
pub mod lawrencium;
pub mod rutherfordium;

// 105–114
pub mod dubnium;
pub mod seaborgium;
pub mod bohrium;
pub mod hassium;
pub mod meitnerium;
pub mod darmstadtium;
pub mod roentgenium;
// note, these modules do not have 
// data from openmc serde data
// pub mod copernicium;
// pub mod nihonium;
// pub mod flerovium;
// 
// // 115–118
// pub mod moscovium;
// pub mod livermorium;
// pub mod tennessine;
// pub mod oganesson;

// notes, code was done from vibe coding using AI, then 
// modified
use serde::Deserialize;
#[derive(Debug, Deserialize,PartialEq)]
pub struct SerdeNuclideVec {
    #[serde(rename = "nuclide")]
    pub nuclides: Vec<SerdeNuclideData>,
}

#[derive(Debug, Deserialize,PartialEq)]
pub struct SerdeNuclideData {
    // Attributes on <nuclide ...>
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@half_life")]
    pub half_life_seconds: Option<f64>,
    #[serde(rename = "@decay_modes")]
    decay_modes: Option<u32>,
    #[serde(rename = "@decay_energy")]
    pub decay_energy_electronvolt: Option<f64>,
    #[serde(rename = "@reactions")]
    reactions: Option<u32>,

    // Child elements (zero or more)
    #[serde(default)]
    reaction: Vec<RawReactionData>,
    #[serde(default)]
    pub raw_decay_data: Vec<RawDecayData>,
    #[serde(default)]
    source: Vec<RawSourceData>,
}


#[derive(Debug, Deserialize,PartialEq)]
pub struct RawReactionData {
    #[serde(rename = "@type")]
    reaction_type: String,
    #[serde(rename = "@Q")]
    q_value_electronvolt: f64,
    #[serde(rename = "@target")]
    target: Option<String>,
}

#[derive(Debug, Deserialize,PartialEq)]
pub struct RawDecayData {
    #[serde(rename = "@type")]
    pub decay_type: String,
    #[serde(rename = "@target")]
    pub target: Option<String>,
    #[serde(rename = "@branching_ratio")]
    pub branching_ratio: f64,
}

#[derive(Debug, Deserialize,PartialEq)]
pub struct RawSourceData {
    #[serde(rename = "@type")]
    source_type: String,
    #[serde(rename = "@particle")]
    particle: String,
    #[serde(rename = "@interpolation")]
    interpolation: Option<String>,
    parameters: Parameters,
}

#[derive(Debug, Deserialize,PartialEq)]
pub struct Parameters {
    // Text content inside <parameters>...</parameters>
    #[serde(rename = "$value")]
    raw: Option<String>,
}
