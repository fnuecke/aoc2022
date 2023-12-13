use std::fmt;
use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn read_non_empty_lines<P>(filename: P) -> io::Result<impl Iterator<Item = String>>
where
    P: AsRef<Path>,
{
    Ok(read_lines(filename)?
        .filter_map(|line| line.ok())
        .filter(|line| !line.is_empty())
        .into_iter())
}

#[derive(Clone, Debug)]
pub struct ParseError {
    description: String,
}

impl ParseError {
    pub fn new(description: String) -> Self {
        Self { description }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description)
    }
}

impl std::error::Error for ParseError {}
