/// this is for hydrogen isotopes 
pub mod hydrogen;

/// this is for helium isotopes 
pub mod helium;


/// this is for lithium isotopes 
pub mod lithium;

// notes, code was done from vibe coding using AI, then 
// modified
use serde::Deserialize;
#[derive(Debug, Deserialize)]
pub struct SerdeNuclideVec {
    #[serde(rename = "nuclide")]
    items: Vec<SerdeNuclideData>,
}

#[derive(Debug, Deserialize)]
pub struct SerdeNuclideData {
    // Attributes on <nuclide ...>
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@half_life")]
    half_life: Option<f64>,
    #[serde(rename = "@decay_modes")]
    decay_modes: Option<u32>,
    #[serde(rename = "@decay_energy")]
    decay_energy: Option<f64>,
    #[serde(rename = "@reactions")]
    reactions: Option<u32>,

    // Child elements (zero or more)
    #[serde(default)]
    reaction: Vec<RawReactionData>,
    #[serde(default)]
    decay: Vec<RawDecayData>,
    #[serde(default)]
    source: Vec<RawSourceData>,
}

#[derive(Debug, Deserialize)]
pub struct RawReactionData {
    #[serde(rename = "@type")]
    reaction_type: String,
    #[serde(rename = "@Q")]
    q: f64,
    #[serde(rename = "@target")]
    target: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RawDecayData {
    #[serde(rename = "@type")]
    decay_type: String,
    #[serde(rename = "@target")]
    target: Option<String>,
    #[serde(rename = "@branching_ratio")]
    branching_ratio: f64,
}

#[derive(Debug, Deserialize)]
pub struct RawSourceData {
    #[serde(rename = "@type")]
    source_type: String,
    #[serde(rename = "@particle")]
    particle: String,
    parameters: Parameters,
}

#[derive(Debug, Deserialize)]
pub struct Parameters {
    // Text content inside <parameters>...</parameters>
    #[serde(rename = "$value")]
    raw: Option<String>,
}
