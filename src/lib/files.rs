use crypto::digest::Digest;
use std::{
    fs,
    io::{self, Read},
    path::{Path, PathBuf},
};

const NES_HEADER_SIZE: usize = 16;

#[derive(Debug, PartialEq)]
pub struct File {
    path: PathBuf,
    hash: String,
}

impl File {
    pub fn read_dir(path: &str) -> io::Result<Vec<File>> {
        let mut files = Vec::new();

        // Read all entries in directory, skipping other dirs.
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                files.push(File::new(&path)?);
            }
        }

        Ok(files)
    }

    pub fn new(src: &PathBuf) -> io::Result<File> {
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

    pub fn name(&self) -> Option<&str> {
        self.path().file_name()?.to_str()
    }
}

#[cfg(test)]
mod tests {
    use std::{fs, path::PathBuf};

    use crate::lib::files::File;

    #[test]
    fn can_read_rom() {
        let rom = File::new(&PathBuf::from(
            "test/roms/megadrive/30yearsofnintendont.bin",
        ))
        .unwrap();
        assert_eq!(rom.hash(), "f1cd840f271d3197d9f6706795898a880c81ff83");
    }

    #[test]
    fn can_read_nes_rom() {
        let rom = File::new(&PathBuf::from("test/roms/nes/1942.nes")).unwrap();
        assert_eq!(rom.hash(), "7f57eace7cada7c36412a50f2299231b304527a8");
    }

    #[test]
    fn can_read_dir() {
        // Create tmp directory to test error checking.
        fs::create_dir_all("test/roms/megadrive/tmp").unwrap();
        let roms = File::read_dir(&"test/roms/megadrive/").unwrap();

        assert_eq!(roms.len(), 3);

        assert!(roms.contains(&File {
            path: PathBuf::from("test/roms/megadrive/30yearsofnintendont.bin",),
            hash: "f1cd840f271d3197d9f6706795898a880c81ff83".to_string()
        }));

        // Clean up tmp directory
        fs::remove_dir_all("test/roms/megadrive/tmp").unwrap();
    }
}
