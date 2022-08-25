use crate::lib::input::File;
pub mod hash;

#[derive(Debug, PartialEq)]
pub enum VerifiedStatus<'a> {
    Verified {
        file: &'a File,
    },
    MatchNotName {
        file: &'a File,
        correct_name: &'a str,
    },
    Missing,
}
