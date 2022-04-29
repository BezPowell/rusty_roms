use serde_derive::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Rom {
    name: String,
    size: u32,
    sha1: String,
    status: Option<String>,
}

impl Rom {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn hash(&self) -> &str {
        &self.sha1
    }
}
