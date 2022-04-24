use std::io::Read;

use crate::lib::rom::Rom;
use serde_derive::{Deserialize, Serialize};
use serde_xml_rs::{from_str, Error};

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
    pub fn from_file(src: &str) -> Result<Datafile, Error> {
        // Open file and read contents.
        let mut file = std::fs::File::open(src)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let dat: Datafile = from_str(&contents)?;

        Ok(dat)
    }

    pub fn check_file(&self, file_contents: &Vec<u8>) -> Option<&Rom> {
        for game in &self.games {
            if let Some(rom) = game.rom.check_file(file_contents) {
                return Some(rom);
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::Datafile;

    #[test]
    fn can_parse_dat() {
        let dat = Datafile::from_file("test/dats/megadrive.dat").unwrap();

        assert_eq!(dat.games.len(), 2);
    }
}
