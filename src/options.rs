use serde::{Deserialize, Serialize};
use std::fs;

use crate::profiles;

#[derive(Serialize, Deserialize, Debug)]
pub enum BoardSize {
    Normal,
}

impl Default for BoardSize {
    fn default() -> Self {
        BoardSize::Normal
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoardType {
    Normal,
    Bsd,
}

impl Default for BoardType {
    fn default() -> Self {
        BoardType::Normal
    }
}

impl std::str::FromStr for BoardType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let p = s.to_uppercase();
        match p.as_str() {
            "NORMAL" => Ok(BoardType::Normal),
            "BSD" => Ok(BoardType::Bsd),
            _ => Err(format!("'{}' is not a valid value for BoardType", s)),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Options {
    pub boardsize: BoardSize,
    pub boardtype: BoardType,

    pub asciionly: bool,
    pub colors: bool,

    pub safe_moves: bool,

    pub initial_type1: u16,
    pub initial_type2: u16,
    pub increment_type1: u16,
    pub increment_type2: u16,
    pub maximum_type1: u16,
    pub maximum_type2: u16,
    pub score_type1: u64,
    pub score_type2: u64,
    pub score_type1_waiting: u64,
    pub score_type2_waiting: u64,
    pub score_type1_splatted: u64,
    pub score_type2_splatted: u64,
    pub num_robots_per_safe: u16,
    pub safe_score_boundary: u64,
    pub initial_safe_teleports: u16,
    pub free_safe_teleports: u16,
    pub max_safe_teleports: u16,
    pub moveable_heaps: bool,
}

impl Options {
    pub fn default() -> Self {
        let mut options = Options {
            boardsize: BoardSize::Normal,
            boardtype: BoardType::Normal,

            asciionly: false,
            colors: true,

            safe_moves: true,

            ..Default::default()
        };

        options.set_profile(profiles::Profiles::Robots2);

        options
    }

    pub fn load() -> Self {
        let xdg_dirs = xdg::BaseDirectories::with_prefix("daleks").unwrap();

        let config_path = xdg_dirs
            .place_config_file("config.json")
            .expect("Cannot create configuration directory");

        if let Ok(serialized) = fs::read_to_string(config_path) {
            if let Ok(options) = serde_json::from_str(&serialized) {
                return options;
            }
        }

        Options::default()
    }

    pub fn store(&self) {
        let xdg_dirs = xdg::BaseDirectories::with_prefix("daleks").unwrap();

        let config_path = xdg_dirs
            .place_config_file("config.json")
            .expect("Cannot create configuration directory");

        let serialized = serde_json::to_string_pretty(&self).unwrap();

        fs::write(config_path, serialized).unwrap();
    }

    pub fn set_profile(&mut self, profile: profiles::Profiles) {
        let profile = profiles::Profile::get_profile(profile);

        self.initial_type1 = profile.initial_type1;
        self.initial_type2 = profile.initial_type2;
        self.increment_type1 = profile.increment_type1;
        self.increment_type2 = profile.increment_type2;
        self.maximum_type1 = profile.maximum_type1;
        self.maximum_type2 = profile.maximum_type2;
        self.score_type1 = profile.score_type1;
        self.score_type2 = profile.score_type2;
        self.score_type1_waiting = profile.score_type1_waiting;
        self.score_type2_waiting = profile.score_type2_waiting;
        self.score_type1_splatted = profile.score_type1_splatted;
        self.score_type2_splatted = profile.score_type2_splatted;
        self.num_robots_per_safe = profile.num_robots_per_safe;
        self.safe_score_boundary = profile.safe_score_boundary;
        self.max_safe_teleports = profile.max_safe_teleports;
        self.free_safe_teleports = profile.free_safe_teleports;
        self.initial_safe_teleports = profile.initial_safe_teleports;
        self.moveable_heaps = profile.moveable_heaps;
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn base() {
        let options = super::Options::load();

        options.store();
    }
}
