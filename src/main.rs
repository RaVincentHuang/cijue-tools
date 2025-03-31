use pyo3::prelude::*;
use pyo3::ffi::c_str;
use colored::Colorize;


pub mod paper;
pub mod formats;
pub mod common;

fn get_commands() -> clap::Command {
    clap::command!("cijue")
        .about("Cijue Tools")
        .subcommand(
            paper::get_commands(),
        )
        .subcommand(
            formats::get_commands(),
        )
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = get_commands().get_matches();

    match matches.subcommand() {
        Some(("paper", sub_m)) => paper::paper_main(sub_m)?,
        Some(("formats", sub_m)) => formats::paper_main(sub_m)?,
        _ => println!("{}", "No subcommand was used!".red()),
    }

    Ok(())
    
}
