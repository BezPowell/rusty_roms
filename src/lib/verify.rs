use std::path::PathBuf;

#[derive(Debug, PartialEq)]
pub enum VerifiedStatus {
    Verified { file: PathBuf },
    MatchNotName { file: PathBuf, rom: String },
    Unverified { file: PathBuf },
}
