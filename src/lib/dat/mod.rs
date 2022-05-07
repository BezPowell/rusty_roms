mod game;
mod rom;

use std::{collections::HashMap, io::Read, str::FromStr};

use game::Game;
use serde_derive::Deserialize;
use serde_xml_rs::{from_str, Error};

use super::verify::hash::Checksum;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Datafile {
    #[serde(rename = "game", default)]
    games: Vec<Game>,
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

    pub fn roms(self) -> HashMap<Checksum, String> {
        let mut roms = HashMap::with_capacity(self.games.len());
        // Convert Vec into HashMap with Rom digest as index.
        for game in self.games {
            for rom in game.roms() {
                roms.insert(
                    Checksum::from_str(rom.hash()).unwrap(),
                    rom.name().to_string(),
                );
            }
        }

        roms
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::lib::verify::hash::Checksum;

    use super::Datafile;

    #[test]
    fn can_parse_dat() {
        let dat = Datafile::from_file("test/dats/megadrive.dat").unwrap();

        assert_eq!(dat.roms().len(), 3);
    }

    #[test]
    fn can_index_roms() {
        let dat = Datafile::from_file("test/dats/megadrive.dat").unwrap();
        let roms = dat.roms();

        assert_eq!(
            roms.get(&Checksum::from_str("f1cd840f271d3197d9f6706795898a880c81ff83").unwrap())
                .unwrap(),
            "30 Years Of Nintendon't.bin"
        );
    }
}
