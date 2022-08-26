use self::input::*;
use self::result::{GameStatus, ResultSet};
use std::error::Error;
use std::path::Path;
use std::{fs, io};

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

    /// Reports the results of the check
    pub fn report(&self, set: &ResultSet) {
        // Display matches
        if set.matches().len() > 0 {
            println!("--- Matched {} games ---", set.matches().len());
            for (name, result) in set.matches() {
                println!("Game {}: {:?}", name, result.status());
                for (rom, status) in result.roms() {
                    let status = match status {
                        Some(status) => match status {
                            verify::VerifiedStatus::Verified { file: _, output: _ } => "Verified.",
                            verify::VerifiedStatus::MatchNotName { file: _, output: _ } => {
                                "Verified, with wrong file name."
                            }
                        },
                        None => "File missing.",
                    };
                    println!("--Rom {}: {}", rom, status);
                }
            }
            println!("");
        }

        // Display non-matches
        if set.nonmatches().len() > 0 {
            println!("--- Could not match {} files ---", set.nonmatches().len());
            for file in set.nonmatches() {
                println!("{:?}", file.path());
            }
        }

        // Display summary
        let complete = set
            .matches()
            .values()
            .filter(|n| n.status() == GameStatus::Complete)
            .count();

        let incomplete = set
            .matches()
            .values()
            .filter(|n| n.status() == GameStatus::Incomplete)
            .count();

        let unmatched = set.nonmatches().len();

        println!("--- Summary ---");
        println!(
            "Found {} complete games, {} incomplete games, and {} unmatched files.",
            complete, incomplete, unmatched
        );
    }

    /// Copy files to specified directory.
    pub fn copy(&self, set: ResultSet, dir: &str) -> io::Result<()> {
        // Create target directory if it doesn't exist.
        let output = Path::new(dir);
        fs::create_dir_all(output)?;

        // Loop through each matched game
        for (name, game) in set.matches() {
            // If game has more than 1 ROM create subdirectory
            let target = if game.roms().len() > 1 {
                let path = output.join(name);
                fs::create_dir_all(path.as_path())?;

                path
            } else {
                output.to_path_buf()
            };

            // Copy all roms
            for (_, rom) in game.roms() {
                if let Some(rom) = rom {
                    let target = Path::new(&target).join(rom.output());
                    fs::copy(rom.file().path(), target).unwrap();
                }
            }
        }

        Ok(())
    }
}
