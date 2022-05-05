use std::collections::HashMap;

use super::{files::File, verify::VerifiedStatus};

pub struct CheckedSet<'a> {
    verified: usize,
    notname: usize,
    unverified: usize,
    results: Vec<(&'a File, VerifiedStatus<'a>)>,
}

impl CheckedSet<'_> {
    pub fn new<'a>(roms: &'a HashMap<String, String>, files: &'a Vec<File>) -> CheckedSet<'a> {
        // Initialise counts
        let mut verified = 0;
        let mut notname = 0;
        let mut unverified = 0;
        let mut results = Vec::with_capacity(files.len());

        // Check files against roms
        for file in files {
            let status = if let Some(rom) = roms.get(file.hash()) {
                if &file.name().unwrap() == &rom {
                    // Exact match
                    verified += 1;
                    VerifiedStatus::Verified { name: rom }
                } else {
                    // Contents match, but not filename
                    notname += 1;
                    VerifiedStatus::MatchNotName { name: rom }
                }
            } else {
                // No match found
                unverified += 1;
                VerifiedStatus::Unverified
            };

            results.push((file, status));
        }

        CheckedSet {
            verified,
            notname,
            unverified,
            results,
        }
    }

    pub fn results(&self) -> &Vec<(&File, VerifiedStatus)> {
        &self.results
    }

    pub fn counts(&self) -> (usize, usize, usize) {
        (self.verified, self.notname, self.unverified)
    }

    pub fn len(&self) -> usize {
        self.results.len()
    }
}

#[cfg(test)]
mod tests {
    use crate::lib::{dat::Datafile, files::File, verify::VerifiedStatus};

    use super::CheckedSet;

    #[test]
    fn can_test_roms() {
        let roms = Datafile::from_file("test/dats/megadrive.dat")
            .unwrap()
            .roms();
        let files = File::read_dir("test/roms/megadrive").unwrap();
        let set = CheckedSet::new(&roms, &files);

        // Check number of Files
        assert_eq!(set.results().len(), 3);

        let exact_match = set
            .results()
            .iter()
            .find(|item| {
                item.0.path().file_name().unwrap().to_str().unwrap()
                    == "30 Years Of Nintendon't.bin"
            })
            .unwrap();

        let partial_match = set
            .results()
            .iter()
            .find(|item| {
                item.0.path().file_name().unwrap().to_str().unwrap() == "30yearsofnintendont.bin"
            })
            .unwrap();

        let no_match = set
            .results()
            .iter()
            .find(|item| item.0.path().file_name().unwrap().to_str().unwrap() == "failed.bin")
            .unwrap();

        assert_eq!(
            exact_match.1,
            VerifiedStatus::Verified {
                name: &"30 Years Of Nintendon't.bin".to_string()
            }
        );
        assert_eq!(
            partial_match.1,
            VerifiedStatus::MatchNotName {
                name: &"30 Years Of Nintendon't.bin".to_string()
            }
        );
        assert_eq!(no_match.1, VerifiedStatus::Unverified);
    }
}
