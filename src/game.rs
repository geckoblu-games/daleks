use std::cmp::min;
use std::io::stdin;
use std::thread;
use std::time::Duration;

use termion::event::Key;
use termion::input::TermRead;

use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::board::Board;
use crate::enums::*;
use crate::options::Options;

const CHANGE_DELAY: u64 = 1000;
const WAITING_DELAY: u64 = 200;

const WAITING_KILLS: [u16; 2] = [1, 2];

pub struct Game {
    board: Board,

    safe_moves: bool,

    arena_width: usize,
    arena_height: usize,
    arena: Vec<ObjectType>,

    state: GameState,
    level: u16,
    score: u64,
    safeteleports: u16,

    player_x: usize,
    player_y: usize,

    initial_type1: u16,
    initial_type2: u16,
    increment_type1: u16,
    increment_type2: u16,
    maximum_type1: u16,
    maximum_type2: u16,
    score_type: [[u64; 2]; 2],
    score_type_splatted: [u64; 2],
    num_robots_per_safe: u16,
    safe_score_boundary: u64,
    initial_safe_teleports: u16,
    free_safe_teleports: u16,
    max_safe_teleports: u16,
    moveable_heaps: bool,
}

impl Game {
    pub fn new(options: &Options) -> Result<Self, String> {
        let mut board = Board::new(options);

        board.init()?;

        let (arena_width, arena_height) = board.get_arena_size();
        let arena = vec![ObjectType::None; arena_width * arena_height];

        let score_type = [
            [options.score_type1, options.score_type2],
            [options.score_type1_waiting, options.score_type2_waiting],
        ];
        let score_type_splatted = [options.score_type1_splatted, options.score_type2_splatted];

        Ok(Game {
            board,

            safe_moves: options.safe_moves,

            arena_width,
            arena_height,
            arena,

            state: GameState::Playing,
            level: 0,
            score: 0,
            safeteleports: 0,

            player_x: 0,
            player_y: 0,

            initial_type1: options.initial_type1,
            initial_type2: options.initial_type2,
            increment_type1: options.increment_type1,
            increment_type2: options.increment_type2,
            maximum_type1: options.maximum_type1,
            maximum_type2: options.maximum_type2,
            score_type,
            score_type_splatted,
            num_robots_per_safe: options.num_robots_per_safe,
            safe_score_boundary: options.safe_score_boundary,
            initial_safe_teleports: options.initial_safe_teleports,
            free_safe_teleports: options.free_safe_teleports,
            max_safe_teleports: options.max_safe_teleports,
            moveable_heaps: options.moveable_heaps,
        })
    }

    pub fn new_game(&mut self) {
        self.score = 0;
        self.level = 0;

        self.generate_level();

        self.safeteleports = self.initial_safe_teleports;

        self.board.set_score(self.score);
        self.board.set_level(self.level);
        self.board.set_safeteleports(self.safeteleports);

        self.board.update(&self.arena);
    }

    pub fn generate_level(&mut self) {
        self.clear_arena();

        let num_robots1 = min(
            self.initial_type1 + self.increment_type1 * self.level,
            self.maximum_type1,
        );
        let num_robots2 = min(
            self.initial_type2 + self.increment_type2 * self.level,
            self.maximum_type2,
        );

        let max_robots = (self.arena_width * self.arena_height / 2) as u16;

        if (num_robots1 + num_robots2) > max_robots {
            self.state = GameState::GameComplete;
        } else {
            self.state = GameState::Playing;
        }

        self.safeteleports += self.free_safe_teleports;

        if self.safeteleports > self.max_safe_teleports {
            self.safeteleports = self.max_safe_teleports;
        }

        for _ in 0..num_robots1 {
            self.place_randomly(ObjectType::Robot1);
        }

        for _ in 0..num_robots2 {
            self.place_randomly(ObjectType::Robot2);
        }

        /*
        // Test
        self.arena[self.player_x - 1 + self.player_y * self.arena_width] = ObjectType::Heap;
        self.arena[self.player_x - 2 + self.player_y * self.arena_width] = ObjectType::Heap;

        self.arena[self.player_x - 5 + self.player_y * self.arena_width] = ObjectType::Robot1;

        // self.arena[self.player_x - 1 + (self.player_y - 5) * self.arena_width] = ObjectType::Robot1;
        // self.arena[self.player_x - 2 + (self.player_y - 5) * self.arena_width] = ObjectType::Robot1;

        self.arena[self.player_x - 9 + self.player_y * self.arena_width] = ObjectType::Robot2;
        // self.arena[self.player_x + 2 + self.player_y * self.arena_width] = ObjectType::Robot1;
        */

        self.board.set_score(self.score);
        self.board.set_level(self.level);
        self.board.set_safeteleports(self.safeteleports);
        self.board.update(&self.arena);
    }

