use crate::lib::dat::Datafile;
use crate::lib::rom::Rom;
use std::{
    fs,
    io::{self, Read},
};

pub fn read_file(src: &str) -> io::Result<Vec<u8>> {
    // Open file and read contents.
    let mut file = std::fs::File::open(src).unwrap();
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;

    // Look for iNES header.
    if String::from_utf8_lossy(&contents[..=16]).starts_with("NES") {
        // If found, return contents minus header.
        contents = contents[16..].to_vec();
    }

    Ok(contents)
}

pub fn check_files<'a>(dir: &'a str, dat: &'a Datafile) -> io::Result<Vec<Match<'a>>> {
    let mut matches = Vec::new();
    let paths = fs::read_dir(dir)?;

    for path in paths {
        if let Some(path) = path?.path().to_str() {
            let file = read_file(path)?;
            matches.push(Match {
                file: path.to_string(),
                rom: dat.check_file(&file),
            });
        };
    }

    Ok(matches)
}

pub struct Match<'a> {
    file: String,
    rom: Option<&'a Rom>,
}

impl<'a> Match<'a> {
    pub fn file(&self) -> &str {
        &self.file
    }

    pub fn rom(&self) -> &Option<&'a Rom> {
        &self.rom
    }
}

#[cfg(test)]
mod tests {
    use crate::lib::{dat::Datafile, files::read_file};

    use super::{check_files, Rom};

    #[test]
    fn can_read_rom() {
        let rom = "test/roms/megadrive/30yearsofnintendont.bin";
        read_file(rom);
    }

    #[test]
    fn can_check_dir() {
        let romdir = "test/roms/megadrive/";
        let dat = Datafile::from_file("test/dats/megadrive.dat").unwrap();
        let matches = check_files(romdir, &dat).unwrap();

        assert_eq!(matches.len(), 1);
        assert_eq!(
            matches[0].file(),
            "test/roms/megadrive/30yearsofnintendont.bin"
        );
        assert_ne!(matches[0].rom(), &None);
    }
}
