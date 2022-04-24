use serde_xml_rs::{from_str, Error};

use self::{dat::Datafile, files::read_file};

pub mod dat;
pub mod files;
pub mod rom;