    pub fn clear_arena(&mut self) {
        self.arena = vec![ObjectType::None; self.arena_width * self.arena_height];

        self.player_x = self.arena_width / 2;
        self.player_y = self.arena_height / 2;

        self.arena[self.player_x + self.player_y * self.arena_width] = ObjectType::Player;
    }

    pub fn run(&mut self) {
        for c in stdin().keys() {
            if let Key::Char(c) = c.unwrap() {
                match c.to_ascii_lowercase() {
                    'q' => {
                        if self.ask_quit() {
                            break;
                        } else {
                            self.board.update(&self.arena);
                        }
                    }
                    '7' | 'y' => self.player_command(PlayerCommand::NW),
                    '8' | 'k' => self.player_command(PlayerCommand::N),
                    '9' | 'u' => self.player_command(PlayerCommand::NE),
                    '4' | 'h' => self.player_command(PlayerCommand::W),
                    '5' | ' ' => self.player_command(PlayerCommand::Stay),
                    '6' | 'l' => self.player_command(PlayerCommand::E),
                    '1' | 'b' => self.player_command(PlayerCommand::SW),
                    '2' | 'j' => self.player_command(PlayerCommand::S),
                    '3' | 'n' => self.player_command(PlayerCommand::SE),
                    '+' | '\n' => self.player_command(PlayerCommand::SafeTeleport),
                    '-' | 't' => self.player_command(PlayerCommand::RandomTeleport),
                    'w' => self.player_command(PlayerCommand::Wait),
                    '?' => self.show_help(),
                    // 's' => {self.board.to_mainscreen()}
                    // 'a' => {self.board.to_alternatescreen()}
                    _ => {}
                }
            }

            if self.state == GameState::Waiting {
                self.waiting();
                // here I will be in state DEAD or LEVELCOMPLETE so I will match one of the following 'if'
            };
            if self.state == GameState::Dead {
                self.show_dead();
                if self.ask_new_game() {
                    self.new_game();
                } else {
                    break; // exit game
                }
            };
            if self.state == GameState::LevelComplete {
                self.show_level_complete();
                self.level += 1;
                self.generate_level();
                // here I will be in state PLAYING or GAMECOMPLETE so I could match the following 'if'
            };
            if self.state == GameState::GameComplete {
                if self.show_game_complete_and_ask() {
                    self.new_game();
                } else {
                    break; // exit game
                }
            };
        }

        self.board.close();
    }

    fn player_command(&mut self, command: PlayerCommand) {
        if self.state != GameState::Playing {
            return;
        }

        match command {
            PlayerCommand::NW
            | PlayerCommand::N
            | PlayerCommand::NE
            | PlayerCommand::W
            | PlayerCommand::Stay
            | PlayerCommand::E
            | PlayerCommand::SW
            | PlayerCommand::S
            | PlayerCommand::SE => {
                let (dx, dy) = command.to_direction();
                self.player_move(dx, dy, self.safe_moves);
            }
            PlayerCommand::SafeTeleport => self.safe_teleport(),
            PlayerCommand::RandomTeleport => self.random_teleport(),
            PlayerCommand::Wait => {
                self.state = GameState::Waiting;
            }
        }
        self.board.update(&self.arena);
    }

