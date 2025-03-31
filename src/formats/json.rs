use std::error::Error;
use std::io::{BufRead, Write};
use std::{path::PathBuf};

use clap::{Command, Arg, arg, command, value_parser, ArgMatches, ArgAction};
use serde_json::Value;


pub fn get_commands() -> Command {
    command!("json")
        .about("json tools")
        .subcommand(
            command!("concat")
                .about("Concat jsonl files")
                .arg(
                    arg!(-i --input <INPUT> "Input jsonl files")
                        .required_unless_present_any(["dir"])
                        .action(ArgAction::Append)
                        .value_parser(value_parser!(PathBuf))
                )
                .arg(
                    arg!(-d --dir <DIR> "Input jsonl files directory")
                        .required_unless_present_any(["input"])
                        .action(ArgAction::Set)
                        .value_parser(value_parser!(PathBuf))
                )
                .arg(
                    arg!(-o --output [OUTPUT] "Output jsonl file")
                        .default_value("output.jsonl")
                        .value_parser(value_parser!(PathBuf))
                )
        )
}

pub fn json_main(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let mut file_store = Vec::new();

    match matches.subcommand() {
        Some(("concat", sub_matches)) => {
            // Check if the user provided a directory or input files
            let input_files: Vec<&PathBuf> = if let Some(dir) = sub_matches.get_one::<PathBuf>("dir") {
                for entry in std::fs::read_dir(dir)? {
                    let entry = entry?;
                    if entry.path().extension().map_or(false, |ext| ext == "jsonl") {
                        file_store.push(entry.path());
                    }
                }
                file_store.iter().collect()
            } else {
                sub_matches.get_many::<PathBuf>("input")
                    .ok_or("No input files provided")?
                    .collect()
            };
            let output_file = sub_matches
                .get_one::<PathBuf>("output")
                .expect("need output file");

            concat_jsonl(input_files, output_file)
        }
        _ => Ok(()),
    }
}

fn concat_jsonl(paths: Vec<&PathBuf>, target: &PathBuf) -> Result<(), Box<dyn Error>> {
    
    let mut output_jsonl = Vec::new();
    
    for path in paths {
        let file = std::fs::File::open(path)?;
        let reader = std::io::BufReader::new(file);
        for line in reader.lines() {
            let line = line?;
            let json: Value = serde_json::from_str(&line)?;
            output_jsonl.push(json);
        }
    }

    let output_file = std::fs::File::create(target)?;
    let mut writer = std::io::BufWriter::new(output_file);
    for json in output_jsonl {
        let json_str = serde_json::to_string(&json)?;
        writeln!(writer, "{}", json_str)?;
    }

    Ok(())
}