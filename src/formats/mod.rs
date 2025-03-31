
use std::path::PathBuf;

use clap::{Command, Arg, arg, command, value_parser};

pub mod json;

pub fn get_commands() -> Command {
    command!("formats")
        .about("Format tools")
        .subcommand(
            json::get_commands()
        )
}

pub fn paper_main(matchs: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    match matchs.subcommand() {
        Some(("json", sub_m)) => json::json_main(sub_m)?,
        _ => unreachable!(),
    }
    Ok(())
}
