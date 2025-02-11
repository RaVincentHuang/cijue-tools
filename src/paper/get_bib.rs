use std::error::Error;
use std::{path::PathBuf};

use clap::{Command, Arg, arg, command, value_parser, ArgMatches};
use pyo3::types::PyFunction;
use pyo3::PyResult;

use crate::common;

use pyo3::prelude::*;
use pyo3::ffi::c_str;

pub fn get_commands() -> Command {
    command!("bib")
        .about("Get paper bibtex")
        .arg(
            arg!(-u --url <URL> "Paper Urls")
                .required(true)
                .value_parser(value_parser!(String))
        )
        .arg(
            arg!(-o --output [OUTPUT] "Output file")
                .value_parser(value_parser!(PathBuf))
        )
        .arg(
            arg!(-d --database "From database")
                .value_parser(["dblp"])
                .default_value("dblp")
        )
}

pub fn bib_main(matchs: &ArgMatches) -> Result<(), Box<dyn Error>> {
    
    let url = matchs.get_one::<String>("url").ok_or("URL argument is required")?;
    let out = matchs.get_one::<PathBuf>("output").map_or(common::output::Output::Console, |path| common::output::Output::File(path.clone()));

    match matchs.get_one::<String>("database").ok_or("no database")?.as_str() {
        "dblp" => get_bib_from_dblp(url, &out)?,
        _ => unreachable!(),
    }

    Ok(())
}

fn get_bib_from_dblp(url: &str, out: &common::output::Output) -> PyResult<()> {
    Python::with_gil(|py| -> PyResult<()> {
        
        let path = c_str!(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/paper/get_paper_bib.py")));
        
        let module = PyModule::from_code(py, path, c_str!("get_paper_bib.py"), c_str!("get_paper_bib"))?;
        let fun: Py<PyAny> = module.getattr("get_paper_bib_from_dblp")?.into();

        let res: String = fun.call1(py, (url,))?.extract(py)?;

        if let common::output::Output::File(path) = out {
            std::fs::write(path, res)?;
        } else {
            println!("{}", res);
        }
        
        Ok(())
    })
}