    fn player_move(&mut self, dx: i16, dy: i16, safe: bool) -> bool {
        let mut kills = 0;
        self.player_move_k(dx, dy, safe, &mut kills)
    }

    /*
     * Parameter kills is used only in WAITING state
     */
    fn player_move_k(&mut self, dx: i16, dy: i16, safe: bool, kills: &mut u16) -> bool {
        assert!(self.arena[self.player_x + self.player_y * self.arena_width] == ObjectType::Player);

        // check boundaries
        if (self.player_x == 0 && dx < 0)
            || (self.player_y == 0 && dy < 0)
            || (self.player_x == self.arena_width - 1 && dx > 0)
            || (self.player_y == self.arena_height - 1 && dy > 0)
        {
            return false;
        }

        // create a new arena with only heap
        let mut new_arena = vec![ObjectType::None; self.arena_width * self.arena_height];
        for i in 0..self.arena_width * self.arena_height {
            if self.arena[i] == ObjectType::Heap {
                new_arena[i] = ObjectType::Heap;
            }
        }

        // calculate new coords
        let player_x = (self.player_x as i16 + dx) as usize;
        let player_y = (self.player_y as i16 + dy) as usize;

        // check for heap and eventually move it
        if new_arena[player_x + player_y * self.arena_width] == ObjectType::Heap {
            if self.moveable_heaps {
                if !self.try_push_heap(&mut new_arena, player_x, player_y, dx, dy) {
                    return false;
                }
            } else {
                return false;
            }
        }

        // move player
        new_arena[player_x + player_y * self.arena_width] = ObjectType::Player;

        // move robots
        let result = self.move_robots(&mut new_arena, player_x, player_y);
        *kills += result.kills;

        // if 'safe moves' don't allow to dead
        if result.dead && safe {
            return false;
        }

        self.update_status(player_x, player_y, new_arena, &result);

        if result.dead {
            self.state = GameState::Dead;
        } else {
            let count = self.count_robots();
            if count == 0 {
                self.state = GameState::LevelComplete;
            }
        }

        true
    }

    fn try_push_heap(
        &self,
        new_arena: &mut Vec<ObjectType>,
        x: usize,
        y: usize,
        dx: i16,
        dy: i16,
    ) -> bool {
        assert!(new_arena[x + y * self.arena_width] == ObjectType::Heap);

        // check boundaries
        if (x == 0 && dx < 0)
            || (y == 0 && dy < 0)
            || (x == self.arena_width - 1 && dx > 0)
            || (y == self.arena_height - 1 && dy > 0)
        {
            return false;
        }

        // calculate new coords
        let new_x = (x as i16 + dx) as usize;
        let new_y = (y as i16 + dy) as usize;

        if new_arena[new_x + new_y * self.arena_width] == ObjectType::None {
            new_arena[x + y * self.arena_width] = ObjectType::None;
            new_arena[new_x + new_y * self.arena_width] = ObjectType::Heap;
            return true;
        } else if new_arena[new_x + new_y * self.arena_width] == ObjectType::Heap {
            if self.try_push_heap(new_arena, new_x, new_y, dx, dy) {
                new_arena[x + y * self.arena_width] = ObjectType::None;
                new_arena[new_x + new_y * self.arena_width] = ObjectType::Heap;
                return true;
            } else {
                return false;
            }
        }

        false
    }

