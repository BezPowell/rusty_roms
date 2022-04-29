use crate::lib::rom::Rom;
use serde_derive::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Game {
    name: String,
    #[serde(rename = "rom", default)]
    roms: Vec<Rom>,
}

impl Game {
    pub fn rom(&self) -> &Vec<Rom> {
        &self.roms
    }
}
