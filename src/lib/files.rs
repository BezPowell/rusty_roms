use crypto::digest::Digest;
use std::{
    io::{self, Read},
    path::{Path, PathBuf},
};

const NES_HEADER_SIZE: usize = 16;

pub struct File {
    path: PathBuf,
    hash: String,
}

impl File {
    pub fn new(src: &str) -> io::Result<File> {
        // Open file and read contents.
        let mut file = std::fs::File::open(src).unwrap();
        let mut contents = Vec::new();
        file.read_to_end(&mut contents)?;

        // Look for iNES header.
        if String::from_utf8_lossy(&contents[..=NES_HEADER_SIZE]).starts_with("NES") {
            // If found, use contents minus header.
            contents = contents[NES_HEADER_SIZE..].to_vec();
        }

        // Hash contents
        let mut hasher1 = crypto::sha1::Sha1::new();
        hasher1.input(&contents);
        let hash = hasher1.result_str();

        Ok(File {
            path: PathBuf::from(src),
            hash,
        })
    }

    pub fn path(&self) -> &Path {
        &self.path.as_path()
    }

    pub fn hash(&self) -> &str {
        &self.hash
    }
}

#[cfg(test)]
mod tests {
    use crate::lib::files::File;

    #[test]
    fn can_read_rom() {
        let rom = File::new("test/roms/megadrive/30yearsofnintendont.bin").unwrap();
        assert_eq!(rom.hash(), "f1cd840f271d3197d9f6706795898a880c81ff83");
    }

    #[test]
    fn can_read_nes_rom() {
        let rom = File::new("test/roms/nes/1942.nes").unwrap();
        assert_eq!(rom.hash(), "7f57eace7cada7c36412a50f2299231b304527a8");
    }
}
