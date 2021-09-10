use std::cmp::max;
use std::io::{stdin, stdout, Stdout, Write};

use termion::color;
use termion::cursor;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::raw::RawTerminal;
// use termion::screen;
use termion::terminal_size;

use crate::enums::ObjectType;
use crate::options::BoardSize;
use crate::options::BoardType;
use crate::options::Options;

pub struct Board {
    // Immutable fields
    board_width: usize,
    board_height: usize,
    boardtype: BoardType,
    //stdout: screen::AlternateScreen<RawTerminal<Stdout>>,
    stdout: RawTerminal<Stdout>,
    asciionly: bool,
    colors: bool,

    // Derived fields
    delta_x: usize,
    delta_y: usize,

    arena_width: usize,
    arena_height: usize,

    // Runtime fields
    level: u16,
    score: u64,
    safeteleports: u16,
}

impl Board {
    pub fn new(options: &Options) -> Self {
        let board_width;
        let board_height;

        match options.boardsize {
            BoardSize::Normal => {
                board_width = 80;
                board_height = 24;
            }
        }

        let arena_width;
        let arena_height;

        match options.boardtype {
            BoardType::Normal => {
                arena_width = board_width - 2;
                arena_height = board_height - 3;
            }
            BoardType::Bsd => {
                arena_width = board_width - 2 - 19;
                arena_height = board_height - 2;
            }
        }

        Board {
            board_width,
            board_height,
            boardtype: options.boardtype,
            //stdout: screen::AlternateScreen::from(stdout().into_raw_mode().unwrap()),
            stdout: stdout().into_raw_mode().unwrap(),
            asciionly: options.asciionly,
            colors: options.colors,

            delta_x: 0,
            delta_y: 0,

            arena_width,
            arena_height,

            level: 0,
            score: 0,
            safeteleports: 0,
        }
    }

    pub fn close(&mut self) {
        write!(self.stdout, "\n\n\r").unwrap();
        write!(self.stdout, "{}", termion::cursor::Show).unwrap();
    }

    pub fn init(&mut self) -> Result<(), String> {
        let (terminal_width, terminal_height) = terminal_size().unwrap();
        let terminal_width = terminal_width as usize;
        let terminal_height = terminal_height as usize;

        if terminal_width < self.board_width {
            return Err(format!(
                "Terminal width is too small for board ({} < {})",
                terminal_width, self.board_width
            ));
        }
        if terminal_height < self.board_height {
            return Err(format!(
                "Terminal height is too small for board ({} < {})",
                terminal_height, self.board_height
            ));
        }

        self.delta_x = (terminal_width - self.board_width) / 2;
        self.delta_y = (terminal_height - self.board_height) / 2;

        write!(self.stdout, "{}", termion::clear::All).unwrap();
        write!(self.stdout, "{}", termion::cursor::Hide).unwrap();

        // let backgroundcolor = color::Bg(color::Rgb(19, 25, 38));
        // write!(self.stdout, "{}{}", backgroundcolor, termion::clear::All).unwrap();
        // write!(self.stdout, "{}{}", color::Bg(color::Blue), termion::clear::All).unwrap();

        if self.colors {
            write!(self.stdout, "{}", color::Bg(color::Black)).unwrap();
        }

        // Clear background
        for x in 0..self.board_width {
            for y in 0..self.board_height {
                self.write_at(x, y, " ", &color::Reset);
            }
        }

        self.draw_walls();
        self.draw_status();
        // self.draw_arena();

        self.stdout.flush().unwrap();

        Ok(())
    }

