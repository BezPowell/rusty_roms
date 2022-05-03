#[derive(Debug, PartialEq)]
pub enum VerifiedStatus {
    Verified,
    MatchNotName { name: String },
    Unverified,
}
