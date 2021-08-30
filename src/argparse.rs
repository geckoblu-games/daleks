use argparse::{ArgumentParser, StoreFalse, StoreOption, StoreTrue};

use crate::options;
use crate::profiles;

pub fn parse_args(options: &mut options::Options) {
    let mut profile: std::option::Option<profiles::Profiles> = None;
    let mut default = false;

    // this block limits scope of borrows by parser.refer() method
    {
        let mut parser = ArgumentParser::new();
        parser.set_description(tr!("Escape from evil robots who want to exterminate you."));

        parser
            .refer(&mut options.safe_moves)
            .add_option(
                &["--safe-moves"],
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
                &["--profile"],
                StoreOption,
                tr!("Set the game profile (CLASSIC, ROBOTS2, NIGHTMARE, ROBOTS2EASY, CLASSICWITHSAFETELEPORTS)"));

        parser
            .refer(&mut options.colors)
            .add_option(&["--colors"], StoreTrue, tr!("Enable terminal colors"))
            .add_option(&["--no-colors"], StoreFalse, tr!("Disable terminal colors"));

        parser
            .refer(&mut options.asciionly)
            .add_option(
                &["--asciionly"],
                StoreTrue,
                tr!("Use only ascii characters"),
            )
            .add_option(
                &["--no-asciionly"],
                StoreFalse,
                tr!("Use extended unicode characters"),
            );

        parser.refer(&mut default).add_option(
            &["--defaults"],
            StoreTrue,
            tr!("Restore default values"),
        );

        parser.parse_args_or_exit();
    }

    if let Some(profile) = profile {
        options.set_profile(profile);
    }

    if default {
        *options = options::Options::default();
    }
}
