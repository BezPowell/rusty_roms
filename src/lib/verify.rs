#[derive(Debug, PartialEq)]
pub enum VerifiedStatus<'a> {
    Verified { name: &'a str },
    MatchNotName { name: &'a str },
    Unverified,
}

impl VerifiedStatus<'_> {
    pub fn pretty_print(&self) -> String {
        match self {
            VerifiedStatus::Verified { name: _ } => "Verified successfully.".to_string(),
            VerifiedStatus::MatchNotName { name } => format!("contents verified as {}", name),
            VerifiedStatus::Unverified => "could not be matched.".to_string(),
        }
    }

    pub fn output_path(&self) -> Option<&str> {
        match self {
            VerifiedStatus::Verified { name } => Some(name),
            VerifiedStatus::MatchNotName { name } => Some(name),
            VerifiedStatus::Unverified => None,
        }
    }
}
