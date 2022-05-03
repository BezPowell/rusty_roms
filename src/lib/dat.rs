use std::io::Read;

use crate::lib::{game::Game, rom::Rom};
use serde_derive::Deserialize;
use serde_xml_rs::{from_str, Error};

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

    pub fn roms(self) -> Vec<Rom> {
        let mut roms = Vec::new();

        for game in self.games {
            for rom in game.rom() {
                roms.push(rom);
            }
        }

        roms
    }
}

#[cfg(test)]
mod tests {
    use super::Datafile;

    #[test]
    fn can_parse_dat() {
        let dat = Datafile::from_file("test/dats/megadrive.dat").unwrap();

        assert_eq!(dat.roms().len(), 3);
    }
}