    fn move_robots(
        &self,
        new_arena: &mut Vec<ObjectType>,
        player_x: usize,
        player_y: usize,
    ) -> MoveRobotsResult {
        let mut result = MoveRobotsResult::new();

        // move all robots of one step
        for y in 0..self.arena_height {
            for x in 0..self.arena_width {
                let i = x + y * self.arena_width;
                if self.arena[i].is_robot() {
                    let robot = self.arena[i];
                    self.move_robot(robot, x, y, new_arena, player_x, player_y, &mut result);
                }
            }
        }

        // move Robots2 of a second step
        let mut arena2 = vec![ObjectType::None; self.arena_width * self.arena_height];
        for i in 0..self.arena_width * self.arena_height {
            arena2[i] = new_arena[i];
            if new_arena[i] == ObjectType::Robot2 {
                new_arena[i] = ObjectType::None;
            }
        }
        for y in 0..self.arena_height {
            for x in 0..self.arena_width {
                let i = x + y * self.arena_width;
                if arena2[i] == ObjectType::Robot2 {
                    let robot = arena2[i];
                    self.move_robot(robot, x, y, new_arena, player_x, player_y, &mut result);
                }
            }
        }

        result
    }

    fn move_robot(
        &self,
        robot: ObjectType,
        x: usize,
        y: usize,
        new_arena: &mut Vec<ObjectType>,
        player_x: usize,
        player_y: usize,
        result: &mut MoveRobotsResult,
    ) {
        assert!(robot.is_robot());

        let i = x + y * self.arena_width;

        // the player jump over the robot?
        if new_arena[i] == ObjectType::Player {
            result.dead = true;
            return;
        }

        // the player push heap over the robot?
        if new_arena[i] == ObjectType::Heap {
            result.score += self.score_type_splatted[robot as usize];
            return;
        }

        // new robot coords
        let new_x = match player_x as isize - x as isize {
            d if d < 0 => x - 1,
            d if d > 0 => x + 1,
            _ => x,
        };
        let new_y = match player_y as isize - y as isize {
            d if d < 0 => y - 1,
            d if d > 0 => y + 1,
            _ => y,
        };
        let new_i = new_x + new_y * self.arena_width;

        // the robot jump over the player?
        if new_arena[new_i] == ObjectType::Player {
            result.dead = true;
            return;
        }

        // the robot jump over a heap?
        if new_arena[new_i] == ObjectType::Heap {
            result.score += self.score_type[self.state as usize][robot as usize];
            result.kills += WAITING_KILLS[robot as usize];
            return;
        }

        // robot collision?
        if new_arena[new_i].is_robot() {
            let robot2 = new_arena[new_i];
            result.score += self.score_type[self.state as usize][robot as usize];
            result.kills += WAITING_KILLS[robot as usize];
            result.score += self.score_type[self.state as usize][robot2 as usize];
            result.kills += WAITING_KILLS[robot2 as usize];
            new_arena[new_i] = ObjectType::Heap;
            return;
        }

        // move the robot
        new_arena[new_i] = robot;
    }

    fn count_robots(&self) -> u16 {
        // for (i, <item>) in new_arena.iter_mut().enumerate().take(self.arena_width * self.arena_height) {
        let mut count = 0;
        for object in &self.arena {
            if object.is_robot() {
                count += 1;
            }
        }
        count
    }

    fn safe_teleport(&mut self) {
        if self.safeteleports > 0 {
            let moved = self.teleport(true);
            if moved {
                self.safeteleports -= 1;
                self.board.set_safeteleports(self.safeteleports);
            }
        }
    }

    fn random_teleport(&mut self) {
        let moved = self.teleport(false);

        // this should never happen
        if !moved {
            panic!("No free space for a random teleport found");
        }
    }

    fn teleport(&mut self, safe: bool) -> bool {
        let mut rng = thread_rng();

        let mut x_list: Vec<usize> = (0..self.arena_width).collect();
        let mut y_list: Vec<usize> = (0..self.arena_height).collect();

        x_list.shuffle(&mut rng);
        y_list.shuffle(&mut rng);

        for &y in y_list.iter() {
            for &x in x_list.iter() {
                let i = x + y * self.arena_width;

                if self.arena[i] != ObjectType::None {
                    continue;
                }

                let dx = x as i16 - self.player_x as i16;
                let dy = y as i16 - self.player_y as i16;
                let moved = self.player_move(dx, dy, safe);

                if moved {
                    return true;
                }
            }
        }

        false
    }

