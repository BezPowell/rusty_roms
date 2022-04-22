use std::{
    fs,
    io::{self, Read},
};

use super::dat::{Datafile, Rom};

pub fn read_file(src: &str) -> io::Result<String> {
    let mut file = std::fs::File::open(src).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

pub fn check_files<'a>(dir: &'a str, dat: &'a Datafile) -> io::Result<Vec<Match<'a>>> {
    let mut matches = Vec::new();
    let paths = fs::read_dir(dir)?;

    for path in paths {
        if let Some(path) = path?.path().to_str() {
            let file = read_file(path)?;
            matches.push(Match {
                file: path.to_string(),
                rom: dat.check_file(&file),
            });
        };
    }

    Ok(matches)
}

pub struct Match<'a> {
    file: String,
    rom: Option<&'a Rom>,
}

impl<'a> Match<'a> {
    pub fn file(&self) -> &str {
        &self.file
    }

    pub fn rom(&self) -> &Option<&'a Rom> {
        &self.rom
    }
}
