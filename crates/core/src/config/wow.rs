use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Struct for settings related to World of Warcraft.
#[serde(default)]
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
pub struct Wow {
    #[serde(default)]
    pub directory: Option<PathBuf>,

    #[serde(default)]
    pub flavor: Flavor,
}

impl Default for Wow {
    fn default() -> Self {
        Wow {
            directory: None,
            flavor: Flavor::Retail,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize, Hash)]
pub enum Flavor {
    #[serde(alias = "retail")]
    Retail,
    #[serde(alias = "classic")]
    Classic,
    #[serde(alias = "ptr")]
    RetailPTR,
    #[serde(alias = "classic_ptr")]
    ClassicPTR,
    #[serde(alias = "beta")]
    Beta,
}

impl Flavor {
    pub const ALL: [Flavor; 5] = [
        Flavor::Retail, 
        Flavor::Classic, 
        Flavor::RetailPTR,
        Flavor::ClassicPTR, 
        Flavor::Beta,
    ];
}

impl Default for Flavor {
    fn default() -> Flavor {
        Flavor::Retail
    }
}

impl std::fmt::Display for Flavor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Flavor::Retail => "retail",
                Flavor::Classic => "classic",
                Flavor::RetailPTR => "ptr",
                Flavor::ClassicPTR => "classic_ptr",
                Flavor::Beta => "beta",
            }
        )
    }
}

