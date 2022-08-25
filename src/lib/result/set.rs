use crate::lib::input::{File, Game, Rom};
use crate::lib::verify::VerifiedStatus;
use std::collections::HashMap;

#[derive(Debug)]
pub struct ResultSet<'a> {
    matched: HashMap<&'a str, GameResult<'a>>,
    unmatched: Vec<&'a File>,
}

impl<'a> ResultSet<'a> {
    pub fn new() -> ResultSet<'a> {
        ResultSet {
            matched: HashMap::new(),
            unmatched: Vec::new(),
        }
    }

    pub fn matches(&self) -> &HashMap<&str, GameResult<'a>> {
        &self.matched
    }

    pub fn nonmatches(&self) -> &Vec<&'a File> {
        &self.unmatched
    }

    pub fn add_match(&mut self, file: &'a File, game: &'a Game, rom: &'a Rom) {
        // Create new entry if none exists.
        if !self.matched.contains_key(game.name()) {
            self.matched.insert(game.name(), GameResult::new(game));
        }

        // Update existing entry
        if let Some(game) = self.matched.get_mut(game.name()) {
            game.add_file(file, rom);
        }
    }

    pub fn add_nonmatch(&mut self, file: &'a File) {
        self.unmatched.push(file);
    }
}

#[derive(Debug)]
pub struct GameResult<'a> {
    roms: HashMap<String, VerifiedStatus<'a>>,
}

impl<'a> GameResult<'a> {
    /// Build a new game result from the specified string.
    pub fn new(game: &'a Game) -> GameResult<'a> {
        let mut roms = HashMap::with_capacity(game.roms().len());
        for rom in game.roms() {
            roms.insert(rom.name().to_string(), VerifiedStatus::Missing);
        }

        GameResult { roms }
    }

    /// Add a file to the given
    pub fn add_file(&mut self, file: &'a File, rom: &'a Rom) {
        let status = if file.name().unwrap() == rom.name() {
            VerifiedStatus::Verified { file }
        } else {
            VerifiedStatus::MatchNotName {
                file: file,
                correct_name: rom.name(),
            }
        };

        self.roms.insert(rom.name().to_string(), status);
    }

    pub fn roms(&self) -> &HashMap<String, VerifiedStatus<'a>> {
        &self.roms
    }
}
