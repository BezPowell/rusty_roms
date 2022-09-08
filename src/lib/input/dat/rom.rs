use serde_derive::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Rom {
    name: String,
    sha1: String,
}

impl Rom {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn hash(&self) -> &str {
        &self.sha1
    }
}
