use std::collections::HashMap;

use crate::lib::{
    input::{File, Game, Rom},
    verify::VerifiedStatus,
};

#[derive(Debug)]
pub struct GameResult<'a> {
    status: GameStatus,
    roms: HashMap<&'a str, Option<VerifiedStatus<'a>>>,
}

impl<'a> GameResult<'a> {
    /// Build a new game result from the specified string.
    pub fn new(game: &'a Game) -> GameResult<'a> {
        let mut roms = HashMap::with_capacity(game.roms().len());
        for rom in game.roms() {
            roms.insert(rom.name(), None);
        }

        GameResult {
            status: GameStatus::Incomplete,
            roms,
        }
    }

    /// Add a file to the given
    pub fn add_file(&mut self, file: &'a File, rom: &'a Rom) {
        let status = if file.name().unwrap() == rom.name() {
            VerifiedStatus::Verified {
                file: file,
                output: rom.name(),
            }
        } else {
            VerifiedStatus::MatchNotName {
                file: file,
                output: rom.name(),
            }
        };

        self.roms.insert(rom.name(), Some(status));

        // If all now found mark status as complete
        if !self.roms.values().any(|x| x.is_none()) {
            self.status = GameStatus::Complete;
        }
    }

    pub fn status(&self) -> GameStatus {
        self.status
    }

    pub fn roms(&self) -> &HashMap<&'a str, Option<VerifiedStatus<'a>>> {
        &self.roms
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameStatus {
    Complete,
    Incomplete,
}
