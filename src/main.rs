use std::process;

mod argparse;
mod board;
mod enums;
mod game;
mod options;
mod profiles;

fn main() {
    let mut options = options::Options::load();

    argparse::parse_args(&mut options);

    let res = game::Game::new(&options);

    match res {
        Ok(mut game) => {
            game.new_game();
            game.run();
        }
        Err(s) => {
            eprintln!("ERROR: {}", s);

            process::exit(1);
        }
    }
}
