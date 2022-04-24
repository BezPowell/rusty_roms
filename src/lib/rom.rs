use crypto::digest::Digest;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Rom {
    name: String,
    size: u32,
    sha1: String,
    status: Option<String>,
}

impl Rom {
    pub fn check_file(&self, src: &Vec<u8>) -> Option<&Rom> {
        // Check file hash
        let mut hasher1 = crypto::sha1::Sha1::new();
        hasher1.input(src);
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

#[cfg(test)]
mod tests {
    use crate::lib::files::read_file;

    use super::Rom;

    #[test]
    fn can_verify_rom() {
        let rom1 = Rom {
            name: "30 Years Of Nintendon't.bin".to_string(),
            size: 0,
            sha1: "f1cd840f271d3197d9f6706795898a880c81ff83".to_string(),
            status: None,
        };

        let rom2 = Rom {
            name: "Some other game.bin".to_string(),
            size: 0,
            sha1: "f1cd840f271d3197d9f6706795898a880c81ff81".to_string(),
            status: None,
        };
        let file = read_file("test/roms/megadrive/30yearsofnintendont.bin").unwrap();

        assert_eq!(rom1.check_file(&file).unwrap(), &rom1);
        assert_eq!(rom2.check_file(&file), None);
    }

    #[test]
    fn can_verify_nes_rom() {
        let rom = Rom {
            name: "1942 (Japan, USA) (En).nes".to_string(),
            size: 0,
            sha1: "7f57eace7cada7c36412a50f2299231b304527a8".to_string(),
            status: None,
        };
        let file = read_file("test/roms/nes/1942.nes").unwrap();

        assert_eq!(rom.check_file(&file).unwrap(), &rom);
    }
}
