use crate::lib::rom::Rom;
use serde_derive::{Deserialize, Serialize};
use serde_xml_rs::{from_str, to_string};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Datafile {
    #[serde(rename = "game", default)]
    games: Vec<Game>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Game {
    name: String,
    rom: Rom,
}

impl Datafile {
    pub fn check_file(&self, file_contents: &Vec<u8>) -> Option<&Rom> {
        for game in &self.games {
            if let Some(rom) = game.rom.check_file(file_contents) {
                return Some(rom);
            }
        }

        None
    }
}
