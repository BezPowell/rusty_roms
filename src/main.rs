use easy_args::{arg_spec, ArgSpec};
use lib::{files::check_files, parse_dat};
use sha1::digest::typenum::Le;

mod lib;

fn main() {
    let args = ArgSpec::build()
        .string("dat")
        .string("input")
        .string("output")
        .parse()
        .unwrap();

    // Process dat
    let dat = match args.string("dat") {
        Some(src) => parse_dat(src).unwrap(),
        None => panic!("No DAT file specified."),
    };

    // Set input directory
    let input = match args.string("input") {
        Some(src) => src,
        None => panic!("No input directory specified."),
    };

    // Verify ROMs
    let matches = check_files(&input, &dat).unwrap();

    // Just print out matches for now
    for entry in matches {
        if let Some(rom) = entry.rom() {
            println!("File {} matched.", entry.file());
            println!("Matching rom: {}", rom.name());
        } else {
            println!("File {} not matched.", entry.file());
        }
    }
}
