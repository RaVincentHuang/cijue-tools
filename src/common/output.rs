use std::path::PathBuf;

pub enum Output {
    File(PathBuf),
    Console,
}