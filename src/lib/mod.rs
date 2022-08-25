use self::input::*;
use self::result::ResultSet;
use self::verify::hash::Checksum;
use std::str::FromStr;
use std::{collections::HashMap, error::Error};

mod input;
mod result;
mod verify;

/// A struct to handle core app logic.
pub struct App {
    dat: Datafile,
    files: Vec<File>,
}

impl App {
    /// Builds a new app
    ///
    /// # Arguments
    ///
    /// * `dat` - Path to a datafile.
    /// * `files` - Path to a directory containing files to be tested.
    pub fn new(dat: &str, files: &str) -> Result<App, Box<dyn Error>> {
        let dat = Datafile::from_file(dat)?;
        let files = File::read_dir(files)?;

        Ok(App { dat, files })
    }

    /// Verifies rom files against the datafile.
    ///
    pub fn verify(&self) -> ResultSet {
        let test_set = self.dat.build_test_set();
        let mut results = ResultSet::new();

        // Verify roms
        for file in &self.files {
            match test_set.get(file.hash()) {
                Some((rom, game)) => results.add_match(file, game, rom),
                None => results.add_nonmatch(file),
            }
        }

        results
    }
}