    fn draw_walls(&mut self) {
        self.draw_borders(
            0,
            0,
            self.arena_width + 2,
            self.arena_height + 2,
            COLOR_WALL,
        );
        if self.boardtype == BoardType::Bsd {
            self.write_at(self.arena_width + 3, 0, tr!("Directions:"), COLOR_TEXT);
            self.write_at(self.arena_width + 3, 2, "y k u", COLOR_TEXT);
            self.write_at(self.arena_width + 3, 3, " \\!/", COLOR_TEXT);
            self.write_at(self.arena_width + 3, 4, "h- -l", COLOR_TEXT);
            self.write_at(self.arena_width + 3, 5, " /|\\", COLOR_TEXT);
            self.write_at(self.arena_width + 3, 6, "b j n", COLOR_TEXT);

            let mut l = 8;
            self.write_at(self.arena_width + 3, l, tr!("Commands:"), COLOR_TEXT);
            self.write_at(
                self.arena_width + 3,
                l + 2,
                tr!("w: wait for end"),
                COLOR_TEXT,
            );
            self.write_at(
                self.arena_width + 3,
                l + 3,
                tr!("+: safe teleport"),
                COLOR_TEXT,
            );
            self.write_at(
                self.arena_width + 3,
                l + 4,
                tr!("-: unsafe teleport"),
                COLOR_TEXT,
            );
            self.write_at(self.arena_width + 3, l + 5, tr!("q: quit"), COLOR_TEXT);

            l = 15;
            self.write_at(self.arena_width + 3, l, tr!("Legend:"), COLOR_TEXT);
            self.write_at(self.arena_width + 3, l + 2, tr!("@; you"), COLOR_TEXT);
            self.write_at(self.arena_width + 3, l + 3, tr!("+ #: robot"), COLOR_TEXT);
            self.write_at(self.arena_width + 3, l + 4, tr!("*: heap"), COLOR_TEXT);
        }
    }

    fn draw_borders(
        &mut self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        color: &dyn color::Color,
    ) {
        let nwcorner;
        let necorner;
        let swcorner;
        let secorner;
        let hwall;
        let vwall;
        if self.asciionly {
            nwcorner = "+";
            necorner = "+";
            swcorner = "+";
            secorner = "+";
            hwall = "-";
            vwall = "|";
        } else {
            nwcorner = "╔";
            necorner = "╗";
            swcorner = "╚";
            secorner = "╝";
            hwall = "═";
            vwall = "║";
        }
        self.write_at(x, y, nwcorner, color);
        self.write_at(x + width - 1, y, necorner, color);
        self.write_at(x, y + height - 1, swcorner, color);
        self.write_at(x + width - 1, y + height - 1, secorner, color);
        for x1 in 1..width - 1 {
            self.write_at(x + x1, y, hwall, color);
            self.write_at(x + x1, y + height - 1, hwall, color);
        }
        for y1 in 1..height - 1 {
            self.write_at(x, y + y1, vwall, color);
            self.write_at(x + width - 1, y + y1, vwall, color);
        }

        self.stdout.flush().unwrap();
    }

    fn draw_status(&mut self) {
        let level = format!("{}: {}", tr!("Level"), self.level);
        let score = format!("{}: {}", tr!("Score"), self.score);
        let safes = format!("{}: {}", tr!("Safe teleports"), self.safeteleports);

        match self.boardtype {
            BoardType::Normal => {
                let status_len = self.board_width - 2;

                let level_len = level.chars().count();
                let score_len = score.chars().count();

                let score_padding = status_len / 2 - level_len + score_len / 2;
                let safes_padding = status_len - score_padding - level_len;

                let status_line = format!(
                    " {}{:>score_padding$}{:>safes_padding$} ",
                    level,
                    score,
                    safes,
                    score_padding = score_padding,
                    safes_padding = safes_padding
                );

                self.write_at(0, self.board_height - 1, &status_line, COLOR_TEXT);
            }
            BoardType::Bsd => {
                self.write_at(
                    self.arena_width + 3,
                    self.board_height - 3,
                    &level,
                    COLOR_TEXT,
                );
                self.write_at(
                    self.arena_width + 3,
                    self.board_height - 2,
                    &score,
                    COLOR_TEXT,
                );
                self.write_at(
                    self.arena_width + 3,
                    self.board_height - 1,
                    &safes,
                    COLOR_TEXT,
                );
            }
        }

        self.stdout.flush().unwrap();
    }

    //fn draw_arena(&mut self, arena: &Vec<ObjectType>) {
    fn draw_arena(&mut self, arena: &[ObjectType]) {
        for x in 0..self.arena_width {
            for y in 0..self.arena_height {
                let c = match arena[x + y * self.arena_width] {
                    ObjectType::None => " ",
                    ObjectType::Player => "@",
                    ObjectType::Heap => "*",
                    ObjectType::Robot1 => "+",
                    ObjectType::Robot2 => "#",
                };
                let color: &dyn color::Color = match arena[x + y * self.arena_width] {
                    ObjectType::None => &color::White, // do not show
                    ObjectType::Player => COLOR_PLAYER,
                    ObjectType::Heap => COLOR_HEAP,
                    ObjectType::Robot1 => COLOR_ROBOT1,
                    ObjectType::Robot2 => COLOR_ROBOT2,
                };

                self.write_at(x + 1, y + 1, c, color);
            }
        }

        self.stdout.flush().unwrap();
    }

