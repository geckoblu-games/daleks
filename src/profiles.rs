#[derive(Debug)]
pub enum Profiles {
    Classic,
    Robots2,
    Nightmare,
    Robots2Easy,
    ClassicWithSafeTeleports,
}

impl std::str::FromStr for Profiles {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let p = s.to_uppercase();
        match p.as_str() {
            "CLASSIC" => Ok(Profiles::Classic),
            "ROBOTS2" => Ok(Profiles::Robots2),
            "NIGHTMARE" => Ok(Profiles::Nightmare),
            "ROBOTS2EASY" => Ok(Profiles::Robots2Easy),
            "CLASSICWITHSAFETELEPORTS" => Ok(Profiles::ClassicWithSafeTeleports),
            _ => Err(format!("'{}' is not a valid value for Profiles", s)),
        }
    }
}

pub struct Profile {
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

impl Profile {
    // Classic Robots
    fn classic() -> Self {
        Profile {
            initial_type1: 10,
            initial_type2: 0,
            increment_type1: 10,
            increment_type2: 0,
            maximum_type1: 9999,
            maximum_type2: 0,
            score_type1: 10,
            score_type2: 0,
            score_type1_waiting: 10,
            score_type2_waiting: 0,
            score_type1_splatted: 10,
            score_type2_splatted: 0,
            num_robots_per_safe: 0,
            safe_score_boundary: 0,
            max_safe_teleports: 0,
            free_safe_teleports: 0,
            initial_safe_teleports: 0,
            moveable_heaps: false,
        }
    }

    // Robots2
    fn robots2() -> Self {
        Profile {
            initial_type1: 8,
            initial_type2: 2,
            increment_type1: 8,
            increment_type2: 2,
            maximum_type1: 9999,
            maximum_type2: 9999,
            score_type1: 10,
            score_type2: 20,
            score_type1_waiting: 10,
            score_type2_waiting: 20,
            score_type1_splatted: 20,
            score_type2_splatted: 40,
            num_robots_per_safe: 1,
            safe_score_boundary: 0,
            initial_safe_teleports: 1,
            free_safe_teleports: 0,
            max_safe_teleports: 10,
            moveable_heaps: true,
        }
    }

    // Nightmare
    fn nightmare() -> Self {
        Profile {
            initial_type1: 2,
            initial_type2: 8,
            increment_type1: 2,
            increment_type2: 8,
            maximum_type1: 9999,
            maximum_type2: 9999,
            score_type1: 10,
            score_type2: 20,
            score_type1_waiting: 10,
            score_type2_waiting: 20,
            score_type1_splatted: 20,
            score_type2_splatted: 40,
            num_robots_per_safe: 2,
            safe_score_boundary: 0,
            initial_safe_teleports: 1,
            free_safe_teleports: 1,
            max_safe_teleports: 10,
            moveable_heaps: true,
        }
    }

    // Slightly easier version of robots2
    fn robots2_easy() -> Self {
        let mut profile = Profile::robots2();
        profile.free_safe_teleports = 1;
        profile
    }

    // Classic Robots with safe teleports
    fn classic_with_safe_teleports() -> Self {
        let mut profile = Profile::classic();
        profile.num_robots_per_safe = 1;
        profile.max_safe_teleports = 10;
        profile
    }

    pub fn get_profile(profile: Profiles) -> Self {
        match profile {
            Profiles::Classic => Profile::classic(),
            Profiles::Robots2 => Profile::robots2(),
            Profiles::Nightmare => Profile::nightmare(),
            Profiles::Robots2Easy => Profile::robots2_easy(),
            Profiles::ClassicWithSafeTeleports => Profile::classic_with_safe_teleports(),
        }
    }
}
