use serde_xml_rs::{from_str, Error};

use self::{dat::Datafile, files::read_file};

pub mod dat;
pub mod files;

pub fn parse_dat(src: &str) -> Result<Datafile, Error> {
    let file = read_file(src)?;
    let dat: Datafile = from_str(&file)?;

    Ok(dat)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