    pub fn get_arena_size(&self) -> (usize, usize) {
        (self.arena_width, self.arena_height)
    }

    pub fn set_score(&mut self, score: u64) {
        self.score = score;
        self.draw_status();
    }

    pub fn set_level(&mut self, level: u16) {
        self.level = level + 1;
        self.draw_status();
    }

    pub fn set_safeteleports(&mut self, safeteleports: u16) {
        self.safeteleports = safeteleports;
        self.draw_status();
    }

    pub fn update(&mut self, arena: &[ObjectType]) {
        self.draw_arena(arena);
        self.draw_status();

        self.stdout.flush().unwrap();
    }

    fn write_at(&mut self, x: usize, y: usize, line: &str, color: &dyn color::Color) {
        let x = (x + self.delta_x + 1) as u16;
        let y = (y + self.delta_y + 1) as u16;
        if self.colors {
            write!(
                self.stdout,
                "{}{}{}{}",
                cursor::Goto(x, y),
                color::Fg(color),
                line,
                color::Fg(color::Reset)
            )
            .unwrap();
        } else {
            write!(self.stdout, "{}{}", cursor::Goto(x, y), line).unwrap();
        }
    }

    pub fn show_player_message(&mut self, mut x: usize, mut y: usize, message: &str) {
        // adapt coords
        x += 1;
        y += 1;

        let message_len = message.chars().count();

        if y == 1 {
            y += 1;
        } else {
            y -= 1;
        }

        if x + message_len > self.arena_width {
            x -= x + message_len - self.arena_width;
        }

        self.write_at(x, y, message, COLOR_TEXT);
        self.stdout.flush().unwrap();
    }

    fn draw_dialog(&mut self, message: &str, center: bool) {
        let mut width = 0;
        let mut height = 0;
        for line in message.lines() {
            width = max(width, line.chars().count());
            height += 1;
        }
        width += 2;
        height += 2;
        let x = (self.arena_width - width) / 2 + 1;
        let y = (self.arena_height - height) / 2 + 1;

        self.draw_borders(x, y, width, height, COLOR_TEXT);

        let mut y1 = y + 1;
        for line in message.lines() {
            let s;
            if center {
                s = format!("{: ^width$}", line, width = width - 2);
            } else {
                s = format!("{: <width$}", line, width = width - 2);
            }
            self.write_at(x + 1, y1, s.as_str(), COLOR_TEXT);
            y1 += 1;
        }

        self.stdout.flush().unwrap();
    }

    pub fn show_confirmation_dialog(&mut self, message: &str) -> bool {
        self.draw_dialog(message, true);

        let stdin = stdin();

        for c in stdin.keys() {
            if let Key::Char(c) = c.unwrap() {
                match c.to_ascii_lowercase() {
                    'y' => return true,
                    'q' | 'n' => return false,
                    _ => {}
                }
            }
        }

        false
    }

    pub fn show_dialog(&mut self, message: &str) {
        self.draw_dialog(message, false);

        let stdin = stdin();

        // wait for a key press
        stdin.keys().next();
    }

    /*
    pub fn to_mainscreen(&mut self) {
        write!(self.stdout, "{}", screen::ToMainScreen).unwrap();
    }

    pub fn to_alternatescreen(&mut self) {
        write!(self.stdout, "{}", screen::ToAlternateScreen).unwrap();
    }
    */
}

const COLOR_WALL: &'static dyn color::Color = &color::Blue;
const COLOR_TEXT: &'static dyn color::Color = &color::White;
const COLOR_PLAYER: &'static dyn color::Color = &color::Green;
const COLOR_HEAP: &'static dyn color::Color = &color::Yellow;
const COLOR_ROBOT1: &'static dyn color::Color = &color::LightWhite;
const COLOR_ROBOT2: &'static dyn color::Color = &color::LightWhite;
