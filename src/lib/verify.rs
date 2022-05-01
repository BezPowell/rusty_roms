use std::path::PathBuf;

#[derive(Debug, PartialEq)]
pub enum VerifiedStatus {
    Verified { file: PathBuf },
    MatchNotName { file: PathBuf, rom: String },
    Unverified { file: PathBuf },
}

impl VerifiedStatus {
    pub fn output_path(&self) -> Option<String> {
        match self {
            VerifiedStatus::Verified { file } => Some(format!("{:?}", file.file_name())),
            VerifiedStatus::MatchNotName { file: _, rom } => Some(rom.clone()),
            VerifiedStatus::Unverified { file: _ } => None,
        }
    }

    pub fn file(&self) -> &PathBuf {
        match self {
            VerifiedStatus::Verified { file } => file,
            VerifiedStatus::MatchNotName { file, rom: _ } => file,
            VerifiedStatus::Unverified { file } => file,
        }
    }
}
