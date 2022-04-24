use std::{
    collections::HashMap,
    fs,
    io::{self, Read},
};

use crate::lib::rom::Rom;
use serde_derive::{Deserialize, Serialize};
use serde_xml_rs::{from_str, Error};

use super::{files::File, rom};

#[derive(Debug, Deserialize, PartialEq)]
pub struct Datafile {
    #[serde(rename = "game", default)]
    games: Vec<Game>,
}

#[derive(Debug, Deserialize, PartialEq)]
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

    pub fn check_directory(&self, dir: &str) -> io::Result<Vec<(String, String)>> {
        let mut files = Vec::new();
        let paths = fs::read_dir(dir)?;

        for path in paths {
            if let Some(path) = path?.path().to_str() {
                files.push(File::new(path)?);
            }
        }

        Ok(self.check_files(files))
    }

    fn check_files(&self, files: Vec<File>) -> Vec<(String, String)> {
        let mut matches = Vec::new();
        let mut roms = HashMap::new();

        // Build Hashmap of ROMs.
        for game in &self.games {
            roms.insert(game.rom().hash(), game.rom());
        }

        // Check files against roms
        for file in files {
            if let Some(rom) = roms.get(file.hash()) {
                matches.push((file.path().to_string(), rom.name().to_string()))
            }
        }

        matches
    }
}

impl Game {
    pub fn rom(&self) -> &Rom {
        &self.rom
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

    #[test]
    fn can_check_files() {
        let dat = Datafile::from_file("test/dats/megadrive.dat").unwrap();
        let matches = dat.check_directory("test/roms/megadrive").unwrap();

        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0], ("test/roms/megadrive/30yearsofnintendont.bin".to_string(), "30 Years Of Nintendon't.bin".to_string()));
    }
}
