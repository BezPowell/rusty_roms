use crypto::digest::Digest;
use serde;
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

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Rom {
    name: String,
    size: u32,
    sha1: String,
    status: Option<String>,
}

impl Datafile {
    pub fn check_file(&self, file_contents: &str) -> Option<&Rom> {
        for game in &self.games {
            if let Some(rom) = game.rom.check_file(file_contents) {
                return Some(rom);
            }
        }

        None
    }
}

impl Rom {
    pub fn check_file(&self, src: &str) -> Option<&Rom> {
        // Check file hash
        let mut hasher1 = crypto::sha1::Sha1::new();
        hasher1.input_str(src);
        let digest = hasher1.result_str();

        if digest == self.sha1 {
            Some(&self)
        } else {
            None
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
