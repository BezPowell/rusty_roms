use std::collections::HashMap;

use super::{dat::Datafile, files::File, rom::Rom, verify::VerifiedStatus};

pub struct CheckedSet {
    verified: usize,
    notname: usize,
    unverified: usize,
    results: Vec<File>,
}

impl CheckedSet {
    pub fn new(dat: Datafile, mut files: Vec<File>) -> CheckedSet {
        // Initialise counts
        let mut verified = 0;
        let mut notname = 0;
        let mut unverified = 0;

        // Build hashmap from dat
        let roms = CheckedSet::indexed_roms(dat.roms());

        // Check files against roms
        for file in &mut files {
            if let Some(rom) = roms.get(file.hash()) {
                if file.path().file_name().unwrap() == rom.name() {
                    // Exact match
                    file.set_status(VerifiedStatus::Verified);
                    verified += 1;
                } else {
                    // Contents match, but not filename
                    file.set_status(VerifiedStatus::MatchNotName {
                        name: rom.name().to_string(),
                    });
                    notname += 1;
                }
            } else {
                // No match found
                file.set_status(VerifiedStatus::Unverified);
                unverified += 1;
            }
        }

        CheckedSet {
            verified,
            notname,
            unverified,
            results: files,
        }
    }

    pub fn results(&self) -> &Vec<File> {
        &self.results
    }

    pub fn counts(&self) -> (usize, usize, usize) {
        (self.verified, self.notname, self.unverified)
    }

    pub fn len(&self) -> usize {
        self.results.len()
    }

    fn indexed_roms(roms: Vec<Rom>) -> HashMap<String, Rom> {
        let mut set = HashMap::new();
        // Convert Vec into HashMap with Rom digest as index.
        for rom in roms {
            set.insert(rom.hash().to_string(), rom);
        }

        set
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::lib::{dat::Datafile, files::File, verify::VerifiedStatus};

    use super::CheckedSet;

    #[test]
    fn can_test_roms() {
        let dat = Datafile::from_file("test/dats/megadrive.dat").unwrap();
        let files = File::read_dir("test/roms/megadrive").unwrap();
        let set = CheckedSet::new(dat, files);

        // Check number of Files
        assert_eq!(set.results().len(), 3);

        let exact_match = set
            .results()
            .iter()
            .find(|item| {
                item.path().file_name().unwrap().to_str().unwrap() == "30 Years Of Nintendon't.bin"
            })
            .unwrap();

        let partial_match = set
            .results()
            .iter()
            .find(|item| {
                item.path().file_name().unwrap().to_str().unwrap() == "30yearsofnintendont.bin"
            })
            .unwrap();

        let no_match = set
            .results()
            .iter()
            .find(|item| item.path().file_name().unwrap().to_str().unwrap() == "failed.bin")
            .unwrap();

        assert_eq!(exact_match.status(), &Some(VerifiedStatus::Verified));
        assert_eq!(
            partial_match.status(),
            &Some(VerifiedStatus::MatchNotName {
                name: "30 Years Of Nintendon't.bin".to_string()
            })
        );
        assert_eq!(no_match.status(), &Some(VerifiedStatus::Unverified));
    }
}