    fn waiting(&mut self) {
        let mut kills = 0;
        loop {
            thread::sleep(Duration::from_millis(WAITING_DELAY));

            let prev_score = self.score;

            self.player_move_k(0, 0, false, &mut kills);

            self.board.update(&self.arena);

            if self.num_robots_per_safe > 0 {
                while kills >= self.num_robots_per_safe {
                    self.safeteleports += 1;
                    self.safeteleports = min(self.safeteleports, self.max_safe_teleports);
                    kills -= self.num_robots_per_safe;
                    self.board.set_safeteleports(self.safeteleports);
                }
            }

            let mut score_step = self.score - prev_score;
            if self.safe_score_boundary > 0 {
                while score_step >= self.safe_score_boundary {
                    self.safeteleports += 1;
                    self.safeteleports = min(self.safeteleports, self.max_safe_teleports);
                    score_step -= self.safe_score_boundary;
                    self.board.set_safeteleports(self.safeteleports);
                }
            }

            if self.state != GameState::Waiting {
                return;
            }
        }
    }

    fn place_randomly(&mut self, object: ObjectType) {
        let mut rng = thread_rng();

        let mut x_list: Vec<usize> = (0..self.arena_width).collect();
        let mut y_list: Vec<usize> = (0..self.arena_height).collect();

        x_list.shuffle(&mut rng);
        y_list.shuffle(&mut rng);

        for &y in y_list.iter() {
            for &x in x_list.iter() {
                let i = x + y * self.arena_width;

                if self.arena[i] == ObjectType::None {
                    self.arena[i] = object;
                    return;
                }
            }
        }

        panic!("Not able to place randomly");
    }

    fn update_status(
        &mut self,
        player_x: usize,
        player_y: usize,
        new_arena: Vec<ObjectType>,
        result: &MoveRobotsResult,
    ) {
        self.player_x = player_x;
        self.player_y = player_y;
        self.arena = new_arena;
        self.score += result.score;
        self.board.set_score(self.score);
    }

    fn show_dead(&mut self) {
        self.board
            .show_player_message(self.player_x, self.player_y, tr!("AARRrrgghhhh...."));
        thread::sleep(Duration::from_millis(CHANGE_DELAY))
    }

    fn show_level_complete(&mut self) {
        self.board
            .show_player_message(self.player_x, self.player_y, tr!("Yahoo!!"));
        thread::sleep(Duration::from_millis(CHANGE_DELAY))
    }

    fn ask_quit(&mut self) -> bool {
        let message = tr!("Do you really want to quit?\n(y\\n))");
        self.board.show_confirmation_dialog(message)
    }

    fn ask_new_game(&mut self) -> bool {
        let message = tr!("You are dead.\nDo you want to play another game?\n(y\\n))");
        self.board.show_confirmation_dialog(message)
    }

    fn show_game_complete_and_ask(&mut self) -> bool {
        let message = tr!(
            "Congratulations!\nYou completed the game.\nDo you want to play another game?\n(y\\n))"
        );
        self.board.show_confirmation_dialog(message)
    }

    fn show_help(&mut self) {
        self.board.show_dialog(HELP_MESSAGE);
        self.board.update(&self.arena);
    }
}

struct MoveRobotsResult {
    dead: bool,
    score: u64,
    kills: u16,
}

impl MoveRobotsResult {
    pub fn new() -> Self {
        MoveRobotsResult {
            dead: false,
            score: 0,
            kills: 0,
        }
    }
}

const HELP_MESSAGE: &str = " Escape from evil robots who want to exterminate you.

 Directions:

    7   8   9        y     k     u
      \\ | /           \\    |   /
    4 - 5 - 6        h - SPACE - l
      / | \\           /    |   \\
    1   2   3        b     j     n

 Commands:
    w          : wait for end
    + or ENTER : safe teleport
    - or t     : unsafe teleport
    q          : quit
    ?          : this help

";
