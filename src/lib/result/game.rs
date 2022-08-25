use std::collections::HashMap;

use crate::lib::{
    input::{File, Game, Rom},
    verify::VerifiedStatus,
};

#[derive(Debug)]
pub struct GameResult<'a> {
    roms: HashMap<&'a str, VerifiedStatus<'a>>,
}

impl<'a> GameResult<'a> {
    /// Build a new game result from the specified string.
    pub fn new(game: &'a Game) -> GameResult<'a> {
        let mut roms = HashMap::with_capacity(game.roms().len());
        for rom in game.roms() {
            roms.insert(rom.name(), VerifiedStatus::Missing);
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

        self.roms.insert(rom.name(), status);
    }

    pub fn roms(&self) -> &HashMap<&'a str, VerifiedStatus<'a>> {
        &self.roms
    }
}
