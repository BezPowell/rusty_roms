use crate::lib::input::File;
pub mod hash;

#[derive(Debug)]
pub enum VerifiedStatus<'a> {
    Verified { file: &'a File, output: &'a str },
    MatchNotName { file: &'a File, output: &'a str },
}

impl<'a> VerifiedStatus<'a> {
    pub fn file(&self) -> &File {
        match self {
            VerifiedStatus::Verified { file, output: _ } => file,
            VerifiedStatus::MatchNotName { file, output: _ } => file,
        }
    }

    pub fn output(&self) -> &str {
        match self {
            VerifiedStatus::Verified { file: _, output } => output,
            VerifiedStatus::MatchNotName { file: _, output } => output,
        }
    }
}
