mod game;
mod rom;

use std::io::Read;

pub use game::Game;
pub use rom::Rom;
use serde_derive::Deserialize;
use serde_xml_rs::{from_str, Error};

/// Contains a list of games,
/// and the checksums for their roms.
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

    pub fn games(&self) -> &Vec<Game> {
        &self.games
    }
}

#[cfg(test)]
mod tests {
    use super::Datafile;

    #[test]
    fn can_parse_dat() {
        let dat = Datafile::from_file("test/dats/megadrive.dat").unwrap();

        assert_eq!(dat.games.len(), 3);
    }
}
