
use std::path::PathBuf;

use clap::{Command, Arg, arg, command, value_parser};

pub mod get_bib;

pub fn get_commands() -> Command {
    command!("paper")
        .about("Paper related commands")
        .subcommand(
            get_bib::get_commands()
        )
}

pub fn paper_main(matchs: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    match matchs.subcommand() {
        Some(("bib", sub_m)) => get_bib::bib_main(sub_m)?,
        _ => unreachable!(),
    }
    Ok(())
}