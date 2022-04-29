use std::{
    collections::HashMap,
    fs,
    io::{self, Read},
};

use crate::lib::{files::File, game::Game};
use serde_derive::Deserialize;
use serde_xml_rs::{from_str, Error};

use super::verify::VerifiedStatus;

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

    pub fn check_directory(&self, dir: &str) -> io::Result<Vec<VerifiedStatus>> {
        let mut files = Vec::new();
        let paths = fs::read_dir(dir)?;

        for path in paths {
            if let Some(path) = path?.path().to_str() {
                files.push(File::new(path)?);
            }
        }

        Ok(self.check_files(files))
    }

    fn check_files(&self, files: Vec<File>) -> Vec<VerifiedStatus> {
        let mut matches = Vec::new();
        let mut roms = HashMap::new();

        // Build Hashmap of ROMs.
        for game in &self.games {
            for rom in game.rom() {
                roms.insert(rom.hash(), rom);
            }
        }

        // Check files against roms
        for file in files {
            let status = match roms.get(file.hash()) {
                Some(rom) => {
                    if file.path().file_name().unwrap() == rom.name() {
                        VerifiedStatus::Verified {
                            file: file.path().to_path_buf(),
                        }
                    } else {
                        VerifiedStatus::MatchNotName {
                            file: file.path().to_path_buf(),
                            rom: rom.name().to_string(),
                        }
                    }
                }
                None => VerifiedStatus::Unverified {
                    file: file.path().to_path_buf(),
                },
            };

            matches.push(status);
        }

        matches
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::lib::verify::VerifiedStatus;

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

        assert_eq!(matches.len(), 3);
        assert!(matches.contains(&VerifiedStatus::Verified {
            file: PathBuf::from("test/roms/megadrive/30 Years Of Nintendon't.bin")
        }));
        assert!(matches.contains(&VerifiedStatus::MatchNotName {
            file: PathBuf::from("test/roms/megadrive/30yearsofnintendont.bin"),
            rom: "30 Years Of Nintendon't.bin".to_string()
        }));
        assert!(matches.contains(&VerifiedStatus::Unverified {
            file: PathBuf::from("test/roms/megadrive/failed.bin")
        }));
    }
}
