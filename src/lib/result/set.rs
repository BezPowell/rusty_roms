use super::GameResult;
use crate::lib::input::{File, Game, Rom};
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
