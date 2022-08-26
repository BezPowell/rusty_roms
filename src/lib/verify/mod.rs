pub mod hash;

#[derive(Debug, PartialEq)]
pub enum VerifiedStatus<'a> {
    Verified { file: &'a str, output: &'a str },
    MatchNotName { file: &'a str, output: &'a str },
}
