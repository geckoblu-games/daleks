#[derive(PartialEq, Clone, Copy)]
pub enum ObjectType {
    Robot1 = 0,
    Robot2 = 1,
    Player,
    Heap,
    None,
}

impl ObjectType {
    pub fn is_robot(&self) -> bool {
        *self == ObjectType::Robot1 || *self == ObjectType::Robot2
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum GameState {
    Playing = 0,
    Waiting = 1,
    GameComplete,
    LevelComplete,
    Dead,
}

pub enum PlayerCommand {
    NW,
    N,
    NE,
    W,
    Stay,
    E,
    SW,
    S,
    SE,
    SafeTeleport,
    RandomTeleport,
    Wait,
}

impl PlayerCommand {
    pub fn to_direction(&self) -> (i16, i16) {
        match self {
            PlayerCommand::NW => (-1, -1),
            PlayerCommand::N => (0, -1),
            PlayerCommand::NE => (1, -1),
            PlayerCommand::W => (-1, 0),
            PlayerCommand::Stay => (0, 0),
            PlayerCommand::E => (1, 0),
            PlayerCommand::SW => (-1, 1),
            PlayerCommand::S => (0, 1),
            PlayerCommand::SE => (1, 1),
            _ => (0, 0),
        }
    }
}
