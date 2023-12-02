use std::fs::File;
use std::io::Read;

use anyhow::Result;

pub mod day01;
pub mod day02;
mod template;

pub fn read_file_to_string(filename: &str) -> Result<String> {
    let mut f = File::open(filename)?;
    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;
    Ok(buffer)
}
