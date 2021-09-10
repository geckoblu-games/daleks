use argparse::{ArgumentParser, StoreFalse, StoreOption, StoreTrue};
use std::process;

use crate::options;
use crate::profiles;

pub fn parse_args(options: &mut options::Options) {
    let mut boardtype: std::option::Option<options::BoardType> = None;
    let mut profile: std::option::Option<profiles::Profiles> = None;
    let mut default = false;
    let mut save = false;
    let mut exterminate = false;

    // this block limits scope of borrows by parser.refer() method
    {
        let mut parser = ArgumentParser::new();
        parser.set_description(tr!("Escape from evil robots who want to exterminate you."));

        parser
            .refer(&mut options.safe_moves)
            .add_option(
                &["-s", "--safe-moves"],
                StoreTrue,
                tr!("Prevent accidental moves that result in getting killed"),
            )
            .add_option(
                &["--no-safe-moves"],
                StoreFalse,
                tr!("Don't prevent accidental moves that result in getting killed"),
            );

        parser
            .refer(&mut profile)
            .add_option(
                &["-p", "--profile"],
                StoreOption,
                tr!("Set the game profile (CLASSIC, ROBOTS2, NIGHTMARE, ROBOTS2EASY, CLASSICWITHSAFETELEPORTS)"));

        parser
            .refer(&mut options.colors)
            .add_option(
                &["-c", "--colors"],
                StoreTrue,
                tr!("Enable terminal colors"),
            )
            .add_option(&["--no-colors"], StoreFalse, tr!("Disable terminal colors"));

        parser
            .refer(&mut options.asciionly)
            .add_option(
                &["-a", "--asciionly"],
                StoreTrue,
                tr!("Use only ascii characters"),
            )
            .add_option(
                &["--no-asciionly"],
                StoreFalse,
                tr!("Use extended unicode characters"),
            );

        parser.refer(&mut boardtype).add_option(
            &["-b", "--boardtype"],
            StoreOption,
            tr!("Set the board layout (NORMAL, BSD)"),
        );

        parser.refer(&mut exterminate).add_option(
            &["-x", "--exterminate"],
            StoreTrue,
            tr!("Use at your own risk"),
        );

        parser.refer(&mut default).add_option(
            &["--defaults"],
            StoreTrue,
            tr!("Restore default values"),
        );
        parser.refer(&mut save).add_option(
            &["--save-conf"],
            StoreTrue,
            tr!("Save current configuration"),
        );

        parser.parse_args_or_exit();
    }

    if exterminate {
        println!("{}", EXTERMINATE);
        process::exit(1);
    }

    if let Some(boardtype) = boardtype {
        options.boardtype = boardtype;
    }

    if let Some(profile) = profile {
        options.set_profile(profile);
    }

    if default {
        *options = options::Options::default();
    }

    if save {
        options.store();
    }
}

const EXTERMINATE: &str = "
              EXTERMINATE!
                       \\
                     _,------,_
          |)=O=====-'     O    '-
                  /==============\\
                  |===============|
                  |__|_|_|_|_|_|_||
    )=============|__|_|_|_|_|_|_||
                 /________________|
               (/() / ()| () | () |
              (/() / () / () | () |
             (/() / ()  | () | () |
            (/() / ()   / () | () |
           (/() / ()   | ()  / () |
          (/() / ()    / () | ()  |
         (/() / ()    | ()  | ()  |
        (/() / ()    /  ()  | ()  |
       (/() / ()    |  ()   | ()  |
      /===========================|
";
