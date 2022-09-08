use super::rom::Rom;
use serde_derive::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Game {
    name: String,
    #[serde(rename = "rom", default)]
    roms: Vec<Rom>,
}

impl Game {
    pub fn roms(&self) -> &Vec<Rom> {
        &self.roms
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
