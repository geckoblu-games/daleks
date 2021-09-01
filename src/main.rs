#[macro_use]
mod internationalization;

mod argparse;
mod board;
mod enums;
mod game;
mod options;
mod profiles;

fn main() {
    let mut options = options::Options::load();

    argparse::parse_args(&mut options);

    let mut game = game::Game::new(&options);

    game.new_game();

    game.run();
}
