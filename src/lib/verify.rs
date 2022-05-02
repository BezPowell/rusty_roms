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
            VerifiedStatus::Verified { file } => {
                Some((*file.file_name().unwrap().to_string_lossy()).to_string())
            }
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

    pub fn pretty_print(&self) -> String {
        match self {
            VerifiedStatus::Verified { file } => {
                format!("{:?} verified.", file.file_name().unwrap())
            }
            VerifiedStatus::MatchNotName { file, rom } => format!(
                "{:?} verified as {}, but filename does not match.",
                file.file_name().unwrap(),
                rom
            ),
            VerifiedStatus::Unverified { file } => {
                format!("No match found for {:?}.", file.file_name().unwrap())
            }
        }
    }
}
