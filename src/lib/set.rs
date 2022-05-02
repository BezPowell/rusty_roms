use super::verify::VerifiedStatus;

pub struct CheckedSet {
    verified: usize,
    notname: usize,
    unverified: usize,
    results: Vec<VerifiedStatus>,
}

impl CheckedSet {
    pub fn new(set: Vec<VerifiedStatus>) -> CheckedSet {
        let mut verified = 0;
        let mut notname = 0;
        let mut unverified = 0;

        for item in &set {
            match item {
                VerifiedStatus::Verified { file: _ } => verified += 1,
                VerifiedStatus::MatchNotName { file: _, rom: _ } => notname += 1,
                VerifiedStatus::Unverified { file: _ } => unverified += 1,
            }
        }

        CheckedSet {
            verified,
            notname,
            unverified,
            results: set,
        }
    }

    pub fn results(&self) -> &Vec<VerifiedStatus> {
        &self.results
    }

    pub fn counts(&self) -> (usize, usize, usize) {
        (self.verified, self.notname, self.unverified)
    }

    pub fn len(&self) -> usize {
        self.results.len()
    }
}
